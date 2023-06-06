use mgmt::config_values::config;

use clap::{arg, Command};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

fn cli() -> Command {
    Command::new("mgmt-defaults")
        .about("Manages config values files for the DE")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("create")
                .args_conflicts_with_subcommands(true)
                .subcommand(
                    Command::new("defaults").arg(
                        arg!(-o --"output-file" <OUTPUT_FILE>)
                            .value_parser(clap::value_parser!(PathBuf)),
                    ),
                )
                .subcommand(
                    Command::new("env-config").arg(
                        arg!(-o --"output-file" <OUTPUT_FILE>)
                            .value_parser(clap::value_parser!(PathBuf)),
                    ),
                ),
        )
        .subcommand(
            Command::new("validate")
                .args_conflicts_with_subcommands(true)
                .arg(
                    arg!(-i --"input-file" <INPUT_FILE>).value_parser(clap::value_parser!(PathBuf)),
                )
                .arg(
                    arg!(-d --"defaults-file" <DEFAULTS_FILE>)
                        .default_value("defaults.yaml")
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
}

fn create_defaults(output_file: Option<&PathBuf>) -> anyhow::Result<()> {
    if let Some(p) = output_file {
        println!("output file is {}", p.display());
    }

    let writer = match output_file {
        Some(x) => Box::new(File::create(x)?) as Box<dyn Write>,
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };

    let defaults = config::ConfigValues::default();
    Ok(serde_yaml::to_writer(writer, &defaults)?)
}

fn create_env_config(output_file: Option<&PathBuf>) -> anyhow::Result<()> {
    if let Some(p) = output_file {
        println!("output file is {}", p.display());
    }

    let writer = match output_file {
        Some(x) => Box::new(File::create(x)?) as Box<dyn Write>,
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };

    let mut env_config = config::ConfigValues::default();
    env_config.ask_for_info()?;
    Ok(serde_yaml::to_writer(writer, &env_config)?)
}

fn validate_file(
    input_file: Option<&PathBuf>,
    defaults_file: Option<&PathBuf>,
) -> anyhow::Result<()> {
    if let Some(p) = input_file {
        println!("input file is {}", p.display());
    }

    if let Some(d) = defaults_file {
        println!("defaults file is {}", d.display());
    }

    let reader = match input_file {
        Some(x) => Box::new(File::open(x)?) as Box<dyn Read>,
        None => Box::new(io::stdin()) as Box<dyn Read>,
    };

    let defaults_reader = match defaults_file {
        Some(i) => Box::new(File::open(i)?) as Box<dyn Read>,
        None => Box::new(io::stdin()) as Box<dyn Read>,
    };

    let defaults: config::ConfigValues = serde_yaml::from_reader(defaults_reader)?;
    let from_values: config::ConfigValues = serde_yaml::from_reader(reader)?;
    let values: config::ConfigValues = config::ConfigValues::merge(&defaults, &from_values)?;
    eprintln!(" {}", serde_yaml::to_string(&values)?);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let command = cli().get_matches();

    match command.subcommand() {
        Some(("create", sub_m)) => {
            let create_cmd = sub_m
                .subcommand()
                .ok_or_else(|| anyhow::anyhow!("bad command"))?;

            match create_cmd {
                ("defaults", sub_m) => {
                    let output_path = sub_m.get_one::<PathBuf>("output-file");
                    create_defaults(output_path)?;
                }
                ("env-config", sub_m) => {
                    let output_path = sub_m.get_one::<PathBuf>("output-file");
                    create_env_config(output_path)?;
                }
                (name, _) => {
                    unreachable!("Bad subcommand: {name}")
                }
            }
        }
        Some(("validate", sub_m)) => {
            let input_path = sub_m.get_one::<PathBuf>("input-file");
            let defaults_path = sub_m.get_one::<PathBuf>("defaults-file");
            validate_file(input_path, defaults_path)?;
        }
        _ => unreachable!("Bad subcommand"),
    }

    Ok(())
}
