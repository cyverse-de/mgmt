// mgmt-site sets up a directory containing repos and configuration values for a DE deployment site.
//
// A site consists of one or more DE deployments.
use mgmt::cli::site;
use mgmt::handlers::sites as site_handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = site::cli().get_matches();

    match matches.subcommand() {
        Some(("init", matches)) => site_handlers::init_site(&matches).await?,
        Some(("deploy", matches)) => site_handlers::deploy_site(&matches).await?,
        _ => unreachable!(),
    }
    Ok(())
}
