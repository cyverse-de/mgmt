use crate::{db, ops};
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use sqlx::{MySql, Pool};

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
