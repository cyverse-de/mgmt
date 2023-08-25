//! # Dolt
//!
//! This module contains functions for interacting with Dolt.
use anyhow::{Context, Result};
use duct::{cmd, Handle};
use std::process::Command;

/// Uses Dolt to clone a repository from the remote repository.
///
/// # Examples
/// ```ignore
///     let result = dolt::clone("discoenv/de_releases", "db/de_releases").unwrap();
///     assert_eq!(result, true);
/// ```
pub fn clone(dolt_repo: &str, db_dir: &str) -> Result<bool> {
    Ok(Command::new("dolt")
        .args(["clone", dolt_repo, db_dir])
        .status()
        .context("Failed to clone dolt repo")?
        .success())
}

/// Uses Dolt to start up the database in the background.
///
/// # Examples
/// ```ignore
///    let handle = dolt::start("db/de_releases").unwrap();
/// ```
pub fn start(db_dir: &str) -> Result<Handle> {
    Ok(cmd!("dolt", "sql-server")
        .dir(db_dir)
        .stderr_to_stdout()
        .stdout_capture()
        .start()
        .context("Failed to start dolt server")?)
}
