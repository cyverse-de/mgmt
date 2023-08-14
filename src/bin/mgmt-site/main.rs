// mgmt-site sets up a directory containing repos and configuration values for a DE deployment site.
//
// A site consists of one or more DE deployments.

use clap::{arg, ArgAction, Command};

/**
 * Set up the CLI for the mgmt-site binary.
 */
fn cli() -> Command {
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
                arg!(-r --"db-remote" [DB_REMOTE] "The Dolt DB remote to set up and use for initializing the local DB.")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
                arg!(-f --force "Overwrite existing files")
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool))
            ]),
        )
}

fn init(dir: &str, remote: &str, force: bool) -> anyhow::Result<()> {
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", matches)) => {
            let dir = matches.get_one::<String>("dir").ok_or_else(|| {
                anyhow::anyhow!("No directory specified. Use -d or --dir to specify a directory.")
            })?;
            let db_remote = matches.get_one::<String>("db-remote").ok_or_else(|| {
                anyhow::anyhow!("No Dolt DB remote specified. Use -r or --db-remote to specify a Dolt DB remote.")
            })?;
            let force = matches.get_flag("force");
            init(dir, db_remote, force)
        }
        _ => unreachable!(),
    }
}
