use anyhow::{anyhow, Context, Result};
use clap::{arg, ArgAction, Command};
use flate2::read::GzDecoder;
use mgmt::{db, git};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::fs;
use std::path::PathBuf;
use tar::Archive;
use url::Url;

fn cli() -> Command {
    Command::new("mgmt-release")
        .about("Creates and posts a release to a git repository")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .help("The URL of the MySQL database to connect to.")
                .default_value("mysql://root@127.0.0.1:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(
            Command::new("create").args([
                arg!(-s --"skip" <SKIP> "A service to skip for the release")
                    .required(false)
                    .action(ArgAction::Append)
                    .value_parser(clap::value_parser!(String)),
                arg!(-e --env <ENV> "The environment to release")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
                arg!(-r --"repo-name" [REPO_NAME] "The repository to release to")
                    .required(false)
                    .default_value("de-releases")
                    .value_parser(clap::value_parser!(String)),
                arg!(-u --"repo-url" [REPO_URL] "The releases Git repository URL")
                    .required(false)
                    .default_value("github.com/cyverse-de/de-releases")
                    .value_parser(clap::value_parser!(String)),
                arg!(-b --branch [BRANCH] "The branch of the releases repo to use")
                    .required(false)
                    .default_value("main")
                    .value_parser(clap::value_parser!(String)),
                arg!(-n --"no-clone" "Do not clone the repository")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(-i --"increment-field" [INCREMENT_FIELD] "The field to increment for the release")
                    .required(false)
                    .default_value("patch")
                    .value_parser(clap::builder::PossibleValuesParser::new(["major", "minor", "patch"])),
            ]),
        )
        .subcommand(
            Command::new("preview")
                .about("Generates a preview of the release")
                .args([
                    arg!(-s --"skip" [SKIP] "A service to skip for the release")
                        .required(false)
                        .action(ArgAction::Append)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-e --env [ENV] "The environment to release")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-r --"repo" [REPO] "The repository to release to"),
                    arg!(-b --"builds" [BUILDS]
                        "Directory containing build.json files for each service. Defaults to the 'builds' subdirectory under the local directory."
                    )
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf))
                ]),
        )
}

#[derive(Debug, Clone, PartialEq)]
struct ReleaseOpts {
    env: String,
    repo_name: String,
    repo_url: String,
    repo_branch: String,
    skips: Vec<String>,
    no_clone: bool,
    increment_field: String,
}

fn latest_release_version(tags: &[String]) -> Result<semver::Version> {
    let tags = tags
        .iter()
        .filter(|tag| tag.starts_with("release-v"))
        .collect::<Vec<_>>();

    let mut versions: Vec<semver::Version> = Vec::new();

    for tag in tags {
        let version = semver::Version::parse(&tag[8..])?;
        versions.push(version);
    }

    versions.sort();
    versions
        .last()
        .cloned()
        .ok_or_else(|| anyhow!("No tags found"))
}

async fn create_release(pool: &Pool<MySql>, opts: &ReleaseOpts) -> Result<()> {
    let mut tx = pool.begin().await?;

    let repo_name = opts.repo_name.clone();

    // Clone the releases repo (default is 'de-releases') if no-clone is false.
    let repo_dir = PathBuf::from(repo_name).canonicalize().context(format!(
        "Failed to canonicalize the releases repo directory: {}",
        opts.repo_name
    ))?;

    let builds_dir = repo_dir.join("builds");
    let services_dir = repo_dir.join("services");

    if !opts.no_clone {
        git::clone(
            &opts.repo_url,
            repo_dir
                .as_path()
                .to_str()
                .context("cannot convert repo directory to string")?,
        )?;
    } else {
        // Otherwise make a directory with the name of the repo, but don't initialize it.
        fs::create_dir_all(&repo_dir)?;
        fs::create_dir_all(&builds_dir)?;
        fs::create_dir_all(&services_dir)?;
    }

    // Make sure the correct branch is checked out (default is 'main') if no clone is false.
    if !opts.no_clone {
        git::checkout(&repo_dir, &opts.repo_branch)?;
    }

    // Get a list of the services included in the environment, filter out the skipped services:
    let services = db::get_services(&mut tx, &opts.env)
        .await?
        .into_iter()
        .filter(|s| {
            !opts.skips.iter().any(|skipped_service| {
                s.name
                    .as_ref()
                    .map(|name| name == skipped_service)
                    .unwrap_or(false)
            })
        })
        .collect::<Vec<_>>();

    //// For each service, get the repository from the database.
    let mut tuples: Vec<(db::Service, db::Repository)> = Vec::new();
    for service in services {
        let repo_id = service
            .repo_id
            .clone()
            .ok_or_else(|| anyhow!("No repository ID found for service"))?;
        let repo = db::get_repo_by_id(&mut tx, repo_id).await?;

        tuples.push((service, repo));
    }

    ////// For each repository, grab the build JSON file from the github release.
    for (service, repo) in tuples {
        let mut repo_url = repo.url.clone().ok_or_else(|| {
            anyhow!(
                "No URL found for repository {}",
                repo.name.as_ref().unwrap_or(&String::new())
            )
        })?;

        if !repo_url.ends_with("/") {
            repo_url.push_str("/");
        }

        let service_name = service
            .name
            .clone()
            .context(format!("No name found for service {:?}", service))?;
        let service_dir = services_dir.join(&service_name);

        let tarball_url =
            Url::parse(&repo_url)?.join("releases/latest/download/deploy-info.tar.gz")?;
        let tarball_url_str = tarball_url.as_str();
        let tarball_resp = reqwest::get(tarball_url.clone()).await?;
        let tarball_status = tarball_resp.status();
        if !tarball_status.is_success() {
            anyhow::bail!(
                "Failed to download deploy-info.tar.gz from {}: {}",
                tarball_url_str,
                tarball_status
            );
        }
        let tarball = tarball_resp.bytes().await?;
        let tar = GzDecoder::new(tarball.as_ref());
        let mut archive = Archive::new(tar);
        archive.unpack(&service_dir)?;

        // move the build.json file from the service_dir into the build directory.
        // with a name like <service_name>.json.
        let build_json_path = service_dir.join("build.json");
        let to = builds_dir.join(format!("{}.json", service_name));
        fs::rename(build_json_path, to)?;
    }

    // if no-clone is false, commit the changes to the repo and push them.
    if !opts.no_clone {
        let current_tags = git::list_tags(&repo_dir, "origin")?;
        let mut latest_version = latest_release_version(&current_tags)?;
        match opts.increment_field.as_str() {
            "major" => {
                latest_version.major += 1;
            }

            "minor" => {
                latest_version.minor += 1;
            }

            "patch" => {
                latest_version.patch += 1;
            }

            _ => {
                anyhow::bail!("Invalid increment field: {}", opts.increment_field);
            }
        }

        git::add(
            &repo_dir,
            builds_dir
                .to_str()
                .context("unable to create build dir string")?,
        )?;

        git::add(
            &repo_dir,
            services_dir
                .to_str()
                .context("unable to create repo dir string")?,
        )?;

        git::tag(&repo_dir, &format!("release-v{}", latest_version))?;
        git::commit(&repo_dir, "update builds")?;
        git::push(&repo_dir, "origin", "main")?;
        git::push_tags(&repo_dir, "origin")?;
    }

    Ok(())
}

async fn preview_release(pool: &Pool<MySql>, opts: &ReleaseOpts) -> Result<()> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();

    let database_url = matches
        .get_one::<String>("database-url")
        .unwrap_or_else(|| {
            panic!("No database URL specified. Use --database-url <url> to specify a database URL.")
        });

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    match matches.subcommand() {
        Some(("create", matches)) => {
            let env = matches.get_one::<String>("env").ok_or_else(|| {
                anyhow!("No environment provided. Use --env <env> to specify an environment.")
            })?;

            let repo_name = matches.get_one::<String>("repo-name").ok_or_else(|| {
                anyhow!(
                    "No repository provided. Use --repo-name <repo_name> to specify a repository."
                )
            })?;

            let repo_url = matches.get_one::<String>("repo-url").ok_or_else(|| {
                anyhow!(
                    "No repository URL provided. Use --repo-url <repo_url> to specify a repository URL."
                )
            })?;

            let repo_branch = matches.get_one::<String>("branch").ok_or_else(|| {
                anyhow!(
                    "No repository branch provided. Use --repo-branch <branch> to specify a repository branch."
                )
            })?;

            let no_clone = matches.get_flag("no-clone");

            let increment_field = matches.get_one::<String>("increment-field").ok_or_else(|| {
                anyhow!("No increment field provided. Use --increment-field <field> to specify an increment field.")
            })?;

            let skips = matches
                .get_many::<String>("skip")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            let opts = ReleaseOpts {
                env: env.to_string(),
                repo_name: repo_name.to_string(),
                repo_url: repo_url.to_string(),
                repo_branch: repo_branch.to_string(),
                no_clone,
                skips,
                increment_field: increment_field.to_string(),
            };

            create_release(&pool, &opts).await?;
        }

        Some(("preview", matches)) => {
            let env = matches.get_one::<String>("env").ok_or_else(|| {
                anyhow!("No environment provided. Use --env <env> to specify an environment.")
            })?;

            let repo_name = matches.get_one::<String>("repo-name").ok_or_else(|| {
                anyhow!("No repository provided. Use --repo <repo> to specify a repository.")
            })?;

            let repo_url = matches.get_one::<String>("repo-url").ok_or_else(|| {
                anyhow!("No repository URL provided. Use --repo-url <repo_url> to specify a repository URL.")
            })?;

            let repo_branch = matches.get_one::<String>("rbranch").ok_or_else(|| {
                anyhow!("No repository branch provided. Use --repo-branch <branch> to specify a repository branch.")
            })?;

            let skips = matches
                .get_many::<String>("skip")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            let opts = ReleaseOpts {
                env: env.to_string(),
                repo_name: repo_name.to_string(),
                repo_url: repo_url.to_string(),
                repo_branch: repo_branch.to_string(),
                no_clone: false,
                skips,
                increment_field: String::new(),
            };

            preview_release(&pool, &opts).await?;
        }

        _ => {
            println!("No subcommand was used");
        }
    }

    Ok(())
}
