use clap::{arg, ArgAction, Command};

pub fn cli() -> Command {
    Command::new("env")
        .about("Manage environments for the DE")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .help("The URL of the MySQL database to connect to.")
                .default_value("mysql://root@127.0.0.1:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(
            Command::new("create").args([
                arg!(-n --env <NAME> "The environment to create")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
                arg!(-n --namespace <NAMESPACE> "The Kubernetes namespace to create")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
                arg!(-f --from <FROM> "The name of the environment to use as the basis for the new environment. Inherits services, config templates but not config values from the original environment.")
                    .required(false)
                    .default_value("de")
                    .value_parser(clap::value_parser!(String))
            ]),
        )
        .subcommand(
            Command::new("delete").args([arg!(-n --name <ENV> "The environment to delete")
                .required(true)
                .value_parser(clap::value_parser!(String))]),
        )
        .subcommand(Command::new("list").about("Lists the environments in the database."))
        .subcommand(
            Command::new("populate")
                .args_conflicts_with_subcommands(true)
                .about("Populates the environments table with a new environment"),
        )
        .subcommand(
            Command::new("service")
                .subcommand(
                    Command::new("add")
                        .about("Adds one or more services to an environment.")
                        .args([
                            arg!(-e --env <ENV> "The environment to add the service to.")
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-s --service [SERVICE] "The name of the service to add.")
                                .required(true)
                                .action(ArgAction::Append)
                                .value_parser(clap::value_parser!(String)),
                        ]),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Deletes one or more services from an environment. Does not remove the service entirely from the system.")
                        .args([
                            arg!(-e --env <ENV> "Removes a service from an environment.")
                                .required(true)
                                .value_parser(clap::value_parser!(String)),
                            arg!(-s --service [SERVICE] "The name of the service to remove.")
                                .required(true)
                                .action(ArgAction::Append)
                                .value_parser(clap::value_parser!(String)),
                        ]),
                )
                .subcommand(
                    Command::new("list").args([
                        arg!(-e --env <ENV> "The environment to list the services for.")
                            .required(true)
                            .value_parser(clap::value_parser!(String)),
                    ]),
                ),
        )
}
