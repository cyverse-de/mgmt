use clap::{arg, ArgAction, Command};
use std::path::PathBuf;

pub fn cli() -> Command {
    Command::new("configs")
        .about("Manages config values files for the DE")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .default_value("mysql://root@127.0.0.1:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(
            Command::new("sections")
                .args_conflicts_with_subcommands(true)
                .subcommand(
                    Command::new("add")
                        .args_conflicts_with_subcommands(true)
                        .args([arg!(-s --"section" <SECTION>)
                            .required(true)
                            .value_parser(clap::value_parser!(String))]),
                )
                .subcommand(
                    Command::new("delete")
                        .args_conflicts_with_subcommands(true)
                        .args([arg!(-s --"section" <SECTION>)
                            .required(true)
                            .value_parser(clap::value_parser!(String))]),
                )
                .subcommand(Command::new("list").args_conflicts_with_subcommands(true)),
        )
        .subcommand(
            Command::new("values")
                .args_conflicts_with_subcommands(true)
                .subcommand(
                    Command::new("set")
                        .args_conflicts_with_subcommands(true)
                        .args([
                            arg!(-e --"environment" <ENVIRONMENT>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-s --"section" <SECTION>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-k --"key" <KEY>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-v --"value" <VALUE>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-t --"type" <TYPE>)
                                .required(true)
                                .value_parser(clap::builder::PossibleValuesParser::new([
                                    "string", "int", "bigint", "float", "bool", "json", "csv",
                                    "tsv", "yaml", "xml",
                                ]))
                                .help("The type of the value"),
                        ]),
                )
                .subcommand(
                    Command::new("get")
                        .args_conflicts_with_subcommands(true)
                        .args([
                            arg!(-e --"environment" <ENVIRONMENT>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-s --"section" <SECTION>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-k --"key" <KEY>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                        ]),
                )
                .subcommand(
                    Command::new("delete")
                        .args_conflicts_with_subcommands(true)
                        .args([
                            arg!(-e --"environment" <ENVIRONMENT>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-s --"section" <SECTION>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-k --"key" <KEY>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                        ]),
                )
                .subcommand(
                    Command::new("list")
                        .args_conflicts_with_subcommands(true)
                        .args([
                            arg!(-e --"environment" <ENVIRONMENT>)
                                .required(false)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-s --"section" <SECTION>)
                                .required(false)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-k --"key" <KEY>)
                                .required(false)
                                .value_parser(clap::value_parser!(String)),
                        ]),
                )
                .subcommand(
                    Command::new("render")
                        .args_conflicts_with_subcommands(true)
                        .args([
                            arg!(
                                -f --file <FILE> "The file to render the config values to"
                            )
                                .required(true)
                                .value_parser(clap::value_parser!(PathBuf)),
                            arg!(
                                -e --"environment" <ENVIRONMENT>
                                    "The environment to render the config values for"
                            ),
                            arg!(--"include-all" "Include all settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-admin" "Include the Admin settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-analytics" "Include the Analytics settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-agave" "Include the Agave settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-base-urls" "Include the BaseURLs settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-cas" "Include the CAS settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-docker" "Include the Docker settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-infosquito" "Include the InfoSquito settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-intercom" "Include the Intercom settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-jaeger" "Include the Jaeger settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-jobs" "Include the Jobs settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-jvmpopts" "Include the JVMOpts settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-permanent-id" "Include the PermanentID settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-qa" "Include the QA settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-qms" "Include the QMS settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                            arg!(--"include-unleash" "Include the Unleash settings in the rendered output")
                                .required(false)
                                .action(ArgAction::SetTrue)
                                .value_parser(clap::value_parser!(bool)),
                        ]),
                )
                .subcommand(
                    Command::new("import")
                        .args_conflicts_with_subcommands(true)
                        .args([
                            arg!(--"file" <FILE>)
                                .required(true)
                                .value_parser(clap::value_parser!(PathBuf)),
                            arg!(--"environment" <ENVIRONMENT>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                        ]),
                ),
        )
        .subcommand(
            Command::new("defaults")
                .args_conflicts_with_subcommands(true)
                .subcommand(
                    Command::new("set")
                        .args_conflicts_with_subcommands(true)
                        .args([
                            arg!(-s --"section" <SECTION>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-k --"key" <KEY>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-v --"value" <VALUE>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-t --"type" <TYPE>)
                                .required(true)
                                .value_parser(clap::builder::PossibleValuesParser::new([
                                    "string", "int", "bigint", "float", "bool", "json", "csv",
                                    "tsv", "yaml", "xml",
                                ]))
                                .help("The type of the value"),
                        ]),
                )
                .subcommand(
                    Command::new("get")
                        .args_conflicts_with_subcommands(true)
                        .args([
                            arg!(-s --"section" <SECTION>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-k --"key" <KEY>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                        ]),
                )
                .subcommand(
                    Command::new("delete")
                        .args_conflicts_with_subcommands(true)
                        .args([
                            arg!(-s --"section" <SECTION>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-k --"key" <KEY>)
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                        ]),
                )
                .subcommand(
                    Command::new("list")
                        .args_conflicts_with_subcommands(true)
                        .args([
                            arg!(-s --"section" <SECTION>)
                                .required(false)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-k --"key" <KEY>)
                                .required(false)
                                .value_parser(clap::value_parser!(String)),
                        ]),
                )
                .subcommand(
                    Command::new("render")
                        .args_conflicts_with_subcommands(true)
                        .arg(arg!(-f --file <FILE> "The file to render the config values to")
                            .value_parser(clap::value_parser!(PathBuf))),
                ),
        )
}
