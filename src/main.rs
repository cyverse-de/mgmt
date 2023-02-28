use anyhow::{anyhow, Context, Result};
use clap::{ArgGroup, Parser};
use std::path::Path;
use std::process::{Command, Stdio};
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

    fn configs_dir(&self) -> Result<String> {
        Ok(String::from(
            Path::new("resources")
                .join("configs")
                .join(&self.environment)
                .canonicalize()
                .context("failed to get absolute path to the configs dir")?
                .to_str()
                .context(format!(
                    "failed to get config dir for env {}",
                    &self.environment
                ))?,
        ))
    }

    fn secrets_dir(&self) -> Result<String> {
        Ok(String::from(
            Path::new("resources")
                .join("secrets")
                .join(&self.environment)
                .canonicalize()
                .context("failed to get absolute path to secrets dir")?
                .to_str()
                .context(format!(
                    "failed to get secrets dir for env {}",
                    &self.environment
                ))?,
        ))
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

    fn load_configs(&self) -> Result<bool> {
        let cfg_dir = self.configs_dir()?;
        let dry_run = Command::new("kubectl")
            .args(["-n", &self.namespace])
            .arg("create")
            .arg("secret")
            .arg("generic")
            .arg("service-configs")
            .args(["--from-file", &cfg_dir])
            .arg("--dry-run")
            .args(["-o", "yaml"])
            .stdout(Stdio::piped())
            .spawn()
            .context("failed to run dry run command in load_configs()")?;
        let load_cmd = Command::new("kubectl")
            .args(["-n", &self.namespace])
            .arg("apply")
            .args(["-f", "-"])
            .stdin(Stdio::from(dry_run.stdout.context("error getting stdout")?))
            .status()
            .context("unable to load configs")?
            .success();
        Ok(load_cmd)
    }

    fn load_resource(&self, resource_file_path: &str) -> Result<bool> {
        let abs_path = Path::new(resource_file_path).canonicalize()?;
        let rfp = abs_path.to_str().context("couldn't get path string")?;
        let cmd = Command::new("kubectl")
            .args(["-n", &self.namespace])
            .arg("apply")
            .args(["-f", rfp])
            .status()
            .context(format!("unable to load {}", rfp))?
            .success();
        Ok(cmd)
    }

    fn load_resource_type(&self, resource_type: &str) -> Result<bool> {
        let abs_path = Path::new(resource_type)
            .join(resource_type)
            .join(format!("{}.yml", &self.environment))
            .canonicalize()?;
        let file_resource = abs_path.to_str().context("couldn't get path string")?;
        self.load_resource(file_resource)
    }

    fn load_secrets(&self) -> Result<bool> {
        let s_dir = self.secrets_dir()?;

        let s_files: Vec<String> = fs::read_dir(s_dir)?
            .into_iter()
            .flat_map(|entry| entry.ok())
            .filter_map(|entry| {
                let m = entry.metadata().ok()?;
                let buf = entry.path();
                let p = buf.to_str()?;
                if m.is_file() {
                    Some(String::from(p))
                } else {
                    None
                }
            })
            .collect();

        let result = s_files
            .iter()
            .map(|s_file| self.load_resource(s_file).unwrap_or_else(|_| false))
            .reduce(|acc, v| acc && v)
            .context("failed to get result of loading secrets")?;

        Ok(result)
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

    fn deploy_project(&self, project: &str) -> Result<bool> {
        let submodule_path = self.repo_path(project)?;
        let build_path = self.build_file_path(project)?;

        if !Path::new(&submodule_path).exists() {
            return Err(anyhow!("missing submodule path {}", submodule_path));
        };

        Ok(Command::new("skaffold")
            .current_dir(submodule_path)
            .arg("deploy")
            .args(["--namespace", &self.namespace])
            .args(["--build-artifacts", &build_path])
            .arg("--force")
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

    fn do_shared_deployment_steps(&self) -> Result<bool> {
        println!("loading configs into namespace {}", self.namespace);
        self.load_configs()?;
        println!("done loading configs into namespace {}", self.namespace);

        println!("loading ingresses");
        self.load_resource_type("ingresses")?;
        println!("done loading ingresses");

        println!("loading secrets");
        self.load_secrets()?;
        println!("done loading secrets");

        println!("loading services");
        let result = self.load_resource_type("services");
        println!("done loading services");

        result
    }

    fn do_deployment(&self, project: &str, shared: bool) -> Result<bool> {
        if shared {
            self.do_shared_deployment_steps()?;
        }
        println!("fetch the submodules");
        fetch_submodule(&self.repo_path(project)?)?;
        println!("done fetch the submodules");

        println!("deploying the project");
        let result = self.deploy_project(project);
        println!("done deploying the project");

        result
    }

    fn check_in_changes(&self, project: &str) -> Result<bool> {
        let submodule_path = self.repo_path(project)?;

        git_add(&submodule_path)?;
        git_add("builds")?;
        if staged_changes()? {
            let msg = format!("update builds for the {} project", &project);
            git_commit(&msg)?;
        };

        Ok(true)
    }

    fn process(&self) -> Result<bool> {
        let is_shared = self.projects.len() > 1;
        if is_shared {
            let shared_complete = self
                .do_shared_deployment_steps()
                .context("failed to do shared deployment steps")?;
            if !shared_complete {
                return Err(anyhow!(
                    "invalid status returned from shared deployment steps"
                ));
            }
        }
        for project in self.projects.iter() {
            if self.do_build {
                if self.do_build(&project).context("do_build failed")? {
                    return Err(anyhow!("non-zero status returned from build steps"));
                };
            }
            if self.do_deploy {
                if self
                    .do_deployment(&project, is_shared)
                    .context("do_deployment failed")?
                {
                    return Err(anyhow!("non-zero status returned from deployment steps"));
                };
            }
            if self.do_build && self.do_check_in {
                if self
                    .check_in_changes(&project)
                    .context("check_in_changes failed")?
                {
                    return Err(anyhow!("non-zero status returned from checking in changes"));
                };
            }
        }

        if self.clean {
            println!("clean not yet implemented");
        }

        Ok(true)
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

fn git_add(path: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["add", "--all", path])
        .status()
        .context("git add failed")?
        .success())
}

fn git_commit(msg: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args(["commit", "-m", msg])
        .status()
        .context("git commit failed")?
        .success())
}

fn fetch_submodule(submodule_path: &str) -> Result<bool> {
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

fn update_submodule(submodule_path: &str) -> Result<bool> {
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

    match state.process() {
        Ok(status) => {
            if !status {
                println!("failed to deploy");
                return;
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
