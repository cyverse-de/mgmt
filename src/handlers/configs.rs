use crate::{config_values::config, db, ops};
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use sqlx::{MySql, Pool};
use std::path::PathBuf;

pub async fn env(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let create_cmd = sub_m
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("bad command"))?;

    match create_cmd {
        ("populate", _) => Ok(ops::populate_env(&pool).await?),

        ("create", sub_m) => {
            let name = sub_m.get_one::<String>("name").ok_or_else(|| {
                anyhow!("No name specified. Use --name <name> to specify a name.")
            })?;

            let namespace = sub_m.get_one::<String>("namespace").ok_or_else(|| {
                anyhow!(
                    "No namespace specified. Use --namespace <namespace> to specify a namespace."
                )
            })?;

            let mut tx = pool.begin().await?;
            db::upsert_environment(&mut tx, &name, &namespace).await?;
            tx.commit().await?;

            println!("Created environment: {}", name);

            Ok(())
        }

        ("list", _) => {
            let mut tx = pool.begin().await?;
            let envs = db::list_envs(&mut tx).await?;
            tx.commit().await?;
            for env in envs {
                println!("{}", env);
            }

            Ok(())
        }

        ("delete", sub_m) => {
            let name = sub_m.get_one::<String>("name").ok_or_else(|| {
                anyhow!("No name specified. Use --name <name> to specify a name.")
            })?;

            let mut tx = pool.begin().await?;
            db::delete_env(&mut tx, &name).await?;
            tx.commit().await?;

            println!("Deleted environment: {}", name);

            Ok(())
        }

        (name, _) => {
            unreachable!("Bad subcommand: {name}")
        }
    }
}

pub async fn sections(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let section_cmd = sub_m
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("bad command"))?;

    match section_cmd {
        // Match on the `sections add` subcommand.
        ("add", sub_m) => {
            let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                anyhow!("No section specified. Use --section <section> to specify a section.")
            })?;

            ops::add_section(&pool, &section).await?;

            Ok(())
        }

        // Match on the `sections delete` subcommand.
        ("delete", sub_m) => {
            let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                anyhow!("No section specified. Use --section <section> to specify a section.")
            })?;

            ops::delete_section(&pool, &section).await?;

            Ok(())
        }

        // Match on the `sections list` subcommand.
        ("list", _) => {
            ops::list_sections(&pool).await?;

            Ok(())
        }

        (name, _) => {
            unreachable!("Bad subcommand: {name}")
        }
    }
}

pub async fn defaults(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let defaults_cmd = sub_m
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("bad command"))?;

    match defaults_cmd {
        ("set", sub_m) => {
            let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                anyhow!("No section specified. Use --section <section> to specify a section.")
            })?;

            let key = sub_m
                .get_one::<String>("key")
                .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

            let value = sub_m.get_one::<String>("value").ok_or_else(|| {
                anyhow!("No value specified. Use --value <value> to specify a value.")
            })?;

            let value_type = sub_m.get_one::<String>("type").ok_or_else(|| {
                anyhow!("No type specified. Use --type <type> to specify a type.")
            })?;

            ops::set_default_value(&pool, &section, &key, &value, &value_type).await?;

            Ok(())
        }

        ("get", sub_m) => {
            let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                anyhow!("No section specified. Use --section <section> to specify a section.")
            })?;

            let key = sub_m
                .get_one::<String>("key")
                .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

            ops::get_default_value(&pool, &section, &key).await?;

            Ok(())
        }

        ("delete", sub_m) => {
            let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                anyhow!("No section specified. Use --section <section> to specify a section.")
            })?;

            let key = sub_m
                .get_one::<String>("key")
                .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

            ops::delete_default_value(&pool, &section, &key).await?;

            Ok(())
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

            ops::list_default_values(&pool, section, key).await?;

            Ok(())
        }

        ("render", sub_m) => {
            let output_file = sub_m.get_one::<PathBuf>("file").cloned();
            ops::render_default_values(&pool, output_file).await?;

            Ok(())
        }

        (name, _) => {
            unreachable!("Bad subcommand: {name}")
        }
    }
}

pub async fn values(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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
                anyhow!("No section specified. Use --section <section> to specify a section.")
            })?;

            let key = sub_m
                .get_one::<String>("key")
                .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

            let value = sub_m.get_one::<String>("value").ok_or_else(|| {
                anyhow!("No value specified. Use --value <value> to specify a value.")
            })?;

            let value_type = sub_m.get_one::<String>("type").ok_or_else(|| {
                anyhow!("No type specified. Use --type <type> to specify a type.")
            })?;

            ops::set_value(&pool, &environment, &section, &key, &value, &value_type).await?;

            Ok(())
        }

        // Match on the `values get` subcommand.
        ("get", sub_m) => {
            let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
                        anyhow!(
                            "No environment specified. Use --environment <environment> to specify an environment."
                        )
                    })?;

            let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                anyhow!("No section specified. Use --section <section> to specify a section.")
            })?;

            let key = sub_m
                .get_one::<String>("key")
                .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

            ops::get_value(&pool, &environment, &section, &key).await?;

            Ok(())
        }

        // Match on the `values delete` subcommand.
        ("delete", sub_m) => {
            let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
                        anyhow!(
                            "No environment specified. Use --environment <environment> to specify an environment."
                        )
                    })?;

            let section = sub_m.get_one::<String>("section").ok_or_else(|| {
                anyhow!("No section specified. Use --section <section> to specify a section.")
            })?;

            let key = sub_m
                .get_one::<String>("key")
                .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

            ops::delete_value(&pool, &environment, &section, &key).await?;

            Ok(())
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

            ops::list_values(&pool, environment, section, key).await?;

            Ok(())
        }

        ("render", sub_m) => {
            let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
                        anyhow!(
                            "No environment specified. Use --environment <environment> to specify an environment."
                        )
                    })?;
            let output_file = match sub_m.get_one::<PathBuf>("file") {
                Some(file) => Some(file.clone()),
                None => None,
            };
            let opts = config::SectionOptions::new(sub_m);
            ops::render_values(&pool, &environment, &opts, output_file).await?;

            Ok(())
        }

        ("import", sub_m) => {
            let path = sub_m.get_one::<PathBuf>("file").ok_or_else(|| {
                anyhow!("No file specified. Use --file <file> to specify a file.")
            })?;

            let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
                        anyhow!(
                            "No environment specified. Use --environment <environment> to specify an environment."
                        )
                    })?;

            ops::import_yaml_file(&pool, path.to_path_buf(), environment).await?;

            Ok(())
        }

        (name, _) => {
            unreachable!("Bad subcommand: {name}")
        }
    }
}
