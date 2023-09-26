use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{arg, Command};
use mgmt::app;
use mgmt::cli::{configs, container_images, deploy, release, site};
use mgmt::handlers;
use sqlx::mysql::MySqlPoolOptions;
use which::which;

#[tokio::main]
async fn main() -> Result<()> {
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

    let database_url = commands.get_one::<String>("database-url").context(
        "No database URL specified. Use --database-url <url> to specify a database URL.",
    )?;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| anyhow::anyhow!("error connecting to database: {}", e))?;

    match commands.subcommand() {
        Some(("configs", sub_m)) => match sub_m.subcommand() {
            Some(("env", sub_m)) => handlers::configs::env(&pool, &sub_m).await?,
            Some(("sections", sub_m)) => handlers::configs::sections(&pool, &sub_m).await?,
            Some(("defaults", sub_m)) => handlers::configs::defaults(&pool, &sub_m).await?,
            Some(("values", sub_m)) => handlers::configs::values(&pool, &sub_m).await?,
            _ => unreachable!("Bad configs subcommand"),
        },
        Some(("container-images", sub_m)) => match sub_m.subcommand() {
            Some(("insert", sub_m)) => handlers::container_images::insert(&pool, &sub_m).await?,
            Some(("upsert", sub_m)) => handlers::container_images::upsert(&pool, &sub_m).await?,
            Some(("upsert-builds", sub_m)) => {
                handlers::container_images::upsert_multi_builds(&pool, &sub_m).await?
            }
            Some(("upsert-a-build", sub_m)) => {
                handlers::container_images::upsert_single_build(&pool, &sub_m).await?
            }
            Some(("delete", sub_m)) => handlers::container_images::delete(&pool, &sub_m).await?,
            Some(("list", _)) => handlers::container_images::list_all_images(&pool).await?,
            _ => unreachable!("Bad container-images subcommand"),
        },
        Some(("release", sub_m)) => match sub_m.subcommand() {
            Some(("create", sub_m)) => handlers::releases::create(&pool, &sub_m).await?,
            Some(("deploy", sub_m)) => handlers::releases::deploy(&pool, &sub_m).await?,
            _ => unreachable!("Bad release subcommand"),
        },
        Some(("site", sub_m)) => match sub_m.subcommand() {
            Some(("init", sub_m)) => handlers::sites::init_site(&sub_m).await?,
            Some(("deploy", sub_m)) => handlers::sites::deploy_site(&sub_m).await?,
            _ => unreachable!("Bad site subcommand"),
        },
        Some(("deploy", sub_m)) => {
            let git_path = which("git").context("git not found")?;
            let skaffold_path = which("skaffold").context("skaffold not found")?;
            let kubectl_path = which("kubectl").context("kubectl not found")?;
            let gomplate_path = which("gomplate").context("gomplate not found")?;

            println!("git path: {}", git_path.display());
            println!("skaffold path: {}", skaffold_path.display());
            println!("kubectl path: {}", kubectl_path.display());
            println!("gomplate path: {}", gomplate_path.display());

            let a = app::App::from(&sub_m)?;
            a.process()?;
        }
        Some(("templates", sub_m)) => match sub_m.subcommand() {
            Some(("render-file", sub_m)) => {
                let template_path = sub_m.get_one::<PathBuf>("template").context(
                    "No template file specified. Use --template <path> to specify a template file.",
                )?;

                let defaults_path = sub_m.get_one::<PathBuf>("defaults").context(
                    "No defaults file specified. Use --defaults <path> to specify a defaults file.",
                )?;

                let values_path = sub_m.get_one::<PathBuf>("values").context(
                    "No values file specified. Use --values <path> to specify a values file.",
                )?;

                let output_path = sub_m.get_one::<PathBuf>("output").context(
                    "No output file specified. Use --output <path> to specify an output file.",
                )?;

                handlers::templates::render_template(
                    template_path,
                    defaults_path,
                    values_path,
                    output_path,
                )?;
            }

            Some(("render-dir", sub_m)) => {
                let templates_path = sub_m.get_one::<PathBuf>("templates").context(
                    "No templates directory specified. Use --templates <path> to specify a templates directory.",
                )?;

                let defaults_path = sub_m.get_one::<PathBuf>("defaults").context(
                    "No defaults file specified. Use --defaults <path> to specify a defaults file.",
                )?;

                let values_path = sub_m.get_one::<PathBuf>("values").context(
                    "No values file specified. Use --values <path> to specify a values file.",
                )?;

                let output_path = sub_m.get_one::<PathBuf>("output").context(
                    "No output directory specified. Use --output <path> to specify an output directory.",
                )?;

                handlers::templates::render_template_dir(
                    templates_path,
                    defaults_path,
                    values_path,
                    output_path,
                )?;
            }
            _ => unreachable!("Bad templates subcommand"),
        },
        _ => unreachable!(),
    };

    Ok(())
}
