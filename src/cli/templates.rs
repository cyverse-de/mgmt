use clap::{arg, ArgAction, Command};
use std::path::PathBuf;

pub fn cli() -> Command {
    Command::new("templates")
        .about("Template-related tools")
        .args_conflicts_with_subcommands(true)
        .subcommand(
            Command::new("render-file")
                .about("Render a template")
                .args([
                    arg!(-t --template [TEMPLATE] "Path to the template to render")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-d --defaults [DEFAULTS] "Path to the defaults file")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-v --values [VALUES] "Path to the values file")
                        .required(false)
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-o --output [OUTPUT] "Path to the output file")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                ]),
        )
        .subcommand(
            Command::new("render-dir")
                .about("Render a directory of templates")
                .args([
                    arg!(-t --"templates" [TEMPLATES] "Path to the templates directory")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-d --defaults [DEFAULTS] "Path to the defaults file")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-v --values [VALUES] "Path to the values file")
                        .required(false)
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-o --output [OUTPUT] "Path to the output directory")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                ]),
        )
        .subcommand(
            Command::new("render-file-db")
                .about("Render configs with values and template paths from the database.")
                .args([
                    arg!(-t --template [TEMPLATE] "Path to the template to render")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-e --environment [ENVIRONMENT] "The name of environment to pull values from.")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-o --output [OUTPUT] "Path to the output directory")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                ]),
        )
        .subcommand(
            Command::new("render-dir-db")
                .about("Render a directory of templates with values from the database.")
                .args([
                    arg!(-t --"templates" [TEMPLATES] "Path to the templates directory")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-e --environment [ENVIRONMENT] "The name of the environment to pull values from.")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-o --output [OUTPUT] "Path to the output directory")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                ])
        )
        .subcommand(
            Command::new("render-db")
                .about("Render the templates associated with an environment, populating them with values from the database.")
                .args([
                    arg!(-t --"templates" [TEMPLATES] "Path to the templates directory")
                        .required(false)
                        .default_value(".")
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-e --environment [ENVIRONMENT] "The name of the environment to update. (default: namespace name)")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-o --output [OUTPUT] "Path to the output directory")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                ])
        )
        .subcommand(
            Command::new("assoc")
                .about("Associates a template with a service in an environment.")
                .args([
                    arg!(-t --template [TEMPLATE] "A path to a template")
                        .required(true)
                        .action(ArgAction::Append)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-s --service [SERVICE] "The name of a service")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-e --environment [ENVIRONMENT] "The name of an environment")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-r --repo-id [REPO_ID] "The ID of the repo the template is in")
                        .required(true)
                        .value_parser(clap::value_parser!(u64)),
                ])
        )
        .subcommand(
            Command::new("list")
                .about("List the services, repos, and environments associated with a template a template.")
                .args([
                    arg!(-t --template [TEMPLATE] "A path to a template")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ])
        )
}
