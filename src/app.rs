use crate::cli::Cli;
use crate::configs;
use crate::git;
use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{fs, str};

#[derive(Debug)]
pub struct App {
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

impl App {
    pub fn from(cli: Cli) -> App {
        let projects: Vec<String>;

        if cli.all_projects {
            projects = get_projects_from_build_dir(&cli.builds_path).unwrap_or(vec![]);
        } else {
            projects = cli.project.clone()
        }

        App {
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

    pub fn new() -> App {
        let empty: Vec<String> = Vec::new();
        App {
            projects: empty,
            namespace: String::from("default"),
            environment: String::from("qa"),
            repos_path: String::from("repos"),
            builds_path: String::from("builds"),
            do_build: false,
            do_deploy: false,
            do_check_in: false,
            clean: false,
        }
    }

    pub fn print_fields(&self) {
        println!("projects: {:?}", self.projects);
        println!("namespace: {}", self.namespace);
        println!("environment: {}", self.environment);
        println!("repos_path: {}", self.repos_path);
        println!("builds_path: {}", self.builds_path);
        println!("do_check_in: {}", self.do_check_in);
        println!("do_build: {}", self.do_build);
        println!("do_deploy: {}", self.do_deploy);
        println!("clean: {}", self.clean);
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

    fn dry_run_cmd(&self, cfg_dir: &str) -> Command {
        let mut command: Command = Command::new("kubectl");
        command
            .args(["-n", &self.namespace])
            .arg("create")
            .arg("secret")
            .arg("generic")
            .arg("service-configs")
            .args(["--from-file", &cfg_dir])
            .arg("--dry-run")
            .args(["-o", "yaml"]);
        command
    }

    fn load_cmd(&self) -> Command {
        let mut command: Command = Command::new("kubectl");
        command
            .args(["-n", &self.namespace])
            .arg("apply")
            .args(["-f", "-"]);
        command
    }

    fn load_configs(&self) -> Result<bool> {
        let cfg_dir = configs::dir(&self.environment)?;

        let mut dr = self.dry_run_cmd(&cfg_dir);
        let mut load = self.load_cmd();

        println!("running: {:#?}", dr);
        println!("running: {:#?}", load);

        let dry_run = dr
            .stdout(Stdio::piped())
            .spawn()
            .context("failed to run dry run command in load_configs()")?;

        let load_cmd = load
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
        let s_dir = configs::secrets_dir(&self.environment)?;

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

    fn skaffold_build_cmd(&self, submodule_path: &str, build_path: &str) -> Command {
        let mut command: Command = Command::new("skaffold");
        command
            .current_dir(submodule_path)
            .arg("build")
            .args(["--namespace", &self.namespace])
            .args(["--file-output", &build_path]);
        command
    }

    fn build_project(&self, project: &str) -> Result<bool> {
        let submodule_path = self.repo_path(project)?;
        let build_file = self.build_file_path(project)?;

        if !Path::new(&submodule_path).exists() {
            return Err(anyhow!("missing submodule path {}", submodule_path));
        };

        let mut build = self.skaffold_build_cmd(&submodule_path, &build_file);

        println!("running: {:?}", build);

        Ok(build.status()?.success())
    }

    fn skaffold_deploy_cmd(&self, build_path: &str, submodule_path: &str) -> Command {
        let mut command: Command = Command::new("skaffold");
        command
            .current_dir(submodule_path)
            .arg("deploy")
            .args(["--namespace", &self.namespace])
            .args(["--build-artifacts", &build_path])
            .arg("--force");
        command
    }

    fn deploy_project(&self, project: &str) -> Result<bool> {
        let submodule_path = self.repo_path(project)?;
        let build_path = self.build_file_path(project)?;

        if !Path::new(&submodule_path).exists() {
            return Err(anyhow!("missing submodule path {}", submodule_path));
        };

        let mut deploy = self.skaffold_deploy_cmd(&build_path, &submodule_path);

        println!("running: {:?}", deploy);

        Ok(deploy.status()?.success())
    }

    fn do_build(&self, project: &str) -> Result<bool> {
        let submodule_path = self.repo_path(project)?;

        println!("generating configs");
        configs::generate_all()?;
        println!("done generating configs");

        println!("updating the submodules");
        git::update_submodule(&submodule_path).context("error updating submodule")?;
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
        git::fetch_submodule(&self.repo_path(project)?)?;
        println!("done fetch the submodules");

        println!("deploying the project");
        let result = self.deploy_project(project);
        println!("done deploying the project");

        result
    }

    pub fn process(&self) -> Result<bool> {
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
            let project_path = self.repo_path(project)?;

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
                if git::check_in_changes(&project_path).context("check_in_changes failed")? {
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new() {
        let n = App::new();
        assert!(!n.clean);
        assert!(!n.do_check_in);
        assert!(!n.do_build);
        assert!(!n.do_deploy);
        assert_eq!(n.builds_path, "builds");
        assert_eq!(n.environment, "qa");
        assert_eq!(n.namespace, "default");
        let empty: Vec<String> = Vec::new();
        assert!(n.projects == empty);
    }
}
