use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{arg, Command};
use mgmt::cli::{
    configs, container_images, deploy, envs, release, repos, services, site, templates,
};
use mgmt::handlers;
use mgmt::{app, db};
use sqlx::postgres::PgPoolOptions;
use tabled::Table;
use which::which;

#[tokio::main]
async fn main() -> Result<()> {
    let commands = Command::new("mgmt")
        .version("0.1.0")
        .about("Discovery Environment deployment management tool")
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .default_value("postgresql://root@127.0.0.1:5432/de_releases?sslmode=disable")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(configs::cli())
        .subcommand(container_images::cli())
        .subcommand(release::cli())
        .subcommand(site::cli())
        .subcommand(deploy::cli())
        .subcommand(templates::cli())
        .subcommand(services::cli())
        .subcommand(envs::cli())
        .subcommand(repos::cli())
        .get_matches();

    let database_url = commands.get_one::<String>("database-url").context(
        "No database URL specified. Use --database-url <url> to specify a database URL.",
    )?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| anyhow::anyhow!("error connecting to database: {}", e))?;

    match commands.subcommand() {
        Some(("configs", sub_m)) => match sub_m.subcommand() {
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

        Some(("deploy", sub_m)) => match sub_m.subcommand() {
            Some(("backwards-compat", sub_m)) => {
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
            _ => unreachable!("Bad deploy subcommand"),
        },

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

            Some(("render-file-db", sub_m)) => {
                let template_path = sub_m.get_one::<PathBuf>("template").context(
                    "No template file specified. Use --template <path> to specify a template file.",
                )?;

                let env = sub_m.get_one::<String>("environment").context(
                    "No environment specified. Use --environment <name> to specify an environment.",
                )?;

                let output_path = sub_m.get_one::<PathBuf>("output").context(
                    "No output file specified. Use --output <path> to specify an output file.",
                )?;

                let mut tx = pool.begin().await?;
                handlers::templates::render_template_from_db(
                    &mut tx,
                    template_path,
                    &env,
                    output_path,
                )
                .await?;

                tx.commit().await?;
            }

            Some(("render-dir-db", sub_m)) => {
                let templates_path = sub_m.get_one::<PathBuf>("templates").context(
                    "No templates directory specified. Use --templates <path> to specify a templates directory.",
                )?;

                let env = sub_m.get_one::<String>("environment").context(
                    "No environment specified. Use --environment <name> to specify an environment.",
                )?;

                let output_path = sub_m.get_one::<PathBuf>("output").context(
                    "No output directory specified. Use --output <path> to specify an output directory.",
                )?;

                let mut tx = pool.begin().await?;
                handlers::templates::render_template_dir_from_db(
                    &mut tx,
                    templates_path,
                    &env,
                    output_path,
                )
                .await?;

                tx.commit().await?;
            }

            Some(("render-db", sub_m)) => {
                let templates_path = sub_m.get_one::<PathBuf>("templates").context(
                    "No templates directory specified. Use --templates <path> to specify a templates directory.",
                )?;

                let env = sub_m.get_one::<String>("environment").context(
                    "No environment specified. Use --environment <name> to specify an environment.",
                )?;

                let output_path = sub_m.get_one::<PathBuf>("output").context(
                    "No output directory specified. Use --output <path> to specify an output directory.",
                )?;

                let mut tx = pool.begin().await?;
                handlers::templates::render_db(&mut tx, &env, templates_path, output_path).await?;

                tx.commit().await?;
            }

            Some(("assoc", sub_m)) => {
                let env = sub_m.get_one::<String>("environment").context(
                    "No environment specified. Use --environment <name> to specify an environment.",
                )?;

                let repo_id = sub_m.get_one::<i32>("repo-id").context(
                    "No repository name specified. Use --repo-name <name> to specify a repository name.",
                )?;

                let svc_name = sub_m.get_one::<String>("service").context(
                    "No service name specified. Use --service <name> to specify a service name.",
                )?;

                let templates = sub_m
                    .get_many::<String>("template")
                    .unwrap_or_default()
                    .map(|s| PathBuf::from(s))
                    .collect::<Vec<PathBuf>>();

                let mut tx = pool.begin().await?;
                handlers::templates::assoc_template(&mut tx, &env, *repo_id, &svc_name, &templates)
                    .await?;
                tx.commit().await?;
            }

            Some(("list", sub_m)) => {
                let templates = sub_m
                    .get_many::<String>("template")
                    .unwrap_or_default()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>();

                let mut tx = pool.begin().await?;
                let template_entries = db::list_template_info(&mut tx, &templates).await?;
                tx.commit().await?;

                println!("{}", Table::new(&template_entries).to_string());
            }

            _ => unreachable!("Bad templates subcommand"),
        },

        Some(("services", sub_m)) => match sub_m.subcommand() {
            Some(("list", _)) => {
                let services = handlers::services::list_all_services(&pool).await?;

                for service in services {
                    println!("{}", service);
                }
            }
            _ => unreachable!("Bad services subcommand"),
        },

        Some(("env", sub_m)) => handlers::envs::env(&pool, &sub_m).await?,

        Some(("repos", sub_m)) => match sub_m.subcommand() {
            Some(("list", _)) => {
                let mut tx = pool.begin().await?;
                let repo_list = db::list_repos(&mut tx).await?;
                tx.commit().await?;

                println!("{}", Table::new(&repo_list).to_string());
            }

            Some(("add", sub_m)) => {
                let name = sub_m.get_one::<String>("name").context(
                    "No repository name specified. Use --name <name> to specify a repository name.",
                )?;

                let url = sub_m.get_one::<url::Url>("url").context(
                    "No repository URL specified. Use --url <url> to specify a repository URL.",
                )?;

                let revision = sub_m.get_one::<String>("revision").context(
                    "No repository revision specified. Use --revision <revision> to specify a repository revision.",
                )?;

                let mut tx = pool.begin().await?;
                let new_id = db::add_repo(&mut tx, &name, &url, &revision).await?;
                tx.commit().await?;

                println!("Added repository {} with ID {}", name, new_id);
            }

            Some(("delete", sub_m)) => {
                let id = sub_m.get_one::<i32>("id").context(
                    "No repository ID specified. Use --id <id> to specify a repository ID.",
                )?;

                let mut tx = pool.begin().await?;
                db::delete_repo(&mut tx, *id).await?;
                tx.commit().await?;

                println!("Deleted repository with ID {}", id);
            }
            _ => unreachable!("Bad repos subcommand"),
        },

        _ => unreachable!("Bad subcommand"),
    };

    Ok(())
}
