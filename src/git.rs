//! # Git
//!
//! This module provides functions for interacting with git.
use anyhow::{Context, Result};
use std::io;
use std::path::PathBuf;
use std::process::Command;

pub fn add(repodir: &PathBuf, path: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["add", "--all", path])
        .current_dir(repodir)
        .status()
        .context("git add failed")?
        .success())
}

pub fn commit(repodir: &PathBuf, msg: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["commit", "-m", msg])
        .current_dir(repodir)
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

pub fn push(repodir: &PathBuf, remote: &str, gref: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["push", remote, gref])
        .current_dir(repodir)
        .status()
        .context("git push failed")?
        .success())
}

pub fn list_tags(repodir: &PathBuf, remote: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["ls-remote", "--tags", remote])
        .current_dir(repodir)
        .output()
        .context("git ls-remote failed")?;

    let tags = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .lines()
        .map(|line| {
            line.split('\t')
                .nth(1)
                .map(|tag| tag.replace("refs/tags/", "").to_string())
        })
        .filter_map(|tag| tag)
        .collect::<Vec<String>>();

    Ok(tags)
}

pub fn tag(repodir: &PathBuf, tag: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["tag", tag])
        .current_dir(repodir)
        .status()
        .context("git tag failed")?
        .success())
}

pub fn push_tags(repodir: &PathBuf, remote: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["push", remote, "--tags"])
        .current_dir(repodir)
        .status()
        .context("git push --tags failed")?
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

pub fn pull(repodir: &PathBuf) -> Result<bool> {
    Ok(Command::new("git")
        .args(["pull"])
        .current_dir(repodir)
        .status()
        .context("git pull failed")?
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
fn staged_changes(repodir: &PathBuf) -> Result<bool> {
    let output = Command::new("git")
        .arg("status")
        .current_dir(repodir)
        .output()?;

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
    let curr_dir = std::env::current_dir()?;
    add(&curr_dir, &project_path)?;
    add(&curr_dir, "builds")?;
    if staged_changes(&curr_dir)? {
        let msg = format!("update builds for the {} project", &project_path);
        commit(&curr_dir, &msg)?;
    };

    Ok(true)
}
