use anyhow::{Context, Result};
use std::io;
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

/// Uses git to fetch a submodule from the remote repository.
///
///
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

pub fn update_submodule(submodule_path: &str) -> Result<bool> {
    fetch_submodule(submodule_path)?;
    Ok(Command::new("git")
        .args(["add", submodule_path])
        .status()
        .context("error updating submodule")?
        .success())
}

fn staged_changes() -> Result<bool> {
    let output = Command::new("git").arg("status").output()?;

    let found = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .contains("nothing to commit");

    Ok(!found)
}

pub fn check_in_changes(project_path: &str) -> Result<bool> {
    add(&project_path)?;
    add("builds")?;
    if staged_changes()? {
        let msg = format!("update builds for the {} project", &project_path);
        commit(&msg)?;
    };

    Ok(true)
}
