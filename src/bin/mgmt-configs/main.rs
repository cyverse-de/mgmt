use mgmt::config_values::config;
use mgmt::db::{self, Configuration, LoadFromConfiguration};

use anyhow::anyhow;
use clap::{arg, Command};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

/**
 * Sets up the CLI for the mgmt-configs binary.
 */
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
                .subcommand(
                    Command::new("populate")
                        .args_conflicts_with_subcommands(true)
                        .about("Populates the environments table with a new environment"),
                )
                .subcommand(
                    Command::new("create").args([
                        arg!(--"name" <NAME>)
                            .required(true)
                            .value_parser(clap::value_parser!(String)),
                        arg!(--"namespace" <NAMESPACE>)
                            .required(true)
                            .value_parser(clap::value_parser!(String)),
                    ]),
                )
                .subcommand(Command::new("list"))
                .subcommand(
                    Command::new("delete").args([arg!(--"name" <NAME>)
                        .required(true)
                        .value_parser(clap::value_parser!(String))]),
                ),
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
                )
                .subcommand(
                    Command::new("render")
                        .args_conflicts_with_subcommands(true)
                        .args([arg!(
                            -e --"environment" <ENVIRONMENT>
                                "The environment to render the config values for"
                        )]),
                ),
        )
        .subcommand(
            Command::new("defaults")
                .args_conflicts_with_subcommands(true)
                .subcommand(
                    Command::new("set")
                        .args_conflicts_with_subcommands(true)
                        .args([
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

/**
 * Handler for the `mgmt-configs env populate` command.
 */
async fn populate_env(pool: &Pool<MySql>) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let mut env_config = config::ConfigValues::default();
    env_config.ask_for_info(&mut tx).await?;
    tx.commit().await?;
    Ok(())
}

/**
 * Handler for the `mgmt-configs sections add` command.
 */
async fn add_section(pool: &Pool<MySql>, section: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    db::add_section(&mut tx, &section).await?;
    tx.commit().await?;
    Ok(())
}
/**
 * Handler for the `mgmt-configs sections delete` command.
 */
async fn delete_section(pool: &Pool<MySql>, section: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    db::delete_section(&mut tx, &section).await?;
    tx.commit().await?;
    Ok(())
}

/**
 * Handler for the `mgmt-configs sections list` command.
 */
async fn list_sections(pool: &Pool<MySql>) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let sections = db::list_sections(&mut tx).await?;
    tx.commit().await?;
    for section in sections {
        println!("{}", section);
    }
    Ok(())
}

/**
 * Handler for the `mgmt-configs defaults set` command.
 */
async fn set_default_value(
    pool: &Pool<MySql>,
    section: &str,
    key: &str,
    value: &str,
    value_type: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let has_section = db::has_section(&mut tx, section).await?;
    if has_section {
        let cfg_id =
            db::set_default_config_value(&mut tx, section, &key, &value, &value_type).await?;
        println!("Added default config value with and ID of {}", cfg_id);
    } else {
        return Err(anyhow!("No section found with name: {section}"));
    }
    tx.commit().await?;
    Ok(())
}

/**
 * Handler for the `mgmt-configs defaults get` command.
 */
async fn get_default_value(pool: &Pool<MySql>, section: &str, key: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let cfg: db::Configuration;
    let has_section = db::has_section(&mut tx, section).await?;
    if has_section {
        let has_default_value = db::has_default_config_value(&mut tx, section, key).await?;
        if has_default_value {
            cfg = db::get_default_config_value(&mut tx, section, key).await?;
        } else {
            return Err(anyhow!(
                "No default value found for section: {section}, key: {key}"
            ));
        }
    } else {
        return Err(anyhow!("No section found with name: {section}"));
    }
    tx.commit().await?;
    println!("{:?}", cfg);
    Ok(())
}

/**
 * Handler for the `mgmt-configs defaults delete` command.
 */
async fn delete_default_value(pool: &Pool<MySql>, section: &str, key: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let has_section = db::has_section(&mut tx, section).await?;
    if has_section {
        let has_default_value = db::has_default_config_value(&mut tx, section, key).await?;
        if has_default_value {
            db::delete_default_config_value(&mut tx, section, key).await?;
        } else {
            return Err(anyhow!(
                "No default value found for section: {section}, key: {key}"
            ));
        }
    } else {
        return Err(anyhow!("No section found with name: {section}"));
    }
    tx.commit().await?;
    println!("Deleted default value: {}.{}", section, key);
    Ok(())
}

/**
 * Handler for the `mgmt-configs defaults list` command.
 */
async fn list_default_values(
    pool: &Pool<MySql>,
    section: Option<&str>,
    key: Option<&str>,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let cfgs = db::list_default_config_values(&mut tx, section, key).await?;
    tx.commit().await?;
    for cfg in cfgs {
        if let (Some(section), Some(key), Some(value)) = (cfg.section, cfg.key, cfg.value) {
            println!("{}.{} = {}", section, key, value);
        }
    }
    Ok(())
}

/**
 * Handler for the `mgmt-configs values set` command.
 */
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

    let has_default = db::has_default_config_value(&mut tx, section, &key).await?;
    if has_default {
        let cfg_id = db::set_config_value(&mut tx, section, &key, &value, &value_type).await?;
        db::add_env_cfg_value(&mut tx, env_id, cfg_id).await?;
        println!(
            "Added config value to environment '{}': {}.{} = {}",
            environment, section, key, value
        );
    } else {
        tx.rollback().await?;
        return Err(anyhow!(
            "No default value found for section: {section}, key: {key}"
        ));
    }

    tx.commit().await?;

    Ok(())
}

/**
 * Handler for the `mgmt-configs values get` command.
 */
async fn get_value(
    pool: &Pool<MySql>,
    environment: &str,
    section: &str,
    key: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let cfg: db::Configuration;

    let has_config_value = db::has_config_value(&mut tx, environment, section, key).await?;
    if has_config_value {
        cfg = db::get_config_value(&mut tx, environment, section, key).await?;
    } else {
        let has_default_value = db::has_default_config_value(&mut tx, section, key).await?;
        if has_default_value {
            cfg = db::get_default_config_value(&mut tx, section, key).await?;
        } else {
            tx.rollback().await?;
            return Err(anyhow!(
                "No default value found for section: {section}, key: {key}"
            ));
        }
    }
    tx.commit().await?;
    if let (Some(section), Some(key), Some(value)) = (cfg.section, cfg.key, cfg.value) {
        println!("{}.{} = {}", section, key, value);
    }
    Ok(())
}

/**
 * Handler for the `mgmt-configs values delete` command.
 */
async fn delete_value(
    pool: &Pool<MySql>,
    environment: &str,
    section: &str,
    key: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    db::delete_config_value(&mut tx, environment, section, key).await?;
    tx.commit().await?;
    println!(
        "Deleted config value from environment '{}': {}.{}",
        environment, section, key
    );
    Ok(())
}

/**
 * Handler for the `mgmt-configs values list` command.
 */
async fn list_values(
    pool: &Pool<MySql>,
    environment: Option<&str>,
    section: Option<&str>,
    key: Option<&str>,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let cfgs = db::list_config_values(&mut tx, environment, section, key).await?;
    tx.commit().await?;
    for cfg in cfgs {
        if let (Some(section), Some(key), Some(value)) = (cfg.section, cfg.key, cfg.value) {
            println!("{}.{} = {}", section, key, value);
        }
    }
    Ok(())
}

/**
 * Handler  for the `mgmt-configs values render` command.
 */
async fn render_values(pool: &Pool<MySql>, environment: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let mut all_cfgs: Vec<Configuration> = Vec::new();

    let all_default_cfgs = db::list_default_config_values(&mut tx, None, None).await?;
    for default in all_default_cfgs.into_iter() {
        if let (Some(section), Some(key)) = (default.section.clone(), default.key.clone()) {
            if db::has_config_value(&mut tx, environment, &section, &key)
                .await
                .unwrap_or(false)
            {
                all_cfgs.push(
                    db::get_config_value(&mut tx, environment, &section, &key)
                        .await
                        .unwrap(),
                );
            } else {
                all_cfgs.push(default);
            }
        }
    }

    let mut cv = config::ConfigValues::default();
    cv.cfg_set_keys(all_cfgs)?;

    let yaml = serde_yaml::to_string(&cv)?;
    println!("{}", yaml);

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
                ("populate", _) => {
                    populate_env(&pool).await?;
                }

                ("create", sub_m) => {
                    let name = sub_m.get_one::<String>("name").ok_or_else(|| {
                        anyhow!("No name specified. Use --name <name> to specify a name.")
                    })?;

                    let namespace = sub_m.get_one::<String>("namespace").ok_or_else(|| {
                        anyhow!("No namespace specified. Use --namespace <namespace> to specify a namespace.")
                    })?;

                    let mut tx = pool.begin().await?;
                    db::upsert_environment(&mut tx, &name, &namespace).await?;
                    tx.commit().await?;

                    println!("Created environment: {}", name);
                }

                ("list", _) => {
                    let mut tx = pool.begin().await?;
                    let envs = db::list_envs(&mut tx).await?;
                    tx.commit().await?;
                    for env in envs {
                        println!("{}", env);
                    }
                }

                ("delete", sub_m) => {
                    let name = sub_m.get_one::<String>("name").ok_or_else(|| {
                        anyhow!("No name specified. Use --name <name> to specify a name.")
                    })?;

                    let mut tx = pool.begin().await?;
                    db::delete_env(&mut tx, &name).await?;
                    tx.commit().await?;

                    println!("Deleted environment: {}", name);
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
                // Match on the `sections add` subcommand.
                ("add", sub_m) => {
                    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                        anyhow!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    })?;

                    add_section(&pool, &section).await?;
                }

                // Match on the `sections delete` subcommand.
                ("delete", sub_m) => {
                    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                        anyhow!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    })?;

                    delete_section(&pool, &section).await?;
                }

                // Match on the `sections list` subcommand.
                ("list", _) => {
                    list_sections(&pool).await?;
                }

                (name, _) => {
                    unreachable!("Bad subcommand: {name}")
                }
            }
        }

        Some(("defaults", sub_m)) => {
            let defaults_cmd = sub_m
                .subcommand()
                .ok_or_else(|| anyhow::anyhow!("bad command"))?;

            match defaults_cmd {
                ("set", sub_m) => {
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

                    set_default_value(&pool, &section, &key, &value, &value_type).await?;
                }

                ("get", sub_m) => {
                    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                        anyhow!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    })?;

                    let key = sub_m.get_one::<String>("key").ok_or_else(|| {
                        anyhow!("No key specified. Use --key <key> to specify a key.")
                    })?;

                    get_default_value(&pool, &section, &key).await?;
                }

                ("delete", sub_m) => {
                    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                        anyhow!(
                            "No section specified. Use --section <section> to specify a section."
                        )
                    })?;

                    let key = sub_m.get_one::<String>("key").ok_or_else(|| {
                        anyhow!("No key specified. Use --key <key> to specify a key.")
                    })?;

                    delete_default_value(&pool, &section, &key).await?;
                }

                ("list", sub_m) => {
                    let section = match sub_m.get_one::<String>("section") {
                        Some(section) => Some(section.as_str()),
                        None => None,
                    };

                    let key = match sub_m.get_one::<String>("key") {
                        Some(key) => Some(key.as_str()),
                        None => None,
                    };

                    list_default_values(&pool, section, key).await?;
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
                // Match on the `values set` subcommand.
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

                // Match on the `values get` subcommand.
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

                // Match on the `values delete` subcommand.
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

                // Match on the `values list` subcommand.
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

                ("render", sub_m) => {
                    let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
                        anyhow!(
                            "No environment specified. Use --environment <environment> to specify an environment."
                        )
                    })?;

                    render_values(&pool, &environment).await?;
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
