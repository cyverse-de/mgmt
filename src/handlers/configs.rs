use crate::{
    config_values::config::{self, ConfigValues},
    db, ops,
};
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use sqlx::{Pool, Postgres};
use std::path::PathBuf;

async fn section_add(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
        anyhow!("No section specified. Use --section <section> to specify a section.")
    })?;

    ops::add_section(&pool, &section).await?;

    Ok(())
}

async fn section_delete(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
        anyhow!("No section specified. Use --section <section> to specify a section.")
    })?;

    ops::delete_section(&pool, &section).await?;

    Ok(())
}

pub async fn sections(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let section_cmd = sub_m
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("bad command"))?;

    match section_cmd {
        ("add", sub_m) => section_add(&pool, &sub_m).await,
        ("delete", sub_m) => section_delete(&pool, &sub_m).await,
        ("list", _) => ops::list_sections(&pool).await,
        (name, _) => unreachable!("Bad subcommand: {name}"),
    }
}

pub async fn defaults_set(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
        anyhow!("No section specified. Use --section <section> to specify a section.")
    })?;

    let key = sub_m
        .get_one::<String>("key")
        .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

    let value = sub_m
        .get_one::<String>("value")
        .ok_or_else(|| anyhow!("No value specified. Use --value <value> to specify a value."))?;

    let value_type = sub_m
        .get_one::<String>("type")
        .ok_or_else(|| anyhow!("No type specified. Use --type <type> to specify a type."))?;

    ops::set_default_value(&pool, &section, &key, &value, &value_type).await?;

    Ok(())
}

async fn defaults_get(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
        anyhow!("No section specified. Use --section <section> to specify a section.")
    })?;

    let key = sub_m
        .get_one::<String>("key")
        .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

    ops::get_default_value(&pool, &section, &key).await?;

    Ok(())
}

async fn defaults_delete(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
        anyhow!("No section specified. Use --section <section> to specify a section.")
    })?;

    let key = sub_m
        .get_one::<String>("key")
        .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

    ops::delete_default_value(&pool, &section, &key).await?;

    Ok(())
}

async fn defaults_list(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
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

async fn defaults_render(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let output_file = sub_m.get_one::<PathBuf>("file").cloned();
    ops::render_default_values(&pool, output_file).await?;

    Ok(())
}

pub async fn defaults(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let defaults_cmd = sub_m
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("bad command"))?;

    match defaults_cmd {
        ("set", sub_m) => defaults_set(&pool, &sub_m).await,
        ("get", sub_m) => defaults_get(&pool, &sub_m).await,
        ("delete", sub_m) => defaults_delete(&pool, &sub_m).await,
        ("list", sub_m) => defaults_list(&pool, &sub_m).await,
        ("render", sub_m) => defaults_render(&pool, &sub_m).await,
        (name, _) => unreachable!("Bad subcommand: {name}"),
    }
}

async fn values_set(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
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

    let value = sub_m
        .get_one::<String>("value")
        .ok_or_else(|| anyhow!("No value specified. Use --value <value> to specify a value."))?;

    let value_type = sub_m
        .get_one::<String>("type")
        .ok_or_else(|| anyhow!("No type specified. Use --type <type> to specify a type."))?;

    ops::set_value(&pool, &environment, &section, &key, &value, &value_type).await?;

    Ok(())
}

async fn values_get(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
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

async fn values_delete(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
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

async fn values_list(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
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

async fn values_render(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
        anyhow!(
            "No environment specified. Use --environment <environment> to specify an environment."
        )
    })?;

    let output_file = match sub_m.get_one::<PathBuf>("file") {
        Some(file) => Some(file.clone()),
        None => None,
    };

    let opts = config::SectionOptions::new_from_db(&pool, &environment).await?;
    ops::render_values(&pool, &environment, &opts, output_file).await?;

    Ok(())
}

async fn values_import(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let path = sub_m
        .get_one::<PathBuf>("file")
        .ok_or_else(|| anyhow!("No file specified. Use --file <file> to specify a file."))?;

    let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
        anyhow!(
            "No environment specified. Use --environment <environment> to specify an environment."
        )
    })?;

    ops::import_yaml_file(&pool, path.to_path_buf(), environment).await?;

    println!(
        "Imported values from {} for the {} environment.",
        path.display(),
        environment
    );

    // Now get all of the newly import values from the database.
    let mut tx = pool.begin().await?;
    let imported_cfgs: ConfigValues =
        db::list_config_values(&mut tx, Some(&environment), None, None)
            .await?
            .into();
    let new_ops = imported_cfgs.generate_section_options();
    db::upsert_feature_flags(&mut tx, &environment, &new_ops.into()).await?;
    tx.commit().await?;

    println!("Set up feature flags for the {} environment.", environment);

    Ok(())
}

pub async fn values(pool: &Pool<Postgres>, sub_m: &ArgMatches) -> Result<()> {
    let values_cmd = sub_m
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("bad command"))?;

    match values_cmd {
        ("set", sub_m) => values_set(&pool, &sub_m).await,
        ("get", sub_m) => values_get(&pool, &sub_m).await,
        ("delete", sub_m) => values_delete(&pool, &sub_m).await,
        ("list", sub_m) => values_list(&pool, &sub_m).await,
        ("render", sub_m) => values_render(&pool, &sub_m).await,
        ("import", sub_m) => values_import(&pool, &sub_m).await,
        (name, _) => unreachable!("Bad subcommand: {name}"),
    }
}
