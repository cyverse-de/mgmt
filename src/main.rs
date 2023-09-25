use anyhow::Result;
use clap::{arg, Command};
use mgmt::app;
use mgmt::cli::{configs, container_images, deploy, release, site};
use mgmt::handlers;
use sqlx::mysql::MySqlPoolOptions;
use which::which;

#[tokio::main]
async fn main() {
    let commands = Command::new("mgmt")
        .version("0.1.0")
        .about("Discovery Environment deployment management tool")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .default_value("mysql://root@127.0.0.1:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(configs::cli())
        .subcommand(container_images::cli())
        .subcommand(release::cli())
        .subcommand(site::cli())
        .subcommand(deploy::cli())
        .get_matches();

    let skaffold_path = match which("skaffold") {
        Ok(path_buf) => path_buf,
        Err(e) => {
            println!("error finding skaffold: {}", e);
            return;
        }
    };

    let git_path = match which("git") {
        Ok(path_buf) => path_buf,
        Err(e) => {
            println!("error finding git: {}", e);
            return;
        }
    };

    let kubectl_path = match which("kubectl") {
        Ok(path_buf) => path_buf,
        Err(e) => {
            println!("error finding kubectl: {}", e);
            return;
        }
    };

    println!("skaffold path: {}", skaffold_path.display());
    println!("git path: {}", git_path.display());
    println!("kubectl path: {}", kubectl_path.display());

    let database_url = commands
        .get_one::<String>("database-url")
        .unwrap_or_else(|| {
            panic!("No database URL specified. Use --database-url <url> to specify a database URL.")
        });

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .unwrap_or_else(|e| {
            println!("error connecting to database: {}", e);
            std::process::exit(1);
        });

    // let state = app::App::from(&cli).unwrap_or_else(|e| {
    //     println!("{}", e);
    //     std::process::exit(1);
    // });

    match commands.subcommand() {
        Some(("configs", sub_m)) => {}
        Some(("container-images", sub_m)) => {}
        Some(("release", sub_m)) => {}
        Some(("site", sub_m)) => {}
        Some(("deploy", sub_m)) => {}
        _ => unreachable!(),
    };
}
