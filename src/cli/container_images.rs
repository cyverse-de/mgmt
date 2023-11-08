use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("container-images")
        .about("Manages container images in the de_releases database")
        .subcommand_required(true)
        .subcommand(
            Command::new("upsert")
                .about("Inserts or updates a container image based on the image's name and tag")
                .arg(arg!(-i --"image" <IMAGE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-s --"service" <SERVICE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f --dockerfile <DOCKERFILE>).value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            Command::new("insert")
                .arg(arg!(-i --"image" <IMAGE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-s --"service" <SERVICE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f --dockerfile <DOCKERFILE>).value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            Command::new("upsert-builds")
                .arg(arg!(-b --"builds-dir" <DIR>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f - -"force-insert").value_parser(clap::value_parser!(bool))),
        )
        .subcommand(
            Command::new("delete").arg(arg!(-i --"id" <ID>).value_parser(clap::value_parser!(i32))),
        )
        .subcommand(Command::new("list"))
        .subcommand(
            Command::new("upsert-a-build")
                .arg(arg!(-s --"service" <SERVICE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-b --"builds-dir" <DIR>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f - -"force-insert").value_parser(clap::value_parser!(bool))),
        )
}
