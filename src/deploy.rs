//! Discovery Environment Deployment Functionality
//!
//! This module contains the functions that can be reused across the mgmt
//! commands to deploy the Discovery Environment.
use anyhow::{Context, Result};
use sqlx::{Pool, Postgres, Transaction};
use std::path::PathBuf;
use std::process::Command;

use crate::handlers::templates;
use crate::{configs, db, ops};

#[derive(Debug, Clone)]
pub struct DeploymentOptions {
    // The database connection pool.
    pub pool: Pool<Postgres>,

    // The path to the directory containing the de-releases repository.
    pub repodir: PathBuf,

    // The URL to the repository to deploy from.
    pub repo_url: String,

    // The branch of the de-releases repository to deploy from.
    pub branch: String,

    // The name of the environment to deploy.
    pub env: String,

    // The name of the services to NOT deploy.
    pub skips: Vec<String>,

    // The base directory for the configurations
    pub configdir: PathBuf,

    // Whether to deploy the services.
    pub no_deploy: bool,

    // Whether to load the configs.
    pub no_load_configs: bool,

    // Whether to load the secrets.
    pub no_load_secrets: bool,

    // Whether to render the configs.
    pub no_render_configs: bool,

    // List of services to deploy before the rest.
    pub pre_deploy: Vec<String>,
}

async fn get_services(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
    skips: &[String],
) -> Result<Vec<db::Service>> {
    let mut services = db::get_services(tx, &env).await?;
    services.retain(|service| !skips.contains(&service.name));
    Ok(services)
}

async fn get_namespace(tx: &mut Transaction<'_, Postgres>, env: &str) -> Result<String> {
    Ok(db::get_namespace(tx, &env).await?)
}

pub fn deploy_service(releases_dir: &PathBuf, ns: &str, svc: &db::Service) -> Result<bool> {
    let svc_json = releases_dir
        .join("builds")
        .join(format!("{}.json", svc.name));

    Ok(Command::new("skaffold")
        .args([
            "deploy",
            "--namespace",
            ns,
            "--build-artifacts",
            svc_json
                .to_str()
                .context("couldn't get service json path")?,
            "--force",
        ])
        .status()?
        .success())
}

pub async fn deploy(
    pool: &Pool<Postgres>,
    env: &str,
    release_repo_dir: &PathBuf,
    release_repo_url: &str,
    release_repo_branch: &str,
    opts: &DeploymentOptions,
) -> Result<bool> {
    let repo_name: String = release_repo_dir
        .to_str()
        .context("couldn't get repo name")?
        .into();

    let mut tx = pool.begin().await?;

    let ro = ops::ReleaseOpts {
        env: env.to_string(),
        repo_name,
        repo_url: release_repo_url.to_string(),
        repo_branch: release_repo_branch.to_string(),
        no_tag: true,
        increment_field: "patch".to_string(),
        no_clone: false,
        no_push: true,
        no_commit: true,
        no_fail: true,
        skips: opts.skips.clone(),
    };
    ops::setup_release_dir(&ro)?;

    let namespace = get_namespace(&mut tx, &env).await?;
    println!("namespace: {}", namespace);

    // Get all of the services in the environments.
    let all_services = get_services(&mut tx, &env, &opts.skips).await?;

    // From the list of all services, get those that should be deployed
    // first.
    let pre_deploy_services = all_services
        .iter()
        .filter(|svc| opts.pre_deploy.contains(&svc.name))
        .collect::<Vec<&db::Service>>();

    println!(
        "services to deploy first:{}",
        pre_deploy_services.iter().fold(String::new(), |acc, svc| {
            format!("{}\n\t{}", acc, svc.name)
        })
    );

    // Get the rest of the services, excluding those that should be deployed
    // first.
    let services = all_services
        .iter()
        .filter(|svc| !opts.pre_deploy.contains(&svc.name))
        .collect::<Vec<&db::Service>>();

    println!(
        "services:{}",
        services.iter().fold(String::new(), |acc, svc| {
            format!("{}\n\t{}", acc, svc.name)
        })
    );

    //let templatesdir = self.repodir.join("templates");
    println!("templates dir: {}", release_repo_dir.display());

    // Create the configuration file directory for the environment.
    if !opts.configdir.exists() {
        std::fs::create_dir(&opts.configdir)?;
    }

    let env_configdir = opts.configdir.join(&env);
    if !env_configdir.exists() {
        std::fs::create_dir(&env_configdir)?;
    }

    // Generate the configuration files
    let secrets_dir = env_configdir.join("secrets");

    if !opts.no_render_configs {
        // Set up the secrets directory
        if !secrets_dir.exists() {
            std::fs::create_dir(&secrets_dir)?;
        }

        // Render the secrets templates
        templates::render_template_dir_from_db(
            &mut tx,
            &release_repo_dir.join("templates").join("secrets").join("*"),
            &env,
            &secrets_dir,
        )
        .await?;

        // Render the configuration templates
        templates::render_db(&mut tx, &env, &release_repo_dir, &env_configdir).await?;
    }

    // Load the configs.
    if !opts.no_load_configs {
        configs::load_configs(&namespace, "service-configs", &env_configdir)?;
    }

    // Load the secrets.
    if !opts.no_load_secrets {
        configs::load_secrets(&namespace, &secrets_dir)?;
    }

    // Deploy the services.
    if !opts.no_deploy {
        pre_deploy_services.iter().for_each(|svc| {
            deploy_service(&release_repo_dir, &namespace, svc).expect("failed to deploy service");
        });

        services.iter().for_each(|svc| {
            deploy_service(&release_repo_dir, &namespace, svc).expect("failed to deploy service");
        });
    }

    tx.commit().await?;

    Ok(true)
}
