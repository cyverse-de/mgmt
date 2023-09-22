use crate::{config_values::config, db, ops};
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use sqlx::{MySql, Pool};
use std::path::PathBuf;

async fn env_create(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let name = sub_m
        .get_one::<String>("name")
        .ok_or_else(|| anyhow!("No name specified. Use --name <name> to specify a name."))?;

    let namespace = sub_m.get_one::<String>("namespace").ok_or_else(|| {
        anyhow!("No namespace specified. Use --namespace <namespace> to specify a namespace.")
    })?;

    let mut tx = pool.begin().await?;
    db::upsert_environment(&mut tx, &name, &namespace).await?;
    tx.commit().await?;

    println!("Created environment: {}", name);

    Ok(())
}

async fn env_list(pool: &Pool<MySql>) -> Result<()> {
    let mut tx = pool.begin().await?;
    let envs = db::list_envs(&mut tx).await?;
    tx.commit().await?;
    for env in envs {
        println!("{}", env);
    }

    Ok(())
}

async fn env_delete(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let name = sub_m
        .get_one::<String>("name")
        .ok_or_else(|| anyhow!("No name specified. Use --name <name> to specify a name."))?;

    let mut tx = pool.begin().await?;
    db::delete_env(&mut tx, &name).await?;
    tx.commit().await?;

    println!("Deleted environment: {}", name);

    Ok(())
}

pub async fn env(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let create_cmd = sub_m
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("bad command"))?;

    match create_cmd {
        ("populate", _) => Ok(ops::populate_env(&pool).await?),
        ("create", sub_m) => env_create(&pool, &sub_m).await,
        ("list", _) => env_list(&pool).await,
        ("delete", sub_m) => env_delete(&pool, &sub_m).await,
        (name, _) => unreachable!("Bad subcommand: {name}"),
    }
}

async fn section_add(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
        anyhow!("No section specified. Use --section <section> to specify a section.")
    })?;

    ops::add_section(&pool, &section).await?;

    Ok(())
}

async fn section_delete(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
        anyhow!("No section specified. Use --section <section> to specify a section.")
    })?;

    ops::delete_section(&pool, &section).await?;

    Ok(())
}

pub async fn sections(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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

pub async fn defaults_set(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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

async fn defaults_get(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
        anyhow!("No section specified. Use --section <section> to specify a section.")
    })?;

    let key = sub_m
        .get_one::<String>("key")
        .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

    ops::get_default_value(&pool, &section, &key).await?;

    Ok(())
}

async fn defaults_delete(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let section = sub_m.get_one::<String>("section").ok_or_else(|| {
        anyhow!("No section specified. Use --section <section> to specify a section.")
    })?;

    let key = sub_m
        .get_one::<String>("key")
        .ok_or_else(|| anyhow!("No key specified. Use --key <key> to specify a key."))?;

    ops::delete_default_value(&pool, &section, &key).await?;

    Ok(())
}

async fn defaults_list(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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

async fn defaults_render(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let output_file = sub_m.get_one::<PathBuf>("file").cloned();
    ops::render_default_values(&pool, output_file).await?;

    Ok(())
}

pub async fn defaults(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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

async fn values_set(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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

async fn values_get(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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

async fn values_delete(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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

async fn values_list(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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

async fn values_render(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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

async fn values_import(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let path = sub_m
        .get_one::<PathBuf>("file")
        .ok_or_else(|| anyhow!("No file specified. Use --file <file> to specify a file."))?;

    let environment = sub_m.get_one::<String>("environment").ok_or_else(|| {
        anyhow!(
            "No environment specified. Use --environment <environment> to specify an environment."
        )
    })?;

    ops::import_yaml_file(&pool, path.to_path_buf(), environment).await?;

    Ok(())
}

pub async fn values(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
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
