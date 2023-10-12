//! Discovery Environment Deployment Functionality
//!
//! This module contains the functions that can be reused across the mgmt
//! commands to deploy the Discovery Environment.
use anyhow::{Context, Result};
use sqlx::{MySql, Pool, Transaction};
use std::path::PathBuf;
use std::process::Command;

use crate::configs;
use crate::db;
use crate::handlers::templates;
use crate::ops;

pub struct Deployment {
    // The database connection pool.
    pool: Pool<MySql>,

    // The path to the directory containing the de-releases repository.
    repodir: PathBuf,

    // The URL to the repository to deploy from.
    repo_url: String,

    // The branch of the de-releases repository to deploy from.
    branch: String,

    // The name of the environment to deploy.
    env: String,

    // The name of the services to NOT deploy.
    skips: Vec<String>,

    // The base directory for the configurations
    configdir: PathBuf,

    // Whether to deploy the services.
    no_deploy: bool,

    // Whether to load the configs.
    no_load_configs: bool,

    // Whether to load the secrets.
    no_load_secrets: bool,

    // Whether to render the configs.
    no_render_configs: bool,

    // List of services to deploy before the rest.
    pre_deploy: Vec<String>,
}

impl Deployment {
    pub fn new(
        pool: Pool<MySql>,
        repodir: PathBuf,
        branch: String,
        env: String,
        skips: Vec<String>,
        configdir: PathBuf,
        no_deploy: bool,
        no_load_configs: bool,
        no_load_secrets: bool,
        no_render_configs: bool,
        pre_deploy: Vec<String>,
        repo_url: String,
    ) -> Self {
        Self {
            pool,
            repodir,
            branch,
            env,
            skips,
            configdir,
            no_deploy,
            no_load_configs,
            no_load_secrets,
            no_render_configs,
            pre_deploy,
            repo_url,
        }
    }

    async fn get_services(&self, tx: &mut Transaction<'_, MySql>) -> Result<Vec<db::Service>> {
        let mut services = db::get_services(tx, &self.env).await?;

        services.retain(|service| {
            !self
                .skips
                .contains(service.name.as_ref().unwrap_or(&"".to_string()))
        });

        Ok(services)
    }

    async fn get_namespace(&self, tx: &mut Transaction<'_, MySql>) -> Result<String> {
        Ok(db::get_namespace(tx, &self.env).await?)
    }

    fn deploy_service(&self, ns: &str, svc: &db::Service) -> Result<bool> {
        let svc_json = self.repodir.join("builds").join(format!(
            "{}.json",
            svc.name.as_ref().context("couldn't get service name")?
        ));
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

    pub async fn deploy(&self) -> Result<bool> {
        let mut tx = self.pool.begin().await?;

        let repo_name: String = self
            .repodir
            .to_str()
            .context("couldn't get repo name")?
            .into();
        let repo_url = self.repo_url.clone();
        let ro = ops::ReleaseOpts {
            env: self.env.clone(),
            repo_name,
            repo_url,
            repo_branch: self.branch.clone(),
            no_tag: true,
            increment_field: "patch".to_string(),
            no_clone: false,
            no_push: true,
            no_commit: true,
            no_fail: true,
            skips: self.skips.clone(),
        };
        ops::setup_release_dir(&ro)?;

        let namespace = self.get_namespace(&mut tx).await?;
        println!("namespace: {}", namespace);

        // Get all of the services in the environments.
        let all_services = self.get_services(&mut tx).await?;

        // From the list of all services, get those that should be deployed
        // first.
        let pre_deploy_services = all_services
            .iter()
            .filter(|svc| {
                self.pre_deploy
                    .contains(&svc.name.as_ref().unwrap_or(&"".to_string()))
            })
            .collect::<Vec<&db::Service>>();

        println!(
            "services to deploy first:{}",
            pre_deploy_services.iter().fold(String::new(), |acc, svc| {
                format!(
                    "{}\n\t{}",
                    acc,
                    svc.name.as_ref().unwrap_or(&"".to_string())
                )
            })
        );

        // Get the rest of the services, excluding those that should be deployed
        // first.
        let services = all_services
            .iter()
            .filter(|svc| {
                !self
                    .pre_deploy
                    .contains(&svc.name.as_ref().unwrap_or(&"".to_string()))
            })
            .collect::<Vec<&db::Service>>();

        println!(
            "services:{}",
            services.iter().fold(String::new(), |acc, svc| {
                format!(
                    "{}\n\t{}",
                    acc,
                    svc.name.as_ref().unwrap_or(&"".to_string())
                )
            })
        );

        //let templatesdir = self.repodir.join("templates");
        println!("templates dir: {}", self.repodir.display());

        // Create the configuration file directory for the environment.
        if !self.configdir.exists() {
            std::fs::create_dir(&self.configdir)?;
        }

        let env_configdir = self.configdir.join(&self.env);
        if !env_configdir.exists() {
            std::fs::create_dir(&env_configdir)?;
        }

        // Generate the configuration files
        if !self.no_render_configs {
            templates::render_db(&mut tx, &self.env, &self.repodir, &env_configdir).await?;
        }

        // Load the configs.
        if !self.no_load_configs {
            configs::load_configs(&namespace, "service-configs", &env_configdir)?;
        }

        // Load the secrets.
        if !self.no_load_secrets {
            let secrets_dir = self.repodir.join("templates").join("secrets");
            configs::load_secrets(&namespace, &secrets_dir)?;
        }

        // Deploy the services.
        if !self.no_deploy {
            pre_deploy_services.iter().for_each(|svc| {
                self.deploy_service(&namespace, svc)
                    .expect("failed to deploy service");
            });

            services.iter().for_each(|svc| {
                self.deploy_service(&namespace, svc)
                    .expect("failed to deploy service");
            });
        }

        Ok(true)
    }
}
