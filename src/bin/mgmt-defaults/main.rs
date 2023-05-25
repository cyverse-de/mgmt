use mgmt::config_values;

use clap::Parser;
use serde_yaml::{self};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(p) = cli.output.as_deref() {
        println!("output file is {}", p.display())
    }

    let writer = match cli.output {
        Some(x) => Box::new(File::create(x)?) as Box<dyn Write>,
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };

    let defaults = config_values::ConfigValues::default();
    Ok(serde_yaml::to_writer(writer, &defaults)?)
}
