use mgmt::config_values::config;
use mgmt::db;

use clap::{arg, Command};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

fn cli() -> Command {
    Command::new("mgmt-configs")
        .about("Manages config values files for the DE")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .default_value("mysql://root@127.0.0.1:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(
            Command::new("env")
                .args_conflicts_with_subcommands(true)
                .subcommand(Command::new("create")),
        )
        .subcommand(
            Command::new("sections")
                .args_conflicts_with_subcommands(true)
                .subcommand(
                    Command::new("add")
                        .args_conflicts_with_subcommands(true)
                        .args([arg!(-s --"section" <SECTION>)
                            .required(true)
                            .value_parser(clap::value_parser!(String))]),
                )
                .subcommand(
                    Command::new("delete")
                        .args_conflicts_with_subcommands(true)
                        .args([arg!(-s --"section" <SECTION>)
                            .required(true)
                            .value_parser(clap::value_parser!(String))]),
                )
                .subcommand(Command::new("list").args_conflicts_with_subcommands(true)),
        )
        .subcommand(
            Command::new("values")
                .args_conflicts_with_subcommands(true)
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
                                    "string", "int", "bigint", "float", "bool", "json", "csv",
                                    "tsv", "yaml", "xml",
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
                ),
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
        Some(("env", sub_m)) => {
            let create_cmd = sub_m
                .subcommand()
                .ok_or_else(|| anyhow::anyhow!("bad command"))?;

            match create_cmd {
                ("create", _) => {
                    create_env(&pool).await?;
                }
                (name, _) => {
                    unreachable!("Bad subcommand: {name}")
                }
            }
        }
        Some(("sections", sub_m)) => {
            let section_cmd = sub_m
                .subcommand()
                .ok_or_else(|| anyhow::anyhow!("bad command"))?;

            match section_cmd {
                ("add", sub_m) => {
                    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                        anyhow::anyhow!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    })?;
                    let mut tx = pool.begin().await?;
                    db::add_section(&mut tx, &section).await?;
                    tx.commit().await?;
                }
                ("delete", sub_m) => {
                    let section = sub_m.get_one::<String>("section").unwrap_or_else(|| {
                        panic!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    });
                    let mut tx = pool.begin().await?;
                    db::delete_section(&mut tx, &section).await?;
                    tx.commit().await?;
                }
                ("list", _) => {
                    let mut tx = pool.begin().await?;
                    let sections = db::list_sections(&mut tx).await?;
                    tx.commit().await?;
                    println!("{:?}", sections);
                }
                (name, _) => {
                    unreachable!("Bad subcommand: {name}")
                }
            }
        }
        Some(("values", sub_m)) => {
            let values_cmd = sub_m
                .subcommand()
                .ok_or_else(|| anyhow::anyhow!("bad command"))?;

            match values_cmd {
                ("add", sub_m) => {
                    let environment = sub_m.get_one::<String>("environment").unwrap_or_else(|| {
                        panic!(
                            "No environment specified. Use --environment <environment> to specify an environment."
                        )
                    });
                    let key = sub_m.get_one::<String>("key").unwrap_or_else(|| {
                        panic!("No key specified. Use --key <key> to specify a key.")
                    });
                    let value = sub_m.get_one::<String>("value").unwrap_or_else(|| {
                        panic!("No value specified. Use --value <value> to specify a value.")
                    });
                    let value_type = sub_m.get_one::<String>("type").unwrap_or_else(|| {
                        panic!("No type specified. Use --type <type> to specify a type.")
                    });
                    let mut tx = pool.begin().await?;
                    let env_id = db::get_env_id(&mut tx, &environment)
                        .await?
                        .ok_or_else(|| {
                            anyhow::anyhow!("No environment found with name: {environment}")
                        })?;
                    let cfg_id =
                        db::set_config_value(&mut tx, &environment, &key, &value, &value_type)
                            .await?;
                    db::add_env_cfg_value(&mut tx, env_id, cfg_id).await?;
                    tx.commit().await?;
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
