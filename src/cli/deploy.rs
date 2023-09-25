use clap::{arg, ArgAction, ArgGroup, Command};
use std::str;

pub fn cli() -> Command {
    Command::new("deploy")
        .about("Builds and deploys a project to a Kubernetes cluster")
        .args_conflicts_with_subcommands(true)
        .args([
            arg!(--"all-projects" "Build and/or deploy all projects for which a build description file exists.")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
            arg!(--project [PROJECT] "The names of the projects to deploy. Corresponds to a submodule in the repos directory.")
                .required(false)
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(String)),
            arg!(--namespace [NAMESPACE] "The Kubernetes namespace to deploy to.")
                .required(true)
                .value_parser(clap::value_parser!(String)),
            arg!(--environment [ENVIRONMENT] "The name of the environment to update. (default: namespace name)")
                .required(false)
                .value_parser(clap::value_parser!(String)),
            arg!(--"builds-path" [BUILDS_PATH] "The path to the builds directory.")
                .required(false)
                .value_parser(clap::value_parser!(String))
                .default_value("builds"),
            arg!(--"repos-path" [REPOS_PATH] "The path to the repos directory.")
                .required(false)
                .value_parser(clap::value_parser!(String))
                .default_value("repos"),
            arg!(--"defaults-path" [DEFAULTS_PATH] "The path to the defaults file.")
                .required(false)
                .value_parser(clap::value_parser!(String))
                .default_value("config_values/defaults.yaml"),
            arg!(-b --build "Build the project before deploying it.")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
            arg!(-B --"no-build" "Don't build the project before deploying it.")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
            arg!(-d --deploy "Deploy the project.")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
            arg!(-D --"no-deploy" "Don't deploy the project. Only useful alongside -b and -c.")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
            arg!(-c --"check-in" "Check in the changes after deploying the artifact.")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
            arg!(-C --"no-check-in" "Don't check in the changes after deploying the artifact.")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
            arg!(--clean "Run an image cleanup after build and deploy, if they are run.")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
        ])
        .groups([
            ArgGroup::new("build_group")
                .required(true)
                .args(["build", "no-build"]),
            ArgGroup::new("deploy_group")
                .required(true)
                .args(["deploy", "no-deploy"]),
            ArgGroup::new("check_in_group")
                .required(true)
                .args(["check-in", "no-check-in"]),
        ])
}
