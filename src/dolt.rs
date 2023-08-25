use anyhow::{Context, Result};
use duct::{cmd, Handle};
use std::process::Command;

pub fn clone(dolt_repo: &str, db_dir: &str) -> Result<bool> {
    Ok(Command::new("dolt")
        .args(["clone", dolt_repo, db_dir])
        .status()
        .context("Failed to clone dolt repo")?
        .success())
}

pub fn start(db_dir: &str) -> Result<Handle> {
    Ok(cmd!("dolt", "sql-server")
        .dir(db_dir)
        .stderr_to_stdout()
        .stdout_capture()
        .start()
        .context("Failed to start dolt server")?)
}
