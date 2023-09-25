use anyhow::{Context, Result};
use mgmt::cli::release;
use mgmt::handlers::releases as release_handlers;
use sqlx::mysql::MySqlPoolOptions;
use which::which;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = release::cli().get_matches();

    let git_path = which("git").context("git not found")?;
    let skaffold_path = which("skaffold").context("skaffold not found")?;
    let kubectl_path = which("kubectl").context("kubectl not found")?;
    let gomplate_path = which("gomplate").context("gomplate not found")?;

    println!("git path: {}", git_path.display());
    println!("skaffold path: {}", skaffold_path.display());
    println!("kubectl path: {}", kubectl_path.display());
    println!("gomplate path: {}", gomplate_path.display());

    let database_url = matches
        .get_one::<String>("database-url")
        .unwrap_or_else(|| {
            panic!("No database URL specified. Use --database-url <url> to specify a database URL.")
        });

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    match matches.subcommand() {
        Some(("create", matches)) => release_handlers::create(&pool, &matches).await?,

        Some(("deploy", matches)) => release_handlers::deploy(&pool, &matches).await?,

        _ => {
            println!("No subcommand was used");
        }
    }

    Ok(())
}
