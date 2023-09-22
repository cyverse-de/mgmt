// mgmt-site sets up a directory containing repos and configuration values for a DE deployment site.
//
// A site consists of one or more DE deployments.

use anyhow::Context;
use mgmt::cli::site;
use mgmt::config_values::config;
use mgmt::db;
use mgmt::dolt;
use mgmt::git;
use mgmt::ops;
use std::path::{Path, PathBuf};

use sqlx::mysql::MySqlPoolOptions;

/**
 * Set up the CLI for the mgmt-site binary.
 */

#[derive(Debug, Clone, PartialEq)]
struct InitOpts {
    dir: String,
    db_repo: String,
    db_name: String,
    force: bool,
    no_db_clone: bool,
    no_repo_clone: bool,
    no_env: bool,
    no_defaults: bool,
    no_values: bool,
    defaults_filename: String,
    values_filename: String,
}

// Create the site directory if it doesn't already exist.
// If it does exist, and force is true, delete it and recreate it.
fn create_site_dir(opts: &InitOpts) -> anyhow::Result<()> {
    let dir = &opts.dir;
    let force = opts.force;
    let site_exists = std::path::Path::new(dir).exists();
    if site_exists && force {
        std::fs::remove_dir_all(dir)?;
    } else if site_exists {
        return Err(anyhow::anyhow!(
            "Directory {} already exists. Use -f or --force to overwrite.",
            dir
        ));
    } else {
        let repo_dir = Path::new(dir).join("repos");
        std::fs::create_dir_all(repo_dir)?;
    }
    Ok(())
}

// Create the dolt database directory inside of the site directory.
// If force is true, delete the directory and recreate it.
fn create_db_dir(opts: &InitOpts) -> anyhow::Result<PathBuf> {
    let dir = &opts.dir;
    let db_name = &opts.db_name;
    let force = opts.force;
    let db_dir = Path::new(dir).join(db_name);
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
fn clone_db(opts: &InitOpts) -> anyhow::Result<PathBuf> {
    let db_repo = &opts.db_repo;
    let db_dir = create_db_dir(&opts)?;
    let db_dir_str = db_dir
        .to_str()
        .context("could not get name of the database directory")?;
    dolt::clone(db_repo, db_dir_str)?;
    Ok(db_dir)
}

async fn init(opts: &InitOpts) -> anyhow::Result<()> {
    // Create the site directory.
    create_site_dir(&opts)?;

    let db_dir: PathBuf;

    println!("Cloning the database from {}...", &opts.db_repo);
    // Clone the base database.
    if !opts.no_db_clone {
        db_dir = clone_db(&opts)?;
    } else {
        db_dir = PathBuf::from(&opts.dir).join(&opts.db_name);
    }
    println!("Done cloning the database.\n");

    println!("Starting the database...");
    // Start the database
    let db_dir_str = db_dir
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("failed to get database directory as string"))?;
    let db_handle = dolt::start(db_dir_str)?;
    println!("Done staring the database.\n");

    println!("Connecting to the database...");
    // Connect to the database.
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!("mysql://root@127.0.0.1:3306/{}", &opts.db_name))
        .await?;
    let mut tx = pool.begin().await?;
    println!("Done connecting to the database.\n");

    // Get the list of repos.
    let repos = db::get_repos(&mut tx).await?;

    println!("Cloning the repos...");
    // Clone each of the repos.
    for repo in repos {
        let (repo_url, repo_name) = repo;
        let repo_dir = Path::new(&opts.dir).join("repos").join(&repo_name);
        let repo_dir_str = repo_dir
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("failed to get repo directory as string"))
            .unwrap();

        println!("Cloning {} into {}", repo_url, repo_dir_str);
        if !opts.no_repo_clone {
            git::clone(&repo_url, repo_dir_str)?;
        } else {
            println!("Skipping cloning of {}", repo_url);
        }
        println!("");
    }
    println!("Done cloning the repos.\n");

    let mut env_config = config::ConfigValues::default();

    if !opts.no_env {
        println!("Setting up the environment...");
        env_config.ask_for_info(&mut tx).await?;
        println!("Done setting up the environment.\n");
    }

    // Write out the default config values into the site directory.
    if !opts.no_defaults {
        println!("Writing out the default values...");
        let defaults_filename = Path::new(&opts.dir).join(&opts.defaults_filename);
        ops::render_default_values(&pool, Some(defaults_filename)).await?;
        println!("Done writing out the default values.\n");
    }

    tx.commit().await?;

    if !opts.no_env && !opts.no_values {
        println!("Writing out the environment config values...");
        println!("env: {:?}", env_config.environment);
        let values_filename = Path::new(&opts.dir).join(&opts.values_filename);
        let mut section_option = config::SectionOptions::default();
        section_option.set_all(true)?;
        ops::render_values(
            &pool,
            &env_config.environment,
            &section_option,
            Some(values_filename),
        )
        .await?;
        println!("Done writing out the environment config values.\n");
    }

    // Clean up and shut down
    println!("Shutting down the database...");
    pool.close().await;
    db_handle.kill()?;
    println!("Done shutting down the database.\n");

    Ok(())
}

struct DeployOpts {
    site_dirpath: PathBuf,
    env: String,
    db_name: String,
    services: Vec<String>,
    defaults_filepath: PathBuf,
    values_filepath: PathBuf,
    builds_dirpath: PathBuf,
}

async fn deploy(opts: &DeployOpts) -> anyhow::Result<()> {
    println!("Deploying {} from {:?}...", opts.env, opts.site_dirpath);
    println!("Using database {}...", opts.db_name);
    println!("Using defaults file {:?}...", opts.defaults_filepath);
    println!("Using values file {:?}...\n", opts.values_filepath);

    print!("Starting the database...");
    let db_dir = Path::new(&opts.site_dirpath).join(&opts.db_name);
    let db_dir_str = db_dir
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("failed to get database directory as string"))?;
    let dolt_handle = dolt::start(db_dir_str)?;
    println!("DONE\n");

    print!("Connecting to the database...");
    // Connect to the database.
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!("mysql://root@127.0.0.1:3306/{}", &opts.db_name))
        .await?;
    let mut tx = pool.begin().await?;
    println!("DONE\n");

    let services_to_deploy: Vec<String>;

    if opts.services.is_empty() {
        services_to_deploy = db::list_services(&mut tx, &opts.env)
            .await?
            .into_iter()
            .filter_map(|svc| svc.name)
            .collect();
    } else {
        services_to_deploy = opts.services.clone();
    }

    // Create the configs directory for the environment in the site directory if it doesn't already exist.
    // If it already exists, use it. opts.site_dirpath / configs / opts.env is the format for the config dir.

    // Generate the configs for the service in the configs directory. See configs::generate for the command.

    // Load the configs into the cluster.

    // Load secrets into the cluster for the service.

    for service in services_to_deploy {
        println!("Deploying service {}...", service);

        // Find builds file for the service. Need to make sure the de-releases repo is pulled.
        let builds_pb = Path::new(&opts.builds_dirpath).join(&service);
        let mut builds_path = builds_pb.to_str().unwrap_or_default().to_string();
        builds_path.push_str(".json");
        println!("Build metadata file: {}", builds_path);

        // Deploy the service. See deploy_project in app.rs for a reference.
    }

    tx.commit().await?;
    dolt_handle.kill()?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = site::cli().get_matches();

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

            let no_db_clone = matches.get_flag("no-db-clone");
            let no_repo_clone = matches.get_flag("no-repo-clone");
            let force = matches.get_flag("force");
            let no_env = matches.get_flag("no-env");
            let no_defaults = matches.get_flag("no-defaults");
            let no_values = matches.get_flag("no-values");
            let defaults_filename = matches.get_one::<String>("defaults-filename").ok_or_else(|| {
                anyhow::anyhow!("No defaults filename specified. Use --defaults-filename to specify a defaults filename.")
            })?;
            let values_filename = matches.get_one::<String>("values-filename").ok_or_else(|| {
                anyhow::anyhow!("No values filename specified. Use --values-filename to specify a values filename.")
            })?;

            let opts = InitOpts {
                dir: dir.clone(),
                db_repo: db_repo.clone(),
                db_name: db_name.clone(),
                force,
                no_db_clone,
                no_repo_clone,
                no_env,
                no_defaults,
                no_values,
                defaults_filename: defaults_filename.clone(),
                values_filename: values_filename.clone(),
            };
            init(&opts).await?;
            println!("Site initialized in {}", dir);
        }
        Some(("deploy", matches)) => {
            let dir = matches.get_one::<PathBuf>("dir").ok_or_else(|| {
                anyhow::anyhow!("No directory specified. Use -d or --dir to specify a directory.")
            })?;

            let env = matches.get_one::<String>("env").ok_or_else(|| {
                anyhow::anyhow!(
                    "No environment specified. Use -e or --env to specify an environment."
                )
            })?;

            let db_name = matches.get_one::<String>("db-name").ok_or_else(|| {
                anyhow::anyhow!(
                    "No Dolt DB name specified. Use -n or --db-name to specify a Dolt DB name."
                )
            })?;

            let services = matches
                .get_many::<String>("service")
                .unwrap_or_default()
                .map(|v| v.to_string())
                .collect::<Vec<_>>();

            let defaults_filename = matches.get_one::<PathBuf>("defaults-filename").ok_or_else(|| {
                anyhow::anyhow!("No defaults filename specified. Use --defaults-filename to specify a defaults filename.")
            })?;

            let values_filename = matches.get_one::<PathBuf>("values-filename").ok_or_else(|| {
                anyhow::anyhow!("No values filename specified. Use --values-filename to specify a values filename.")
            })?;

            let dir_canon = dir.canonicalize()?;
            let opts = DeployOpts {
                site_dirpath: dir_canon.clone(),
                env: env.clone(),
                db_name: db_name.clone(),
                services,
                defaults_filepath: Path::new(&dir_canon).join(defaults_filename),
                values_filepath: Path::new(&dir_canon).join(values_filename),
                builds_dirpath: Path::new(&dir_canon).join("builds"),
            };

            deploy(&opts).await?;
        }
        _ => unreachable!(),
    }
    Ok(())
}
