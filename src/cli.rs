use clap::{ArgGroup, Parser};
use std::str;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(
    group(
        ArgGroup::new("project_group")
            .required(true)
            .args(["all_projects", "project"]),
    ),
    group(
        ArgGroup::new("build_group")
            .required(true)
            .args(["build", "no_build"]),
    ),
    group(
        ArgGroup::new("deploy_group")
            .required(true)
            .args(["deploy", "no_deploy"]),
    ),
    group(
        ArgGroup::new("check_in_group")
            .required(true)
            .args(["check_in", "no_check_in"])
    ))]
pub struct Cli {
    #[arg(short, long)]
    /// Build and/or deploy all projects for which a build description file exists.
    pub all_projects: bool,

    #[arg(short, long)]
    /// The names of the projects to deploy. Corresponds to a submodule in the repos directory.
    pub project: Vec<String>,

    #[arg(short, long)]
    /// The Kubernetes namespace to deploy to.
    pub namespace: String,

    #[arg(short, long)]
    /// The name of the environment to update. (default: namespace name)
    pub environment: Option<String>,

    #[arg(long, default_value_t = String::from("builds"))]
    /// The path to the builds directory.
    pub builds_path: String,

    #[arg(long, default_value_t = String::from("repos"))]
    /// The path to the repos directory.
    pub repos_path: String,

    #[arg(short = 'b', long, group = "build_group")]
    /// Build the project before deploying it.
    pub build: bool,

    #[arg(short = 'B', long, group = "build_group")]
    /// Don't build the project before deploying it.
    pub no_build: bool,

    #[arg(short = 'd', long, group = "deploy_group")]
    /// Deploy the project.
    pub deploy: bool,

    #[arg(short = 'D', long, group = "deploy_group")]
    /// Don't deploy the project. Only useful alongside -b and -c.
    pub no_deploy: bool,

    #[arg(short = 'c', long, group = "check_in_group")]
    /// Check in the changes after deploying the artifact.
    pub check_in: bool,

    #[arg(short = 'C', long, group = "check_in_group")]
    /// Don't check in the changes after deploying the artifact.
    pub no_check_in: bool,

    #[arg(long)]
    /// Run an image cleanup after build and deploy, if they are run.
    pub clean: bool,
}
