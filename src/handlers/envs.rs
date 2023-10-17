use crate::{db, ops};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use sqlx::{MySql, Pool};

async fn env_create(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let name = sub_m
        .get_one::<String>("name")
        .ok_or_else(|| anyhow!("No name specified. Use --name <name> to specify a name."))?;

    let namespace = sub_m.get_one::<String>("namespace").ok_or_else(|| {
        anyhow!("No namespace specified. Use --namespace <namespace> to specify a namespace.")
    })?;

    let from = sub_m.get_one::<String>("from").ok_or_else( || {
        anyhow!("No environment specified for --from. Use --from <environment> to specify a basis environment.")
    })?;

    let mut tx = pool.begin().await?;
    db::upsert_environment(&mut tx, &name, &namespace).await?;
    println!("Created environment: {}", name);

    println!("Setting up environment...");

    // Get the list of services available in the --from environment.
    let from_services = db::list_services(&mut tx, &from).await?;
    for from_service in from_services {
        let from_service_name = from_service.name.context("no name set for service")?;
        db::add_service_to_env(&mut tx, &name, &from_service_name).await?;
        println!(
            "Added {} service to {} environment",
            from_service_name, name
        );

        let service_template_ids =
            db::list_service_templates(&mut tx, &from, &from_service_name).await?;
        for service_template_id in service_template_ids {
            db::copy_service_template_to_env(
                &mut tx,
                &name,
                &from,
                &from_service_name,
                service_template_id,
            )
            .await?;
        }
        println!(
            "Copied config templates for {} service to {} environment",
            from_service_name, name
        );
    }

    tx.commit().await?;

    println!("Environment setup complete.");

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

async fn env_services_add(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let env = sub_m.get_one::<String>("env").ok_or_else(|| {
        anyhow!("No environment specified. Use --env <env> to specify an environment.")
    })?;

    let services = sub_m
        .get_many::<String>("service")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

    let mut tx = pool.begin().await?;
    for svc in services {
        db::add_service_to_env(&mut tx, &env, &svc).await?;
    }
    tx.commit().await?;

    Ok(())
}

async fn env_services_remove(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let env = sub_m.get_one::<String>("env").ok_or_else(|| {
        anyhow!("No environment specified. Use --env <env> to specify an environment.")
    })?;

    let services = sub_m
        .get_many::<String>("service")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

    let mut tx = pool.begin().await?;
    for svc in services {
        db::remove_service_from_env(&mut tx, &env, &svc).await?;
    }
    tx.commit().await?;

    Ok(())
}

async fn env_services_list(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let env = sub_m.get_one::<String>("env").ok_or_else(|| {
        anyhow!("No environment specified. Use --env <env> to specify an environment.")
    })?;

    let mut tx = pool.begin().await?;
    let services = db::get_services(&mut tx, &env).await?;
    tx.commit().await?;

    for svc in services {
        if let Some(name) = svc.name {
            println!("{}", name);
        }
    }

    Ok(())
}

async fn env_services_handler(pool: &Pool<MySql>, sub_m: &ArgMatches) -> Result<()> {
    let services_cmd = sub_m
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("bad command"))?;

    match services_cmd {
        ("add", sub_m) => env_services_add(&pool, &sub_m).await,
        ("delete", sub_m) => env_services_remove(&pool, &sub_m).await,
        ("list", sub_m) => env_services_list(&pool, &sub_m).await,
        (name, _) => unreachable!("Bad subcommand: {name}"),
    }
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
        ("service", sub_m) => env_services_handler(&pool, &sub_m).await,
        (name, _) => unreachable!("Bad subcommand: {name}"),
    }
}
