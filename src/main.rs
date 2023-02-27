use anyhow::{anyhow, Context, Result};
use clap::{ArgGroup, Parser};
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::{fs, io, str};
use which::which;

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
struct Cli {
    #[arg(short, long)]
    /// Build and/or deploy all projects for which a build description file exists.
    all_projects: bool,

    #[arg(short, long)]
    /// The names of the proejcts to deploy. Corresponds to a submodule in the repos directory.
    project: Vec<String>,

    #[arg(short, long)]
    /// The Kubernetes namespace to deploy to.
    namespace: String,

    #[arg(short, long)]
    /// The name of the environment to update. (default: namespace name)
    environment: Option<String>,

    #[arg(long, default_value_t = String::from("builds"))]
    /// The path to the builds directory.
    builds_path: String,

    #[arg(long, default_value_t = String::from("repos"))]
    /// The path to the repos directory.
    repos_path: String,

    #[arg(short = 'b', long, group = "build_group")]
    /// Build the project before deploying it.
    build: bool,

    #[arg(short = 'B', long, group = "build_group")]
    /// Don't build the project before deploying it.
    no_build: bool,

    #[arg(short = 'd', long, group = "deploy_group")]
    /// Deploy the project.
    deploy: bool,

    #[arg(short = 'D', long, group = "deploy_group")]
    /// Don't deploy the project. Only useful alongside -b and -c.
    no_deploy: bool,

    #[arg(short = 'c', long, group = "check_in_group")]
    /// Check in the changes after deploying the artifact.
    check_in: bool,

    #[arg(short = 'C', long, group = "check_in_group")]
    /// Don't check in the changes after deploying the artifact.
    no_check_in: bool,

    #[arg(long)]
    /// Run an image cleanup after build and deploy, if they are run.
    clean: bool,
}

#[derive(Debug)]
struct State {
    projects: Vec<String>,
    namespace: String,
    environment: String,
    repos_path: String,
    builds_path: String,
    do_build: bool,
    do_deploy: bool,
    do_check_in: bool,
    clean: bool,
}

impl State {
    fn from(cli: Cli) -> State {
        let projects: Vec<String>;

        if cli.all_projects {
            projects = get_projects_from_build_dir(&cli.builds_path).unwrap_or(vec![]);
        } else {
            projects = cli.project.clone()
        }

        State {
            projects: projects,
            namespace: cli.namespace.clone(),
            environment: cli.environment.clone().unwrap_or(cli.namespace.clone()),
            repos_path: cli.repos_path.clone(),
            builds_path: cli.builds_path.clone(),
            do_build: cli.build && !cli.no_build,
            do_deploy: cli.deploy && !cli.no_deploy,
            do_check_in: cli.check_in && !cli.no_check_in,
            clean: cli.clean,
        }
    }

    fn repo_path(&self, repo: &str) -> Result<String> {
        Ok(String::from(
            Path::new(&self.repos_path)
                .join(repo)
                .canonicalize()?
                .to_str()
                .context(format!("failed to get the repo path for {}", repo))?,
        ))
    }

    fn build_file_path(&self, project: &str) -> Result<String> {
        Ok(String::from(
            Path::new("builds")
                .join(format!("{}.json", project))
                .canonicalize()?
                .to_str()
                .context("failed to build path to build JSON file")?,
        ))
    }

    fn build_project(&self, project: &str) -> Result<bool> {
        let submodule_path = self.repo_path(project)?;
        let build_file = self.build_file_path(project)?;

        if !Path::new(&submodule_path).exists() {
            return Err(anyhow!("missing submodule path {}", submodule_path));
        };

        Ok(Command::new("skaffold")
            .current_dir(submodule_path)
            .arg("build")
            .args(["--namespace", &self.namespace])
            .args(["--file-output", &build_file])
            .status()?
            .success())
    }

    fn do_build(&self, project: &str) -> Result<bool> {
        let submodule_path = self.repo_path(project)?;

        println!("generating configs");
        generate_all_configs()?;
        println!("done generating configs");

        println!("updating the submodules");
        update_submodule(&submodule_path).context("error updating submodule")?;
        println!("done updating the submodules");

        println!("printing the {} project", project);
        let build_result = self.build_project(project);
        println!("done building the {} project", project);

        build_result
    }
}

fn get_projects_from_build_dir(builds_path: &str) -> Result<Vec<String>> {
    let projects: Vec<String> = fs::read_dir(builds_path)?
        .into_iter()
        .flat_map(|entry| entry.ok())
        .filter_map(|entry| {
            let m = entry.metadata().ok()?;
            let p = entry.path();
            let ext = p.extension()?.to_str()?;
            if m.is_file() && ext == "json" {
                Some(String::from(
                    entry.file_name().to_str()?.strip_suffix(".json")?,
                ))
            } else {
                None
            }
        })
        .collect();

    Ok(projects)
}

fn git_add(path: &str) -> Result<ExitStatus> {
    Ok(Command::new("git").args(["add", "--all", path]).status()?)
}

fn git_commit(msg: &str) -> Result<ExitStatus> {
    Ok(Command::new("git").args(["commit", "-m", msg]).status()?)
}

fn fetch_submodule(submodule_path: &str) -> Result<ExitStatus> {
    Ok(Command::new("git")
        .args([
            "submodule",
            "update",
            "--remote",
            "--init",
            "--recursive",
            submodule_path,
        ])
        .status()?)
}

fn update_submodule(submodule_path: &str) -> Result<ExitStatus> {
    fetch_submodule(submodule_path)?;
    Ok(Command::new("git").args(["add", submodule_path]).status()?)
}

fn staged_changes() -> Result<bool> {
    let output = Command::new("git").arg("status").output()?;

    let found = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .contains("nothing to commit");

    Ok(found)
}

fn config_dir(env: &str) -> Result<String> {
    Ok(String::from(
        Path::new("resources")
            .join("configs")
            .join(env)
            .canonicalize()?
            .to_str()
            .context("no env config dir")?,
    ))
}

fn config_values_path(env: &str) -> Result<String> {
    Ok(String::from(
        Path::new("config_values")
            .join(format!("{}.yaml", env))
            .canonicalize()?
            .to_str()
            .context("failed to create path to env config_values file")?,
    ))
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

fn generate_configs(env: &str) -> Result<bool> {
    let cfg_dir = config_dir(env)?;
    let config_values_file = config_values_path(env)?;

    Ok(Command::new("gomplate")
        .args(["--input-dir", "templates/configs"])
        .args(["--output-dir", &cfg_dir])
        .args(["-d", &format!("config={}", config_values_file)])
        .status()?
        .success())
}

fn generate_all_configs() -> Result<bool> {
    let mut success: bool = false;
    let envs = list_envs()?;
    for env in envs.iter() {
        let r = generate_configs(env)?;
        success = success && r;
        if !r {
            println!("failed to generate configs for {}", &env)
        }
    }
    Ok(success)
}

fn main() {
    let cli = Cli::parse();

    let skaffold_path = match which("skaffold") {
        Ok(path_buf) => path_buf,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let git_path = match which("git") {
        Ok(path_buf) => path_buf,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("skaffold path is {}", skaffold_path.display());
    println!("git path is {}", git_path.display());

    let state = State::from(cli);
    println!("projects: {:?}", state.projects);
    println!("namespace: {}", state.namespace);
    println!("environment: {}", state.environment);
    println!("repos_path: {}", state.repos_path);
    println!("builds_path: {}", state.builds_path);
    println!("do_check_in: {}", state.do_check_in);
    println!("do_build: {}", state.do_build);
    println!("do_deploy: {}", state.do_deploy);
    println!("clean: {}", state.clean);
}
