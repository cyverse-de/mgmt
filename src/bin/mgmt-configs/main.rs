use mgmt::cli::configs;
use mgmt::handlers;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let command = configs::cli().get_matches();

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
        Some(("env", sub_m)) => handlers::configs::env(&pool, &sub_m).await?,
        Some(("sections", sub_m)) => handlers::configs::sections(&pool, &sub_m).await?,
        Some(("defaults", sub_m)) => handlers::configs::defaults(&pool, &sub_m).await?,
        Some(("values", sub_m)) => handlers::configs::values(&pool, &sub_m).await?,
        _ => unreachable!("Bad subcommand"),
    }

    Ok(())
}
