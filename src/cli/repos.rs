use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("repos")
        .about("Manage repositories for the DE")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .help("The URL of the MySQL database to connect to.")
                .default_value("mysql://root@localhost:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(Command::new("list").about("List repositories in the database."))
        .subcommand(
            Command::new("add")
                .about("Adds a repository to the database.")
                .args([
                    arg!(-n --name <NAME> "The name of the repository to add.")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-u --url <URL> "The URL of the repository to add.")
                        .required(true)
                        .value_parser(clap::value_parser!(url::Url)),
                    arg!(-r --revision <REVISION> "The revision of the repository to add.")
                        .default_value("main")
                        .value_parser(clap::value_parser!(String)),
                ]),
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes a repository from the database.")
                .args([arg!(-i --id <ID> "The ID of the repository to delete.")
                    .required(true)
                    .value_parser(clap::value_parser!(u64))]),
        )
}
