use clap::{arg, ArgAction, Command};
use std::path::PathBuf;

pub fn cli() -> Command {
    Command::new("mgmt-site")
        .about(
            "Sets up directory containing repos and configuration values for a DE deployment site.",
        )
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("init").args([
                arg!(-d --dir [DIR] "Directory to initialize")
                    .help("The directory containing the site information. Defaults to the currect directory.")
                    .default_value(".")
                    .value_parser(clap::value_parser!(String)),
                arg!(-r --"db-repo" [DB_REPO] "The Dolt DB repo to set up and use for initializing the local DB.")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
                arg!(-n --"db-name" [DB_NAME] "The name of the DB")
                    .default_value("de_releases")
                    .value_parser(clap::value_parser!(String)),
                arg!(-C --"no-db-clone" "Do not clone the Dolt DB repo")
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(-R --"no-repo-clone" "Do not clone the repos")
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(-f --force "Overwrite existing files")
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(-E --"no-env" "Do not prompt the user for values for an environment")
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(-D --"no-defaults" "Do not write out the default values to a file in the site directory")
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(-V --"no-values" "Do not write out the config values for the environment to a file in the site directory")
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(--"defaults-filename" [DEFAULTS_FILENAME] "The name of the file to write the default values to in the site directory")
                    .default_value("defaults.yaml")
                    .value_parser(clap::value_parser!(String)),
                arg!(--"values-filename" [VALUES_FILENAME] "The name of the file to write the config values to in the site directory")
                    .default_value("deployment.yaml")
                    .value_parser(clap::value_parser!(String)),
            ]),
        )
        .subcommand(
            Command::new("deploy")
                .args([
                    arg!(-d --dir [DIR] "Directory to deploy from")
                        .default_value(".")
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-n --"db-name" [DB_NAME] "The name of the DB")
                        .default_value("de_releases")
                        .value_parser(clap::value_parser!(String)),
                    arg!(-e --env [ENV] "The environment to deploy")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-s --service [SERVICE] "The service to deploy")
                        .required(false)
                        .action(ArgAction::Append)
                        .value_parser(clap::value_parser!(String)),
                    arg!(--"defaults-filename" [DEFAULTS_FILENAME] "The file containing the default configuration values")
                        .default_value("defaults.yaml")
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(--"values-filename" [VALUES_FILENAME] "The file containing the configuration values for the environment")
                        .default_value("deployment.yaml")
                        .value_parser(clap::value_parser!(PathBuf)),
                ])
        )
}
