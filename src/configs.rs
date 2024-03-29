//! # configs
//!
//! Contains the functions needed for loading configs and secrets in mgmt.

use anyhow::{Context, Result};
use duct::cmd;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Get the config directory for the given environment.
///
/// # Parameters
/// * `env` - A string slice containing the name of the environment.
///
/// # Returns
/// * `anyhow::Result<String>` - An anyhow::Result wrapping a String containing the path.
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

/// Get the secrets directory for the given environment.
///
/// # Parameters
/// * `env` - A string slice containing the name of the environment.
///
/// # Returns
/// * `anyhow::Result<String>` - An anyhow::Result wrapping a String containing the path.
///
/// # Examples
/// ```
/// let env = "prod";
/// let result = mgmt::configs::secrets_dir(env).unwrap();
///
/// assert_eq!(result, "resources/secrets/prod");
/// ```
pub fn secrets_dir(env: &str) -> Result<String> {
    Ok(String::from(
        Path::new("resources")
            .join("secrets")
            .join(env)
            .to_str()
            .context(format!("failed to get secrets dir for env {}", env))?,
    ))
}

/// Get the path to the directory containing configuration values.
///
/// # Parameters
/// * `env` - A string slice containing the name of the environment.
///
/// # Returns
/// * `anyhow::Result<String>` - An anyhow::Result containing the relative path to the directory.
///
/// # Examples
///
/// ```ignore
/// let env = "prod";
/// let result = values_path(env).unwrap();
///
/// assert_eq!(result, "config_values/prod.yaml")
/// ```
fn values_path(env: &str) -> Result<String> {
    Ok(String::from(
        Path::new("config_values")
            .join(format!("{}.yaml", env))
            .to_str()
            .context("failed to create path to env config_values file")?,
    ))
}

pub fn generate_cmd(
    input_dir: &str,
    output_dir: &str,
    defaults_path: &str,
    values_path: &str,
) -> Result<()> {
    cmd!(
        "gomplate",
        "-d",
        "config=merge:env|defaults",
        "-d",
        &format!("env={}", values_path),
        "-d",
        &format!("defaults={}", defaults_path),
        "--input-dir",
        input_dir,
        "--output-dir",
        output_dir
    )
    .run()
    .context("failed to generate configs")?;

    Ok(())
}

fn generate(env: &str, defaults_path: &str) -> Result<bool> {
    let dir_buf = fs::canonicalize(dir(env)?)?;
    let cfg_dir = dir_buf
        .to_str()
        .context("failed to get the absolute path to the configs dir")?;

    let cv_buf = fs::canonicalize(values_path(env)?)?;
    let config_values_file = cv_buf
        .to_str()
        .context("failed to get the absolute path to the config_values directory")?;

    Ok(Command::new("gomplate")
        .args(["-d", "config=merge:env|defaults"])
        .args(["-d", &format!("env={}", config_values_file)])
        .args(["-d", &format!("defaults={}", defaults_path)])
        .args(["--input-dir", "templates/configs"])
        .args(["--output-dir", &cfg_dir])
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

/// Renders all of the configuration files.
///
/// # Returns
/// * `anyhow::Result<bool>` - An anyhow::Result containing a boolean indicating whether the operation succeeded.
///
/// # Examples
/// ```no_run
/// let generation_succeeded = mgmt::configs::generate_all().unwrap();
///
/// assert!(generation_succeeded);
/// ```
pub fn generate_all(defaults_path: &str) -> Result<bool> {
    let mut success: bool = false;
    let envs = list_envs()?;
    for env in envs.iter() {
        let r = generate(env, &defaults_path)?;
        success = success && r;
        if !r {
            println!("failed to generate configs for {}", &env)
        }
    }
    Ok(success)
}

/// Load the configuration values at a given path for a provided namespace
/// and environment.
pub fn load_configs(ns: &str, configmap_name: &str, cfg_dir: &PathBuf) -> Result<bool> {
    Ok(cmd!(
        "kubectl",
        "-n",
        ns,
        "create",
        "secret",
        "generic",
        configmap_name,
        format!(
            "--from-file={}",
            cfg_dir
                .to_str()
                .context("failed to get the absolute path to the configs dir")?
        ),
        "--dry-run",
        "-o",
        "yaml"
    )
    .pipe(cmd!("kubectl", "-n", ns, "apply", "-f", "-"))
    .run()?
    .status
    .success())
}

pub fn load_secrets(ns: &str, secrets_dir: &PathBuf) -> Result<bool> {
    let contents = fs::read_dir(secrets_dir)?;
    for entry in contents.into_iter() {
        let entry = entry?;
        if entry.metadata()?.is_file() {
            let path = entry.path();
            cmd!(
                "kubectl",
                "-n",
                ns,
                "apply",
                "-f",
                path.to_str()
                    .context("failed to get the absolute path to the secret file")?
            )
            .run()?;
        }
    }

    Ok(true)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_values_path() {
        let env = "prod";
        let result = values_path(env).unwrap();
        assert_eq!(result, "config_values/prod.yaml");
    }
}
