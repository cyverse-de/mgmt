use anyhow::Result;
use mgmt::cli::container_images;
use mgmt::handlers::container_images as ci_handlers;
use sqlx::mysql::MySqlPoolOptions;

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
        Some(("insert", sub_m)) => ci_handlers::insert(&pool, &sub_m).await?,
        Some(("upsert", sub_m)) => ci_handlers::upsert(&pool, &sub_m).await?,
        Some(("upsert-builds", sub_m)) => ci_handlers::upsert_multi_builds(&pool, &sub_m).await?,
        Some(("upsert-a-build", sub_m)) => ci_handlers::upsert_single_build(&pool, &sub_m).await?,
        Some(("delete", sub_m)) => ci_handlers::delete(&pool, &sub_m).await?,
        Some(("list", _)) => ci_handlers::list_all_images(&pool).await?,
        _ => unreachable!(),
    }

    Ok(())
}
