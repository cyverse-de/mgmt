//! # configs
//!
//! Contains the functions needed for loading configs and secrets in mgmt.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Get the config directory for the given environment.
///
/// # Arguments
/// * `env` - A string slice containing the name of the environment.
///
/// # Examples
///
/// ```
/// let env = "prod";
/// let result = mgmt::configs::dir(env).unwrap();
///
/// assert_eq!(result, "resources/configs/prod");
/// ```
pub fn dir(env: &str) -> Result<String> {
    Ok(String::from(
        Path::new("resources")
            .join("configs")
            .join(env)
            .to_str()
            .context(format!("failed to get the configs dir for env {}", env))?,
    ))
}

pub fn secrets_dir(env: &str) -> Result<String> {
    Ok(String::from(
        Path::new("resources")
            .join("secrets")
            .join(env)
            .to_str()
            .context(format!("failed to get secrets dir for env {}", env))?,
    ))
}

fn values_path(env: &str) -> Result<String> {
    Ok(String::from(
        Path::new("config_values")
            .join(format!("{}.yaml", env))
            .canonicalize()?
            .to_str()
            .context("failed to create path to env config_values file")?,
    ))
}

fn generate(env: &str) -> Result<bool> {
    let dir_buf = fs::canonicalize(dir(env)?)?;
    let cfg_dir = dir_buf
        .to_str()
        .context("failed to get the absolute path to the configs dir")?;

    let cv_buf = fs::canonicalize(values_path(env)?)?;
    let config_values_file = cv_buf
        .to_str()
        .context("failed to get the absolute path to the config_values directory")?;

    Ok(Command::new("gomplate")
        .args(["--input-dir", "templates/configs"])
        .args(["--output-dir", &cfg_dir])
        .args(["-d", &format!("config={}", config_values_file)])
        .status()?
        .success())
}

fn list_envs() -> Result<Vec<String>> {
    let envs = fs::read_dir("config_values")?
        .into_iter()
        .flat_map(|r| r.ok())
        .filter_map(|entry| {
            let m = entry.metadata().ok()?;
            let p = entry.path();
            let ext = p.extension()?.to_str()?;
            if m.is_file() && (ext == "yaml" || ext == "yml") {
                Some(String::from(entry.file_name().to_str()?.strip_suffix(ext)?))
            } else {
                None
            }
        })
        .collect();
    Ok(envs)
}

pub fn generate_all() -> Result<bool> {
    let mut success: bool = false;
    let envs = list_envs()?;
    for env in envs.iter() {
        let r = generate(env)?;
        success = success && r;
        if !r {
            println!("failed to generate configs for {}", &env)
        }
    }
    Ok(success)
}
