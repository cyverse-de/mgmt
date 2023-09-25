use anyhow::{anyhow, Result};
use mgmt::cli::container_images;
use mgmt::handlers::container_images as ci_handlers;
use sqlx::mysql::MySqlPoolOptions;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let command = container_images::cli().get_matches();

    let database_url = command
        .get_one::<String>("database-url")
        .unwrap_or_else(|| {
            panic!("No database URL specified. Use --database-url <url> to specify a database URL.")
        });

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    match command.subcommand() {
        Some(("insert", sub_m)) => {
            let image = sub_m.get_one::<String>("image").ok_or_else(|| {
                anyhow!("No image specified. Use --image <image> to specify an image to insert.")
            })?;
            let dockerfile = sub_m.get_one::<String>("dockerfile").ok_or_else(|| {
                anyhow!("No dockerfile specified. Use --dockerfile <dockerfile> to specify a dockerfile to insert.")
            })?;
            let service = sub_m.get_one::<String>("service").ok_or_else(|| {
                anyhow!(
                    "No service specified. Use --service <service> to specify a service to insert."
                )
            })?;
            let mut tx = pool.begin().await?;
            let repo_id = ci_handlers::get_service_repo_id(&mut tx, &service).await?;
            let container_image = ci_handlers::parse_container_image(&image)?;
            ci_handlers::insert_image(&mut tx, repo_id, &dockerfile, &container_image).await?;
            tx.commit().await?;
        }
        Some(("upsert", sub_m)) => {
            let image = sub_m.get_one::<String>("image").ok_or_else(|| {
                anyhow!("No image specified. Use --image <image> to specify an image to insert.")
            })?;
            let dockerfile = sub_m.get_one::<String>("dockerfile").ok_or_else(|| {
                anyhow!("No dockerfile specified. Use --dockerfile <dockerfile> to specify a dockerfile to insert.")
            })?;
            let service = sub_m.get_one::<String>("service").ok_or_else(|| {
                anyhow!(
                    "No service specified. Use --service <service> to specify a service to insert."
                )
            })?;
            ci_handlers::upsert_image(&pool, &image, &service, &dockerfile).await?;
        }
        Some(("upsert-builds", sub_m)) => {
            let builds_dir = sub_m.get_one::<String>("builds-dir").ok_or_else(|| {
                anyhow!("No builds-dir specified. Use --builds-dir <builds-dir> to specify a builds-dir to insert.")
            })?;
            let force_insert = sub_m.get_flag("force-insert");
            ci_handlers::upsert_builds(&pool, &builds_dir, force_insert).await?;
        }
        Some(("upsert-a-build", sub_m)) => {
            let builds_dir = sub_m.get_one::<String>("builds-dir").ok_or_else(|| {
                anyhow!("No builds-dir specified. Use --builds-dir <builds-dir> to specify a builds-dir to insert.")
            })?;

            let service = sub_m.get_one::<String>("service").ok_or_else(|| {
                anyhow!(
                    "No service specified. Use --service <service> to specify a service to insert."
                )
            })?;

            let force_insert = sub_m.get_flag("force-insert");

            let mut tx = pool.begin().await?;
            ci_handlers::upsert_build(&mut tx, &PathBuf::from(builds_dir), &service, force_insert)
                .await?;
            tx.commit().await?;
        }
        Some(("delete", sub_m)) => {
            let id = sub_m.get_one::<i32>("id").ok_or_else(|| {
                anyhow!("No id specified. Use --id <id> to specify an id to delete.")
            })?;
            ci_handlers::delete_image(&pool, id).await?;
        }
        Some(("list", _)) => {
            ci_handlers::list_images(&pool).await?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
