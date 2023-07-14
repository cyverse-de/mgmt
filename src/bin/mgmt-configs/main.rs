use mgmt::config_values::config;
use mgmt::db;

use anyhow::anyhow;
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
                            arg!(-s --"section" <SECTION>)
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
                            arg!(-s --"section" <SECTION>)
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
                            arg!(-s --"section" <SECTION>)
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
                            arg!(-s --"section" <SECTION>)
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

async fn add_section(pool: &Pool<MySql>, section: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    db::add_section(&mut tx, &section).await?;
    tx.commit().await?;
    Ok(())
}

async fn delete_section(pool: &Pool<MySql>, section: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    db::delete_section(&mut tx, &section).await?;
    tx.commit().await?;
    Ok(())
}

async fn list_sections(pool: &Pool<MySql>) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let sections = db::list_sections(&mut tx).await?;
    tx.commit().await?;
    println!("{:?}", sections);
    Ok(())
}

async fn set_value(
    pool: &Pool<MySql>,
    environment: &str,
    section: &str,
    key: &str,
    value: &str,
    value_type: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let env_id = db::get_env_id(&mut tx, &environment)
        .await?
        .ok_or_else(|| anyhow::anyhow!("No environment found with name: {environment}"))?;
    let cfg_id = db::set_config_value(&mut tx, section, &key, &value, &value_type).await?;
    db::add_env_cfg_value(&mut tx, env_id, cfg_id).await?;
    tx.commit().await?;
    Ok(())
}

async fn get_value(
    pool: &Pool<MySql>,
    environment: &str,
    section: &str,
    key: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let cfg = db::get_config_value(&mut tx, environment, section, key).await?;
    tx.commit().await?;
    println!("{:?}", cfg);
    Ok(())
}

async fn delete_value(
    pool: &Pool<MySql>,
    environment: &str,
    section: &str,
    key: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let cfg = db::delete_config_value(&mut tx, environment, section, key).await?;
    tx.commit().await?;
    println!("Deleted {:?}", cfg);
    Ok(())
}

async fn list_values(
    pool: &Pool<MySql>,
    environment: Option<&str>,
    section: Option<&str>,
    key: Option<&str>,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let cfgs = db::list_config_values(&mut tx, environment, section, key).await?;
    tx.commit().await?;
    println!("{:?}", cfgs);
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
                        anyhow!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    })?;

                    add_section(&pool, &section).await?;
                }
                ("delete", sub_m) => {
                    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                        anyhow!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    })?;
                    delete_section(&pool, &section).await?;
                }
                ("list", _) => {
                    list_sections(&pool).await?;
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
                ("set", sub_m) => {
                    let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
                        anyhow!(
                            "No environment specified. Use --environment <environment> to specify an environment."
                        )
                    })?;

                    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                        anyhow!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    })?;

                    let key = sub_m.get_one::<String>("key").ok_or_else(|| {
                        anyhow!("No key specified. Use --key <key> to specify a key.")
                    })?;

                    let value = sub_m.get_one::<String>("value").ok_or_else(|| {
                        anyhow!("No value specified. Use --value <value> to specify a value.")
                    })?;

                    let value_type = sub_m.get_one::<String>("type").ok_or_else(|| {
                        anyhow!("No type specified. Use --type <type> to specify a type.")
                    })?;

                    set_value(&pool, &environment, &section, &key, &value, &value_type).await?;
                }

                ("get", sub_m) => {
                    let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
                        anyhow!(
                            "No environment specified. Use --environment <environment> to specify an environment."
                        )
                    })?;

                    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                        anyhow!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    })?;

                    let key = sub_m.get_one::<String>("key").ok_or_else(|| {
                        anyhow!("No key specified. Use --key <key> to specify a key.")
                    })?;

                    get_value(&pool, &environment, &section, &key).await?;
                }

                ("delete", sub_m) => {
                    let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
                        anyhow!(
                            "No environment specified. Use --environment <environment> to specify an environment."
                        )
                    })?;

                    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                        anyhow!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    })?;

                    let key = sub_m.get_one::<String>("key").ok_or_else(|| {
                        anyhow!("No key specified. Use --key <key> to specify a key.")
                    })?;

                    delete_value(&pool, &environment, &section, &key).await?;
                }

                ("list", sub_m) => {
                    let environment = match sub_m.get_one::<String>("environment") {
                        Some(env) => Some(env.as_str()),
                        None => None,
                    };

                    let section = match sub_m.get_one::<String>("section") {
                        Some(section) => Some(section.as_str()),
                        None => None,
                    };

                    let key = match sub_m.get_one::<String>("key") {
                        Some(key) => Some(key.as_str()),
                        None => None,
                    };

                    list_values(&pool, environment, section, key).await?;
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
