//! # Git
//!
//! This module provides functions for interacting with git.
use anyhow::{Context, Result};
use std::io;
use std::path::PathBuf;
use std::process::Command;

fn add(path: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["add", "--all", path])
        .status()
        .context("git add failed")?
        .success())
}

fn commit(msg: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["commit", "-m", msg])
        .status()
        .context("git commit failed")?
        .success())
}

pub fn checkout(repodir: &PathBuf, branch: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["checkout", branch])
        .current_dir(repodir)
        .status()
        .context("git checkout failed")?
        .success())
}

/// Uses git to fetch a submodule from the remote repository.
///
/// # Examples
/// ```ignore
///     let result = mgmt::git::fetch_submodule("repos/terrain").unwrap();
///     assert_eq!(result, true);
/// ```
pub fn fetch_submodule(submodule_path: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args([
            "submodule",
            "update",
            "--remote",
            "--init",
            "--recursive",
            submodule_path,
        ])
        .status()
        .context("error fetching submodule")?
        .success())
}

/// Uses git to clone a repository from the remote repository.
///
/// # Examples
/// ```ignore
///    let result = mgmt::git::clone("https://github.com/cyverse-de/terrain", "repos/terrain").unwrap();
/// ```
pub fn clone(url: &str, path: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["clone", url, path])
        .status()
        .context("error cloning repository")?
        .success())
}

/// Uses git to update a submodule from the remote repository.
///
/// # Examples
/// ```ignore
///   let result = mgmt::git::update_submodule("repos/terrain").unwrap();
/// ```
pub fn update_submodule(submodule_path: &str) -> Result<bool> {
    fetch_submodule(submodule_path)?;
    Ok(Command::new("git")
        .args(["add", submodule_path])
        .status()
        .context("error updating submodule")?
        .success())
}

/// Uses git to look for changes to the remote repository.
///
/// # Examples
/// ```ignore
///     let result = mgmt::git::staged_changes().unwrap();
///     assert_eq!(result, true);
/// ```
fn staged_changes() -> Result<bool> {
    let output = Command::new("git").arg("status").output()?;

    let found = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .contains("nothing to commit");

    Ok(!found)
}

/// Uses git to add and commit changes to the remote repository.
///
/// # Examples
/// ```ignore
///     let result = mgmt::git::check_in_changes("repos/terrain").unwrap();
///     assert_eq!(result, true);
/// ```
pub fn check_in_changes(project_path: &str) -> Result<bool> {
    add(&project_path)?;
    add("builds")?;
    if staged_changes()? {
        let msg = format!("update builds for the {} project", &project_path);
        commit(&msg)?;
    };

    Ok(true)
}
