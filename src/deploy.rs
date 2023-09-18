//! Discovery Environment Deployment Functionality
//!
//! This module contains the functions that can be reused across the mgmt
//! commands to deploy the Discovery Environment.
use anyhow::{Context, Result};
use sqlx::{MySql, Pool, Transaction};
use std::path::PathBuf;
use std::process::Command;

use crate::db;

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
}

impl Deployment {
    pub fn new(
        pool: Pool<MySql>,
        repodir: PathBuf,
        branch: String,
        env: String,
        skips: Vec<String>,
    ) -> Self {
        Self {
            pool,
            repodir,
            branch,
            env,
            skips,
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

    pub async fn deploy(&self) -> Result<bool> {
        let mut tx = self.pool.begin().await?;
        self.checkout()?;
        let namespace = self.get_namespace(&mut tx).await?;
        println!("namespace: {}", namespace);
        let services = self.get_services(&mut tx).await?;
        println!("services: {:?}", services);

        Ok(true)
    }
}
