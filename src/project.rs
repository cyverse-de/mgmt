use crate::git;
use anyhow::Context;
use std::path::{Path, PathBuf};

pub fn create_project_dir(project_name: &str) -> anyhow::Result<PathBuf> {
    let project_dir = Path::new(project_name);
    if project_dir.exists() {
        anyhow::bail!("Project directory already exists");
    }
    std::fs::create_dir(project_dir)?;
    Ok(project_dir.to_path_buf())
}

pub fn clone_repos(project_dir: &Path, repos: &[url::Url]) -> anyhow::Result<()> {
    for repo_url in repos {
        let repo_name = repo_url
            .path_segments()
            .context("missing path for repo url")?
            .last()
            .context("empty path for repo url")?;

        let repo_dir = project_dir.join(repo_name);
        if repo_dir.exists() {
            continue;
        }

        git::clone(
            repo_url.as_str(),
            &repo_dir
                .to_str()
                .context("failed to create repo dir string")?,
        )?;
    }
    Ok(())
}
