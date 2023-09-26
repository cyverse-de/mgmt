use clap::{arg, Command};
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
}
