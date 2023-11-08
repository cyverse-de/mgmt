use clap::Command;

pub fn cli() -> Command {
    Command::new("services")
        .about("Manage services for the DE")
        .subcommand_required(true)
        .subcommand(Command::new("list").about("Lists the services in the database."))
}
