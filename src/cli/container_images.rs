use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("mgmt-container-images")
        .about("Manages container images in the de_releases database")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .default_value("mysql://root@127.0.0.1:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(
            Command::new("upsert")
                .about("Inserts or updates a container image based on the image's name and tag")
                .args_conflicts_with_subcommands(true)
                .arg(arg!(-i --"image" <IMAGE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-s --"service" <SERVICE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f --dockerfile <DOCKERFILE>).value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            Command::new("insert")
                .args_conflicts_with_subcommands(true)
                .arg(arg!(-i --"image" <IMAGE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-s --"service" <SERVICE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f --dockerfile <DOCKERFILE>).value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            Command::new("upsert-builds")
                .args_conflicts_with_subcommands(true)
                .arg(arg!(-b --"builds-dir" <DIR>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f - -"force-insert").value_parser(clap::value_parser!(bool))),
        )
        .subcommand(
            Command::new("delete")
                .args_conflicts_with_subcommands(true)
                .arg(arg!(-i --"id" <ID>).value_parser(clap::value_parser!(i32))),
        )
        .subcommand(Command::new("list").args_conflicts_with_subcommands(true))
        .subcommand(
            Command::new("upsert-a-build")
                .args_conflicts_with_subcommands(true)
                .arg(arg!(-s --"service" <SERVICE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-b --"builds-dir" <DIR>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f - -"force-insert").value_parser(clap::value_parser!(bool))),
        )
}
