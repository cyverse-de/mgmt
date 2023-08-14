// mgmt-site sets up a directory containing repos and configuration values for a DE deployment site.
//
// A site consists of one or more DE deployments.

use anyhow::Context;
use clap::{arg, ArgAction, Command};
use mgmt::db;
use mgmt::dolt;
use mgmt::git;
use std::path::{Path, PathBuf};

use sqlx::mysql::MySqlPoolOptions;

/**
 * Set up the CLI for the mgmt-site binary.
 */
fn cli() -> Command {
    Command::new("mgmt-site")
        .about(
            "Sets up directory containing repos and configuration values for a DE deployment site.",
        )
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("init").args([
                arg!(-d --dir [DIR] "Directory to initialize")
                    .help("The directory containing the site information. Defaults to the currect directory.")
                    .default_value(".")
                    .value_parser(clap::value_parser!(String)),
                arg!(-r --"db-repo" [DB_REPO] "The Dolt DB repo to set up and use for initializing the local DB.")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
                arg!(-n --"db-name" [DB_NAME] "The name of the DB")
                    .default_value("de_releases")
                    .value_parser(clap::value_parser!(String)),
                arg!(-f --force "Overwrite existing files")
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool))
            ]),
        )
}

// Create the site directory if it doesn't already exist.
// If it does exist, and force is true, delete it and recreate it.
fn create_site_dir(dir: &str, force: bool) -> anyhow::Result<()> {
    let site_exists = std::path::Path::new(dir).exists();
    if site_exists && force {
        std::fs::remove_dir_all(dir)?;
    } else if site_exists {
        return Err(anyhow::anyhow!(
            "Directory {} already exists. Use -f or --force to overwrite.",
            dir
        ));
    } else {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

// Create the dolt database directory inside of the site directory.
// If force is true, delete the directory and recreate it.
fn create_db_dir(site_dir: &str, db_name: &str, force: bool) -> anyhow::Result<PathBuf> {
    let db_dir = Path::new(site_dir).join(db_name);
    if db_dir.exists() && force {
        std::fs::remove_dir_all(&db_dir)?;
    } else if db_dir.exists() {
        return Err(anyhow::anyhow!(
            "Directory {} already exists. Use -f or --force to overwrite.",
            db_dir.to_str().unwrap()
        ));
    }
    std::fs::create_dir_all(&db_dir)?;
    Ok(db_dir)
}

// Use the dolt command to clone the initial database state from the remote.
fn clone_db(site_dir: &str, db_repo: &str, db_name: &str, force: bool) -> anyhow::Result<PathBuf> {
    let db_dir = create_db_dir(site_dir, db_name, force)?;
    let db_dir_str = db_dir
        .to_str()
        .context("could not get name of the database directory")?;
    dolt::clone(db_repo, db_dir_str)?;
    Ok(db_dir)
}

async fn init(dir: &str, db_repo: &str, db_name: &str, force: bool) -> anyhow::Result<()> {
    // Create the site directory.
    create_site_dir(dir, force)?;

    // Clone the base database and start it up.
    let db_dir = clone_db(dir, db_repo, db_name, force)?;
    let db_dir_str = db_dir
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("failed to get database directory as string"))?;
    let db_handle = dolt::start(db_dir_str)?;

    // Connect to the database.
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!("mysql://root@127.0.0.1:3306/{}", db_name))
        .await?;
    let mut tx = pool.begin().await?;

    // Get the list of repos.
    let repos = db::get_repos(&mut tx).await?;

    // Clone each of the repos.
    for repo in repos {
        let (repo_url, repo_name) = repo;
        let repo_dir = Path::new(dir).join(&repo_name);
        let repo_dir_str = repo_dir
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("failed to get repo directory as string"))
            .unwrap();
        git::clone(&repo_url, repo_dir_str)?;
    }

    // Clean up and shut down
    tx.commit().await?;
    pool.close().await;
    db_handle.kill()?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", matches)) => {
            let dir = matches.get_one::<String>("dir").ok_or_else(|| {
                anyhow::anyhow!("No directory specified. Use -d or --dir to specify a directory.")
            })?;
            let db_repo = matches.get_one::<String>("db-repo").ok_or_else(|| {
                anyhow::anyhow!("No Dolt DB remote specified. Use -r or --db-remote to specify a Dolt DB remote.")
            })?;
            let db_name = matches.get_one::<String>("db-name").ok_or_else(|| {
                anyhow::anyhow!(
                    "No Dolt DB name specified. Use -n or --db-name to specify a Dolt DB name."
                )
            })?;
            let force = matches.get_flag("force");
            init(dir, db_repo, db_name, force).await?;
        }
        _ => unreachable!(),
    }
    Ok(())
}
