use crate::{config_values, db, deploy, git};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use flate2::read::GzDecoder;
use sqlx::{MySql, Pool};
use std::fs;
use std::path::{Path, PathBuf};
use tar::Archive;
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, PartialEq)]
struct ReleaseOpts {
    env: String,
    repo_name: String,
    repo_url: String,
    repo_branch: String,
    skips: Vec<String>,
    no_fail: bool,
    no_clone: bool,
    no_push: bool,
    no_commit: bool,
    no_tag: bool,
    increment_field: String,
}

#[derive(Debug, Error)]
enum VersionTagError {
    #[error("No tags found")]
    NotFound,

    #[error("Error parsing version tag: {0}")]
    ParseError(String),
}

/// Returns the latest release version from a list of tags.
///
/// # Examples
/// ```
/// let tags = vec![
///   "v1.0.0".to_string(),
///   "v1.0.1".to_string(),
///   "v1.1.0".to_string(),
///   "v1.1.1".to_string(),
/// ];
/// let latest_version = mgmt::latest_release_version(&tags).unwrap();
/// assert_eq!(latest_version, semver::Version::parse("1.1.1").unwrap());
/// ```
fn latest_release_version(tags: &[String]) -> Result<semver::Version, VersionTagError> {
    let tags = tags
        .iter()
        .filter(|tag| tag.starts_with("v"))
        .collect::<Vec<_>>();

    let mut versions: Vec<semver::Version> = Vec::new();

    for tag in tags {
        let tag_str = tag
            .strip_prefix("v")
            .context("invalid tag")
            .map_err(|e| VersionTagError::ParseError(format!("invalid tag: {}", e.to_string())))?;
        let version = semver::Version::parse(tag_str)
            .map_err(|e| VersionTagError::ParseError(e.to_string()))?;
        versions.push(version);
    }

    versions.sort();
    versions
        .last()
        .cloned()
        .ok_or_else(|| VersionTagError::NotFound.into())
}

/// Creates or clones the release directory and creates the builds and services subdirectories.
/// If no-clone is false, the repository will be cloned from the remote repository.
/// Otherwise, the directory will be created but not initialized.
/// Returns the path to the repository directory, the builds directory, and the services directory.
///
/// # Examples
/// ```ignore
/// let opts = ReleaseOpts {
///  env: "dev".to_string(),
///  repo_name: "de-releases".to_string(),
///  repo_url: "https://github.com/cyverse-de/de-releases".to_string(),
/// }
///
/// let (repo_dir, builds_dir, services_dir) = mgmt::setup_release_dir(&opts).unwrap();
/// ```
fn setup_release_dir(opts: &ReleaseOpts) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let repo_name = opts.repo_name.clone();

    let repo_dir = PathBuf::from(repo_name);
    let builds_dir = repo_dir.join("builds");
    let services_dir = repo_dir.join("services");

    if !opts.no_clone {
        // If the repository doesn't exist already, clone it.
        if !Path::exists(&repo_dir) {
            git::clone(
                &opts.repo_url,
                repo_dir
                    .as_path()
                    .to_str()
                    .context("cannot convert repo directory to string")?,
            )?;
        } else {
            let git_dir = repo_dir.join(".git");
            if !Path::exists(&git_dir) {
                anyhow::bail!("{} exists but is not a git repository", repo_dir.display());
            }

            // Otherwise, pull the latest changes.
            git::pull(&repo_dir)?;
        }

        // Make sure the correct branch is checked out (default is 'main') if no clone is false.
        git::checkout(&repo_dir, &opts.repo_branch)?;

        // Make sure the builds directory exists.
        if !Path::exists(&builds_dir) {
            fs::create_dir_all(&builds_dir)?;
        }
    } else {
        if Path::exists(&repo_dir) {
            anyhow::bail!(
                "The releases repository directory {} already exists",
                repo_dir.display()
            );
        }

        // Otherwise make a directory with the name of the repo, but don't initialize it.
        fs::create_dir_all(&repo_dir)?;
        fs::create_dir_all(&builds_dir)?;
        fs::create_dir_all(&services_dir)?;
    }

    Ok((repo_dir, builds_dir, services_dir))
}

/// Returns a list of tuples containing the service and repository for each service in the environment.
///
/// # Examples
/// ```ignore
/// let opts = ReleaseOpts {
///   env: "dev".to_string(),
///   repo_name: "de-releases".to_string(),
///   repo_url: "https://github.com/cyverse-de/de-releases".to_string(),
/// }
///
/// let (repo_dir, builds_dir, services_dir) = mgmt::setup_release_dir(&opts).unwrap();
/// ```
async fn get_service_repos(
    tx: &mut sqlx::Transaction<'_, MySql>,
    opts: &ReleaseOpts,
) -> Result<Vec<(db::Service, db::Repository)>> {
    let services = db::get_services(tx, &opts.env)
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
        let repo = db::get_repo_by_id(tx, repo_id).await?;

        tuples.push((service, repo));
    }

    Ok(tuples)
}

/// Returns the URL for the repository.
///
/// # Examples
/// ```ignore
/// let repo_url = mgmt::get_repo_url(&repo).unwrap();
/// ```
fn get_repo_url(repo: &db::Repository) -> Result<String> {
    let mut repo_url = repo.url.clone().ok_or_else(|| {
        anyhow!(
            "No URL found for repository {}",
            repo.name.as_ref().unwrap_or(&String::new())
        )
    })?;

    if !repo_url.ends_with("/") {
        repo_url.push_str("/");
    }

    Ok(repo_url)
}

/// Returns the path to the service directory.
///
/// # Examples
/// ```ignore
/// let service_dir = mgmt::get_service_dir(&services_dir, &service).unwrap();
/// ```
fn get_service_dir(services_dir: &PathBuf, service: &db::Service) -> Result<PathBuf> {
    let service_name = service
        .name
        .clone()
        .context(format!("No name found for service {:?}", service))?;
    let service_dir = services_dir.join(&service_name);

    Ok(service_dir)
}

/// Downloads the deploy-info.tar.gz file from the latest release of the repository and unpacks it
/// into the service directory. Then moves the build.json file from the service directory into the
/// builds directory.
///
/// # Examples
/// ```ignore
/// mgmt::process_release_tarball(&repo_url, &service_name, &builds_dir, &service_dir).await?;
/// ```
async fn process_release_tarball(
    repo_url: &str,
    service_name: &str,
    builds_dir: &PathBuf,
    service_dir: &PathBuf,
) -> Result<()> {
    let tarball_url = Url::parse(&repo_url)?.join("releases/latest/download/deploy-info.tar.gz")?;
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
    println!("Moving {} to {}", build_json_path.display(), to.display());
    fs::rename(build_json_path, to)?;

    Ok(())
}

/// Returns the new version number for the release.
/// The increment field can be "major", "minor", or "patch".
/// The latest release version is determined by looking at the tags in the repository.
///
/// # Examples
/// ```ignore
/// let new_version = mgmt::get_new_version_number(&repo_dir, &opts)?;
/// ```
fn get_new_version_number(repodir: &PathBuf, opts: &ReleaseOpts) -> Result<semver::Version> {
    let current_tags = git::list_tags(&repodir, "origin")?;
    let latest_version = match latest_release_version(&current_tags) {
        Ok(version) => version,
        Err(e) => match e {
            VersionTagError::NotFound => semver::Version::new(0, 0, 0),
            _ => anyhow::bail!("Error parsing version tag: {}", e),
        },
    };

    let mut new_version = latest_version.clone();
    match opts.increment_field.as_str() {
        "major" => {
            new_version.major += 1;
        }

        "minor" => {
            new_version.minor += 1;
        }

        "patch" => {
            new_version.patch += 1;
        }

        _ => {
            anyhow::bail!("Invalid increment field: {}", opts.increment_field);
        }
    }

    Ok(new_version)
}

/// Creates a release in the releases repository.
/// Clones the releases repository (default is 'de-releases') if no-clone is false.
/// For each repository, grabs the build JSON file from the github release.
/// If no-clone is false, commits the changes to the repo and pushes them.
///
/// # Examples
/// ```ignore
/// mgmt::create_release(&pool, &opts).await?;
/// ```
async fn create_release(pool: &Pool<MySql>, opts: &ReleaseOpts) -> Result<()> {
    let mut tx = pool.begin().await?;

    // Clone the releases repo (default is 'de-releases') if no-clone is false.
    println!("Setting up release directory...");
    let (repo_dir, builds_dir, services_dir) = setup_release_dir(opts)?;
    println!("Done setting up release directory.");

    // Get a list of the services included in the environment, filter out the skipped services:
    println!("\nGetting service repositories...");
    let tuples = get_service_repos(&mut tx, &opts).await?;
    println!("Done getting service repositories.");

    let mut process_failures: Vec<String> = Vec::new();

    println!("");

    ////// For each repository, grab the build JSON file from the github release.
    for (service, repo) in tuples {
        let repo_url = get_repo_url(&repo)?;
        let service_dir = get_service_dir(&services_dir, &service)?;
        let service_name = service
            .name
            .as_ref()
            .ok_or_else(|| anyhow!("No name found for service {:?}", service))?;

        println!("Downloading release tarball for {}", service_name);
        match process_release_tarball(&repo_url, service_name, &builds_dir, &service_dir).await {
            Ok(_) => {
                println!("Processed release tarball for {}\n", service_name);
            }

            Err(e) => {
                let msg = format!(
                    "Failed to process release tarball for {}: {}",
                    service_name, e
                );

                println!("{}", msg);

                process_failures.push(msg);

                continue;
            }
        };
    }

    if !process_failures.is_empty() {
        println!("\nThe following errors occurred while processing the release tarballs:");
        process_failures.iter().for_each(|failure| {
            println!("{}", failure);
        });

        if !opts.no_fail {
            anyhow::bail!("Errors occurred while processing release tarballs.");
        }
    }

    // if no-clone is false, commit the changes to the repo and push them.
    if !opts.no_clone {
        let latest_version = get_new_version_number(&repo_dir, &opts)?;

        println!(
            "\nAdding and committing changes to the repository {} as version {}...",
            repo_dir.display(),
            latest_version
        );

        println!("\nAdding changes in the builds directory...");
        git::add(
            &repo_dir,
            builds_dir
                .file_name()
                .context("unable to create build dir basename")?
                .to_str()
                .context("unable to create build dir string")?,
        )?;
        println!("Done adding changes in the builds directory.");

        println!("\nAdding changes in the services directory...");
        git::add(
            &repo_dir,
            services_dir
                .file_name()
                .context("unable to create repo dir basename")?
                .to_str()
                .context("unable to create repo dir string")?,
        )?;
        println!("Done adding changes in the services directory.");

        if !opts.no_tag {
            println!("\nAdding tag v{}...", latest_version);
            git::tag(&repo_dir, &format!("v{}", latest_version))?;
            println!("Done adding tag v{}.", latest_version);
        }

        if !opts.no_commit {
            println!("\nCommitting changes...");
            git::commit(&repo_dir, "update builds")?;
            println!("Done committing changes.");
        }

        if !opts.no_push {
            println!("\nPushing changes...");
            git::push(&repo_dir, "origin", "main")?;
            println!("Done pushing changes.");

            println!("\nPushing tags...");
            git::push_tags(&repo_dir, "origin")?;
            println!("Done pushing tags.");
        }
    }

    println!("\nUpdating database...");
    tx.commit().await?;
    println!("Done updating database.");

    println!("\nDone creating release.");

    Ok(())
}

pub async fn create(pool: &Pool<MySql>, matches: &ArgMatches) -> Result<()> {
    let env = matches.get_one::<String>("env").ok_or_else(|| {
        anyhow!("No environment provided. Use --env <env> to specify an environment.")
    })?;

    let repo_name = matches.get_one::<String>("repo-name").ok_or_else(|| {
        anyhow!("No repository provided. Use --repo-name <repo_name> to specify a repository.")
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

    let no_fail = matches.get_flag("no-fail");
    let no_clone = matches.get_flag("no-clone");
    let no_push = matches.get_flag("no-push");
    let no_commit = matches.get_flag("no-commit");
    let no_tag = matches.get_flag("no-tag");

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
        no_fail,
        no_clone,
        no_push,
        no_commit,
        no_tag,
        skips,
        increment_field: increment_field.to_string(),
    };

    create_release(&pool, &opts).await?;

    Ok(())
}

pub async fn deploy(pool: &Pool<MySql>, matches: &ArgMatches) -> Result<()> {
    let env = matches.get_one::<String>("env").ok_or_else(|| {
        anyhow!("No environment provided. Use --env <env> to specify an environment.")
    })?;

    let repo_name = matches.get_one::<PathBuf>("repo-name").ok_or_else(|| {
        anyhow!("No repository provided. Use --repo-name <repo_name> to specify a repository.")
    })?;

    let repo_branch = matches.get_one::<String>("branch").ok_or_else(|| {
        anyhow!(
            "No repository branch provided. Use --repo-branch <branch> to specify a repository branch."
        )
    })?;

    let configdir = matches.get_one::<PathBuf>("configs").ok_or_else(|| {
        anyhow!("No config directory provided. Use --configs <dir> to specify a config directory.")
    })?;

    let skips = matches
        .get_many::<String>("skip")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let section_opts = config_values::config::SectionOptions::new_from_db(&pool, &env).await?;

    let opts = deploy::Deployment::new(
        pool.to_owned(),
        repo_name.to_owned(),
        repo_branch.to_owned(),
        env.to_owned(),
        skips,
        configdir.to_owned(),
        section_opts,
    );

    opts.deploy().await?;

    Ok(())
}
