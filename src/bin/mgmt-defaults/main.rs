use mgmt::config_values;

use clap::{arg, Command};
use serde_yaml::{self};
use std::fs::File;
use std::io::{self, Write};
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
                ),
        )
}

fn create_defaults(output_file: Option<&PathBuf>) -> anyhow::Result<()> {
    if let Some(p) = output_file.as_deref() {
        println!("output file is {}", p.display())
    }

    let writer = match output_file {
        Some(x) => Box::new(File::create(x)?) as Box<dyn Write>,
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };

    let defaults = config_values::ConfigValues::default();
    Ok(serde_yaml::to_writer(writer, &defaults)?)
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
                (name, _) => {
                    unreachable!("Bad subcommand: {name}")
                }
            }
        }
        _ => unreachable!("Bad subcommand"),
    }

    Ok(())
}
