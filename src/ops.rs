//! # Operations
//!
//! The `ops` module contains functions that are used across the various `mgmt`
//! subcommands. This includes functions for creating the site directory,
//! cloning repos into it, and the various handlers for the subcommands
//! implemented by the tools inside this crate.
//!
use crate::config_values::config;
use crate::db::{self, Configuration, LoadFromConfiguration};
use anyhow::anyhow;
use sqlx::{MySql, Pool};
use std::path::PathBuf;

/// Adds a set of configuration values for an environment to the database.
/// Interactively prompts the user for all of the values, including the
/// environment.
///
/// Handler for the `mgmt-configs env populate` command.
///
/// # Example
///
/// ```ignore
///     let pool = MySqlPoolOptions::new()
///        .max_connections(5)
///        .connect(&format!("mysql://root@127.0.0.1:3306/{}", &opts.db_name))
///        .await?;
///
///     populate_env(&pool).await?;
/// ```
pub async fn populate_env(pool: &Pool<MySql>) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let mut env_config = config::ConfigValues::default();
    env_config.ask_for_info(&mut tx).await?;
    tx.commit().await?;
    Ok(())
}

/// Adds a configuration section to the database.
///
/// Handler for the `mgmt-configs sections add` command.
///
/// # Example
/// ```ignore
///    add_section(&pool, "Agave").await?;
/// ```
pub async fn add_section(pool: &Pool<MySql>, section: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    db::add_section(&mut tx, &section).await?;
    tx.commit().await?;
    Ok(())
}
/// Deletes a configuration section from the database. Creates a transaction
/// that either commits for rolls back before the function returns.
///
/// Handler for the `mgmt-configs sections delete` command.
///
/// # Example
/// ```ignore
///     delete_section(&pool, "Agave").await?;
/// ```
pub async fn delete_section(pool: &Pool<MySql>, section: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    db::delete_section(&mut tx, &section).await?;
    tx.commit().await?;
    Ok(())
}

/// Prints the list of configuration sections to stdout.
///
/// Handler for the `mgmt-configs sections list` command.
///
/// # Example
/// ```ignore
///    list_sections(&pool).await?;
/// ```
pub async fn list_sections(pool: &Pool<MySql>) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let sections = db::list_sections(&mut tx).await?;
    tx.commit().await?;
    for section in sections {
        println!("{}", section);
    }
    Ok(())
}

/// Sets a default configuration value in the database.
///
/// Handler for the `mgmt-configs defaults set` command.
///
/// # Example
/// ```ignore
///    set_default_value(&pool, "Agave", "Key", "12345", "string").await?;
/// ```
pub async fn set_default_value(
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

/// Gets a default configuration value from the database and prints it to
/// stdout.
///
/// Handler for the `mgmt-configs defaults get` command.
///
/// # Example
/// ```ignore
///   get_default_value(&pool, "Agave", "Key").await?;
/// ```
pub async fn get_default_value(pool: &Pool<MySql>, section: &str, key: &str) -> anyhow::Result<()> {
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

/// Deletes a default configuration value from the database and prints out a
/// status message to stdout.
///
/// Handler for the `mgmt-configs defaults delete` command.
///
/// # Example
/// ```ignore
///     delete_default_value(&pool, "Agave", "Key").await?;
/// ```
pub async fn delete_default_value(
    pool: &Pool<MySql>,
    section: &str,
    key: &str,
) -> anyhow::Result<()> {
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

/// Lists default configuration values from the database and prints them to
/// stdout.
///
/// Handler for the `mgmt-configs defaults list` command.
///
/// # Example
/// To list all of the default configuation values:
/// ```ignore
///    list_default_values(&pool, None, None).await?;
/// ```
///
/// To list all of the default configuration values for a section:
/// ```ignore
///   list_default_values(&pool, Some("Agave"), None).await?;
/// ```
///
/// To list all of the default configuration values for a section and key:
/// ```ignore
///     list_default_values(&pool, Some("Agave"), Some("Key"))
/// ```
///
/// To list all of the default configuration values for a key:
/// ```ignore
///    list_default_values(&pool, None, Some("Key"))
/// ```
pub async fn list_default_values(
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

/// Gets all of the default configuration values from the database and
/// serializes them to YAML. If an output file is specified, the YAML is
/// written to that file. Otherwise, the YAML is printed to stdout.
///
/// Handler for the `mgmt-configs defaults render` command.
///
/// # Example
/// To render all of the default configuration values to stdout:
/// ```ignore
///     render_default_values(&pool, None).await?;
/// ```
///
/// To render all of the default configuration values to a file:
/// ```ignore
///     render_default_values(&pool, Some(PathBuf::from("defaults.yaml"))).await?;
/// ```
pub async fn render_default_values(
    pool: &Pool<MySql>,
    output_file: Option<PathBuf>,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    let all_default_cfgs = db::list_default_config_values(&mut tx, None, None).await?;
    let mut cv = config::ConfigValues::default();
    let mut section_options = config::SectionOptions::default();
    section_options.set_all(true)?;
    cv.set_section_options(section_options);
    cv.reset_sections()?;
    cv.cfg_set_keys(all_default_cfgs)?;

    if let Some(output_file) = output_file {
        let yaml = serde_yaml::to_string(&cv)?;
        std::fs::write(output_file, yaml)?;
    } else {
        let yaml = serde_yaml::to_string(&cv)?;
        println!("{}", yaml);
    }

    tx.commit().await?;

    Ok(())
}

/// Sets a configuration value for an environment in the database.
///
/// Handler for the `mgmt-configs values set` command.
///
/// # Example
/// ```ignore
///    set_value(&pool, "prod", "Agave", "Key", "12345", "string").await?;
/// ```
pub async fn set_value(
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

/// Gets a configuration value for an environment from the database and prints
/// it to stdout. If the value is not found in the environment, the default
/// value is printed instead. If the default value is not found, an error is
/// returned.
///
/// Handler for the `mgmt-configs values get` command.
///
/// # Example
/// ```ignore
///    get_value(&pool, "prod", "Agave", "Key").await?;
/// ```
pub async fn get_value(
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

/// Deletes a configuration value from an environment in the database and
/// prints a status message to stdout.
///
/// Handler for the `mgmt-configs values delete` command.
///
/// # Example
/// ```ignore
///   delete_value(&pool, "prod", "Agave", "Key").await?;
/// ```
pub async fn delete_value(
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

/// Lists configuration values for an environment from the database and prints
/// them to stdout.
///
/// Handler for the `mgmt-configs values list` command.
///
/// # Example
/// To list all of the configuration values for an environment:
/// ```ignore
///     list_values(&pool, Some("prod"), None, None).await?;
/// ```
///
/// To list all of the configuration values for a section in an environment:
/// ```ignore
///     list_values(&pool, Some("prod"), Some("Agave"), None).await?;
/// ```
///
/// To list all of the configuration values for a key in an environment:
/// ```ignore
///     list_values(&pool, Some("prod"), None, Some("Key")).await?;
/// ```
pub async fn list_values(
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

/// Gets all of the configuration values for an environment from the database
/// and serializes them to YAML. If an output file is specified, the YAML is
/// written to that file. Otherwise, the YAML is printed to stdout.
///
/// Handler  for the `mgmt-configs values render` command.
///
/// # Example
/// To render all of the configuration values for an environment to stdout:
/// ```ignore
///    render_values(&pool, "prod", &opts, None).await?;
/// ```
///
/// To render all of the configuration values for an environment to a file:
/// ```ignore
///   render_values(&pool, "prod", &opts, Some(PathBuf::from("prod.yaml"))).await?;
/// ```
pub async fn render_values(
    pool: &Pool<MySql>,
    environment: &str,
    opts: &config::SectionOptions,
    output_file: Option<PathBuf>,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let mut all_cfgs: Vec<Configuration> = Vec::new();
    let all_default_cfgs = db::list_default_config_values(&mut tx, None, None).await?;

    for default in all_default_cfgs
        .into_iter()
        .filter(|cfg| opts.include_section(&cfg.section.clone().unwrap_or_default()))
    {
        if let (Some(section), Some(key)) = (default.section.clone(), default.key.clone()) {
            let has_config_value = db::has_config_value(&mut tx, environment, &section, &key)
                .await
                .unwrap_or(false);

            if has_config_value {
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
    cv.set_section_options(opts.clone());
    cv.reset_sections()?;
    cv.cfg_set_keys(all_cfgs)?;

    tx.commit().await?;

    if let Some(output_file) = output_file {
        let yaml = serde_yaml::to_string(&cv)?;
        std::fs::write(output_file, yaml)?;
    } else {
        let yaml = serde_yaml::to_string(&cv)?;
        println!("{}", yaml);
    }

    Ok(())
}

/// Imports a YAML file into the database. The YAML file must be in the same
/// format as the output of the `mgmt-configs values render` command.
///
/// Handler for importing files.
///
/// # Example
/// ```ignore
///    import_yaml_file(&pool, PathBuf::from("prod.yaml"), "prod").await?;
/// ```
pub async fn import_yaml_file(
    pool: &Pool<MySql>,
    path: PathBuf,
    environment: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let file = std::fs::File::open(path)?;
    let cv: config::ConfigValues = serde_yaml::from_reader(file)?;
    let items: Vec<db::Configuration> = cv.into();
    for item in items.into_iter() {
        if let (Some(section), Some(key), Some(value), Some(value_type)) = (
            item.section.clone(),
            item.key.clone(),
            item.value.clone(),
            item.value_type.clone(),
        ) {
            println!("{}.{} = {}", section, key, value);
            let real_section: String;
            if section.is_empty() {
                real_section = "TopLevel".to_string();
            } else {
                real_section = section;
            }

            if db::has_default_config_value(&mut tx, &real_section, &key).await? {
                let cfg_id =
                    db::set_config_value(&mut tx, &real_section, &key, &value, &value_type).await?;
                let env_id = db::get_env_id(&mut tx, &environment)
                    .await?
                    .ok_or_else(|| {
                        anyhow::anyhow!("No environment found with name: {environment}")
                    })?;
                db::add_env_cfg_value(&mut tx, env_id, cfg_id).await?;
            } else {
                tx.rollback().await?;
                return Err(anyhow!(
                    "No default value found for section: {real_section}, key: {key}"
                ));
            }
        }
    }

    tx.commit().await?;
    Ok(())
}
