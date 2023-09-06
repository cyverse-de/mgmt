use anyhow::{anyhow, Context, Result};
use clap::{arg, builder::ArgPredicate, ArgAction, Command};
use mgmt::{db, dolt, git, ops};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;

fn cli() -> Command {
    Command::new("mgmt-release")
        .about("Creates and posts a release to a git repository")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .help("The URL of the MySQL database to connect to.")
                .default_value("mysql:://root@127.0.0.1:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(
            Command::new("create").args([
                arg!(-s --"skip" [SKIP] "A service to skip for the release")
                    .required(false)
                    .action(ArgAction::Append)
                    .value_parser(clap::value_parser!(String)),
                arg!(-e --env [ENV] "The environment to release")
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
                arg!(-b --branch [REPO_BRANCH] "The branch of the releases repo to use")
                    .required(false)
                    .default_value("main")
                    .value_parser(clap::value_parser!(String)),
                arg!(-n --"no-clone" "Do not clone the repository")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(-v --"version" [VERSION] "The version to release")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
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
    release_version: String,
    no_clone: bool,
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
        let repo_url = repo.url.clone().ok_or_else(|| {
            anyhow!(
                "No URL found for repository {}",
                repo.name.as_ref().unwrap_or(&String::new())
            )
        })?;

        let service_name = service
            .name
            .clone()
            .context(format!("No name found for service {:?}", service))?;

        let build_dl_url = Url::parse(&repo_url)?.join("releases/latest/download/build.json")?;
        let build_dl_resp = reqwest::get(build_dl_url).await?;
        let build_dl_status = build_dl_resp.status();
        if !build_dl_status.is_success() {
            anyhow::bail!(
                "Failed to download build.json for service {}: {}",
                service_name,
                build_dl_status
            );
        }
        let build_json = build_dl_resp.text().await?;

        //////// store it in the <repo-dir>/builds/ directory
        fs::write(
            builds_dir.join(format!("{}.json", service_name)),
            build_json.as_bytes(),
        )?;

        ////// Get the skaffold file for the service from the same github release.
        let skaffold_dl_url =
            Url::parse(&repo_url)?.join("releases/latest/download/skaffold.yaml")?;
        let skaffold_dl_resp = reqwest::get(skaffold_dl_url).await?;
        let skaffold_dl_status = skaffold_dl_resp.status();
        if !skaffold_dl_status.is_success() {
            anyhow::bail!(
                "Failed to download skaffold.yaml for service {}: {}",
                service_name,
                skaffold_dl_status
            );
        }
        let skaffold_yaml = skaffold_dl_resp.text().await?;

        //////// store it in <repo-dir>/services/<service>/
        fs::write(
            services_dir.join(service_name).join("skaffold.yaml"),
            skaffold_yaml.as_bytes(),
        )?;
    }

    ////// Get the referenced files from the skaffold file.
    //////// store them in <repo-dir>/services/<service>/, alongside the skaffold.yaml file.
    // if no-clone is false, commit the changes to the repo and push them.
    // if no clone is false, create a new release based on the last commit (only if not terribly difficult, otherwise use github actions for that)
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

            let repo_name = matches.get_one::<String>("repo_name").ok_or_else(|| {
                anyhow!(
                    "No repository provided. Use --repo-name <repo_name> to specify a repository."
                )
            })?;

            let repo_url = matches.get_one::<String>("repo_url").ok_or_else(|| {
                anyhow!(
                    "No repository URL provided. Use --repo-url <repo_url> to specify a repository URL."
                )
            })?;

            let repo_branch = matches.get_one::<String>("repo_branch").ok_or_else(|| {
                anyhow!(
                    "No repository branch provided. Use --repo-branch <branch> to specify a repository branch."
                )
            })?;

            let release_version = matches.get_one::<String>("version").ok_or_else(|| {
                anyhow!("No release version provided. Use --version <version> to specify a release version.")
            })?;

            let no_clone = matches.get_flag("no-clone");

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
                release_version: release_version.to_string(),
                no_clone,
                skips,
            };

            create_release(&pool, &opts).await?;
        }

        Some(("preview", matches)) => {
            let env = matches.get_one::<String>("env").ok_or_else(|| {
                anyhow!("No environment provided. Use --env <env> to specify an environment.")
            })?;

            let repo_name = matches.get_one::<String>("repo_name").ok_or_else(|| {
                anyhow!("No repository provided. Use --repo <repo> to specify a repository.")
            })?;

            let repo_url = matches.get_one::<String>("repo_url").ok_or_else(|| {
                anyhow!("No repository URL provided. Use --repo-url <repo_url> to specify a repository URL.")
            })?;

            let repo_branch = matches.get_one::<String>("repo_branch").ok_or_else(|| {
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
                release_version: String::new(),
                no_clone: false,
                skips,
            };

            preview_release(&pool, &opts).await?;
        }

        _ => {
            println!("No subcommand was used");
        }
    }

    Ok(())
}
