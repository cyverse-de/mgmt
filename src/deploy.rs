//! Discovery Environment Deployment Functionality
//!
//! This module contains the functions that can be reused across the mgmt
//! commands to deploy the Discovery Environment.
use anyhow::{Context, Result};
use sqlx::{MySql, Pool, Transaction};
use std::path::PathBuf;
use std::process::Command;

use crate::config_values;
use crate::configs;
use crate::db;
use crate::ops;

pub struct Deployment {
    // The database connection pool.
    pool: Pool<MySql>,

    // The path to the directory containing the de-releases repository.
    repodir: PathBuf,

    // The branch of the de-releases repository to deploy from.
    branch: String,

    // The name of the environment to deploy.
    env: String,

    // The name of the services to NOT deploy.
    skips: Vec<String>,

    // The base directory for the configurations
    configdir: PathBuf,

    // The options for which optional sections to include in the configurations
    section_opts: config_values::config::SectionOptions,
}

impl Deployment {
    pub fn new(
        pool: Pool<MySql>,
        repodir: PathBuf,
        branch: String,
        env: String,
        skips: Vec<String>,
        configdir: PathBuf,
        section_opts: config_values::config::SectionOptions,
    ) -> Self {
        Self {
            pool,
            repodir,
            branch,
            env,
            skips,
            configdir,
            section_opts,
        }
    }

    fn checkout(&self) -> Result<bool> {
        Ok(Command::new("git")
            .args(["checkout", &self.branch])
            .current_dir(&self.repodir)
            .status()
            .context("git checkout failed")?
            .success())
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

        self.checkout()?;

        let namespace = self.get_namespace(&mut tx).await?;
        println!("namespace: {}", namespace);

        let services = self.get_services(&mut tx).await?;
        println!("services: {:?}", services);

        let templatesdir = self.repodir.join("templates");

        // Create the configuration file directory for the environment.
        if !self.configdir.exists() {
            std::fs::create_dir(&self.configdir)?;
        }

        let env_configdir = self.configdir.join(&self.env);
        if !env_configdir.exists() {
            std::fs::create_dir(&env_configdir)?;
        }

        // Serialize the configuration defaults
        let defaults_file = env_configdir.join("defaults.yaml");
        ops::render_default_values(&self.pool, Some(defaults_file.clone())).await?;

        // Serialize the configuration values files
        let values_file = env_configdir.clone().join("values.yaml");
        ops::render_values(
            &self.pool,
            &self.env,
            &self.section_opts,
            Some(values_file.clone()),
        )
        .await?;

        // Generate the configuration files
        let input_dir = templatesdir
            .to_str()
            .context("failed to get the templates dir")?;
        let output_dir = env_configdir.join("configs");
        let defaults_path = defaults_file
            .to_str()
            .context("failed to get the defaults path")?;
        let values_path = values_file
            .to_str()
            .context("failed to get the values path")?;
        configs::generate_cmd(
            input_dir,
            output_dir
                .to_str()
                .context("failed to get output directory path")?,
            defaults_path,
            values_path,
        )?;

        // Load the configs.
        configs::load_configs(&namespace, "service-configs", &output_dir)?;

        // Load the secrets.
        let secrets_pbuf = templatesdir.join("secrets");
        let secrets_dir = secrets_pbuf.to_str().context("failed to get secrets dir")?;
        let secrets_output_pbuf = env_configdir.join("secrets");
        let secrets_output_dir = secrets_output_pbuf
            .to_str()
            .context("failed to get secrets output dir")?;
        configs::generate_cmd(secrets_dir, secrets_output_dir, defaults_path, values_path)?;
        configs::load_secrets(&namespace, &secrets_pbuf)?;

        // Deploy the services.
        services.iter().for_each(|svc| {
            self.deploy_service(&namespace, svc)
                .expect("failed to deploy service");
        });

        Ok(true)
    }
}
