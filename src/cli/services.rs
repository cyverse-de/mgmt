use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("services")
        .about("Manage services for the DE")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .help("The URL of the MySQL database to connect to.")
                .default_value("mysql://root@localhost:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(Command::new("list").about("Lists the services in the database."))
}
