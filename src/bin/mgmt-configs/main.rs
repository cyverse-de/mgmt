use mgmt::config_values::config;

use clap::{arg, Command};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

fn cli() -> Command {
    Command::new("mgmt-defaults")
        .about("Manages config values files for the DE")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .default_value("mysql://root@127.0.0.1:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(
            Command::new("create")
                .args_conflicts_with_subcommands(true)
                .subcommand(Command::new("env")),
        )
        .subcommand(
            Command::new("set")
                .args_conflicts_with_subcommands(true)
                .args([
                    arg!(-e --"environment" <ENVIRONMENT>)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-k --"key" <KEY>)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-v --"value" <VALUE>)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-t --"type" <TYPE>)
                        .required(true)
                        .value_parser(clap::builder::PossibleValuesParser::new([
                            "string", "int", "bigint", "float", "bool", "json", "csv", "tsv",
                            "yaml", "xml",
                        ]))
                        .help("The type of the value"),
                ]),
        )
        .subcommand(
            Command::new("get")
                .args_conflicts_with_subcommands(true)
                .args([
                    arg!(-e --"environment" <ENVIRONMENT>)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-k --"key" <KEY>)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ]),
        )
        .subcommand(
            Command::new("delete")
                .args_conflicts_with_subcommands(true)
                .args([
                    arg!(-e --"environment" <ENVIRONMENT>)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-k --"key" <KEY>)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ]),
        )
        .subcommand(
            Command::new("list")
                .args_conflicts_with_subcommands(true)
                .args([
                    arg!(-e --"environment" <ENVIRONMENT>)
                        .required(false)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-k --"key" <KEY>)
                        .required(false)
                        .value_parser(clap::value_parser!(String)),
                ]),
        )
}

async fn create_env(pool: &Pool<MySql>) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let mut env_config = config::ConfigValues::default();
    env_config.ask_for_info(&mut tx).await?;
    tx.commit().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let command = cli().get_matches();

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
        Some(("create", sub_m)) => {
            let create_cmd = sub_m
                .subcommand()
                .ok_or_else(|| anyhow::anyhow!("bad command"))?;

            match create_cmd {
                ("env", _) => {
                    create_env(&pool).await?;
                }
                (name, _) => {
                    unreachable!("Bad subcommand: {name}")
                }
            }
        }
        _ => unreachable!("Bad subcommand"),
    }

    Ok(())
}
