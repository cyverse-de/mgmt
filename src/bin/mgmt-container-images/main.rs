use anyhow::{anyhow, Result};
use clap::{arg, Command};
use serde::Deserialize;
use sqlx::mysql::{MySql, MySqlPoolOptions};
use sqlx::{Pool, Transaction};
use std::fs;

fn cli() -> Command {
    Command::new("mgmt-container-images")
        .about("Manages container images in the de_releases database")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .default_value("mysql://root@127.0.0.1:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(
            Command::new("upsert")
                .about("Inserts or updates a container image based on the image's name and tag")
                .args_conflicts_with_subcommands(true)
                .arg(arg!(-i --"image" <IMAGE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-s --"service" <SERVICE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f --dockerfile <DOCKERFILE>).value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            Command::new("insert")
                .args_conflicts_with_subcommands(true)
                .arg(arg!(-i --"image" <IMAGE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-s --"service" <SERVICE>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f --dockerfile <DOCKERFILE>).value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            Command::new("upsert-builds")
                .args_conflicts_with_subcommands(true)
                .arg(arg!(-b --"builds-dir" <DIR>).value_parser(clap::value_parser!(String)))
                .arg(arg!(-f - -"force-insert").value_parser(clap::value_parser!(bool))),
        )
        .subcommand(
            Command::new("delete")
                .args_conflicts_with_subcommands(true)
                .arg(arg!(-i --"id" <ID>).value_parser(clap::value_parser!(i32))),
        )
        .subcommand(Command::new("list").args_conflicts_with_subcommands(true))
}

struct ContainerImageParts {
    name: String,
    tag: String,
    digest: String,
}

fn parse_container_image(image: &str) -> Result<ContainerImageParts> {
    let mut parts = image.split('@');
    let name_and_tag = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("image name not found"))?;
    let digest = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("digest not found"))?;

    let mut parts = name_and_tag.split(':');
    let name = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("image name not found"))?;
    let tag = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("tag not found"))?;

    Ok(ContainerImageParts {
        name: name.to_string(),
        tag: tag.to_string(),
        digest: digest.to_string(),
    })
}

async fn get_service_repo_id(tx: &mut Transaction<'_, MySql>, service: &str) -> Result<i32> {
    let repo_id = sqlx::query!(
        r#"
        SELECT repo_id FROM services WHERE name = (?)
        "#,
        service,
    )
    .fetch_one(tx)
    .await?
    .repo_id
    .ok_or_else(|| anyhow::anyhow!("repo_id not found"))?;

    Ok(repo_id)
}

async fn service_exists(tx: &mut Transaction<'_, MySql>, service: &str) -> Result<bool> {
    let services = sqlx::query!(
        r#"
        SELECT COUNT(*) AS count FROM services WHERE name = (?)
        "#,
        service,
    )
    .fetch_one(tx)
    .await?;

    Ok(services.count.unwrap_or(0) > 0)
}

async fn image_exists(
    tx: &mut Transaction<'_, MySql>,
    image: &ContainerImageParts,
) -> Result<bool> {
    let images = sqlx::query!(
        r#"
        SELECT COUNT(*) AS count 
        FROM container_images 
        WHERE name = (?)
        AND tag = (?)
        "#,
        image.name,
        image.tag,
    )
    .fetch_one(tx)
    .await?;

    Ok(images.count.unwrap_or(0) > 0)
}

async fn update_image(
    tx: &mut Transaction<'_, MySql>,
    repo_id: i32,
    dockerfile: &str,
    container_image: &ContainerImageParts,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE container_images 
            SET 
                dockerfile = (?),
                repo_id = (?),
                digest = (?)
            WHERE name = (?)
            AND tag = (?)
        "#,
        dockerfile,
        repo_id,
        container_image.digest,
        container_image.name,
        container_image.tag,
    )
    .execute(tx)
    .await?;

    Ok(())
}

async fn insert_image(
    tx: &mut Transaction<'_, MySql>,
    repo_id: i32,
    dockerfile: &str,
    container_image: &ContainerImageParts,
) -> Result<u64> {
    let image_id = sqlx::query!(
        r#"
            INSERT INTO container_images 
                (name, tag, digest, repo_id, dockerfile)
            VALUES 
                (?, ?, ?, ?, ?)
        "#,
        container_image.name,
        container_image.tag,
        container_image.digest,
        repo_id,
        dockerfile,
    )
    .execute(tx)
    .await?
    .last_insert_id();

    Ok(image_id)
}

async fn list_images(pool: &Pool<MySql>) -> Result<()> {
    let images = sqlx::query!(
        r#"
        SELECT 
            ci.id, ci.name, ci.tag, ci.digest, ci.dockerfile, s.name as service_name
        FROM 
            container_images ci
        INNER JOIN
            services s
        ON
            ci.repo_id = s.repo_id
        "#,
    )
    .fetch_all(pool)
    .await?;

    println!("Images:");
    for image in images {
        println!(
            "  id: {}, name: {}, tag: {}, digest: {}, dockerfile: {}, service: {}",
            image.id.ok_or_else(|| anyhow!("missing id"))?,
            image.name.ok_or_else(|| anyhow!("missing name"))?,
            image.tag.ok_or_else(|| anyhow!("missing tag"))?,
            image.digest.ok_or_else(|| anyhow!("missing digest"))?,
            image
                .dockerfile
                .ok_or_else(|| anyhow!("missing dockerfile"))?,
            image
                .service_name
                .ok_or_else(|| anyhow!("missing service_name"))?,
        );
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct BuildImage {
    tag: String,
}
#[derive(Debug, Deserialize)]
struct BuildsData {
    builds: Vec<BuildImage>,
}

async fn upsert_builds(pool: &Pool<MySql>, builds_dir: &str, force_insert: bool) -> Result<()> {
    let mut build_dirs = fs::read_dir(builds_dir)?;
    let mut tx = pool.begin().await?;

    while let Some(entry_result) = build_dirs.next() {
        let entry = entry_result?;
        let entry_os_name = entry.file_name();
        let entry_name = entry_os_name
            .to_str()
            .ok_or_else(|| anyhow!("invalid entry name"))?;

        if !entry_name.ends_with(".json") {
            continue;
        }

        let service_name = entry_name.trim_end_matches(".json");
        if service_exists(&mut tx, service_name).await? {
            println!("Inserting image for service {}", service_name);
        } else {
            println!("Service {} does not exist", service_name);
            continue;
        }

        let data: String = fs::read_to_string(entry.path())?;

        let parsed: BuildsData = serde_json::from_str(&data)?;
        if parsed.builds.is_empty() {
            continue;
        }

        let build_image = &parsed.builds[0];
        let image = build_image.tag.clone();

        println!("  image: {}", image);

        let repo_id = get_service_repo_id(&mut tx, service_name).await?;
        let container_image = parse_container_image(&image)?;

        if !image_exists(&mut tx, &container_image).await? || force_insert {
            let last_id =
                insert_image(&mut tx, repo_id.clone(), "Dockerfile", &container_image).await?;
            println!(
                "  inserted image for service {} with id {}",
                service_name, last_id
            );
        } else {
            update_image(&mut tx, repo_id.clone(), "Dockerfile", &container_image).await?;
            println!("  updated image for service {}", service_name);
        }
    }

    tx.commit().await?;

    Ok(())
}

async fn upsert_image(
    pool: &Pool<MySql>,
    image: &str,
    service_name: &str,
    dockerfile: &str,
) -> Result<()> {
    let mut tx = pool.begin().await?;

    let container_image = parse_container_image(image)?;
    if !service_exists(&mut tx, &service_name).await? {
        println!("Service {} does not exist", service_name);
        return Ok(());
    }
    let repo_id = get_service_repo_id(&mut tx, &service_name).await?;

    if !image_exists(&mut tx, &container_image).await? {
        let last_id = insert_image(&mut tx, repo_id, dockerfile, &container_image).await?;
        println!("Inserted image with id {}", last_id);
    } else {
        update_image(&mut tx, repo_id, dockerfile, &container_image).await?;
        println!("Updated image");
    }

    tx.commit().await?;

    Ok(())
}

async fn delete_image(pool: &Pool<MySql>, id: &i32) -> Result<()> {
    let mut tx = pool.begin().await?;
    println!("Deleting image with id {}", id);

    sqlx::query!(
        r#"
        DELETE FROM container_images WHERE id = (?)
        "#,
        *id,
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
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
        Some(("insert", sub_m)) => {
            let image = sub_m.get_one::<String>("image").ok_or_else(|| {
                anyhow!("No image specified. Use --image <image> to specify an image to insert.")
            })?;
            let dockerfile = sub_m.get_one::<String>("dockerfile").ok_or_else(|| {
                anyhow!("No dockerfile specified. Use --dockerfile <dockerfile> to specify a dockerfile to insert.")
            })?;
            let service = sub_m.get_one::<String>("service").ok_or_else(|| {
                anyhow!(
                    "No service specified. Use --service <service> to specify a service to insert."
                )
            })?;
            let mut tx = pool.begin().await?;
            let repo_id = get_service_repo_id(&mut tx, &service).await?;
            let container_image = parse_container_image(&image)?;
            insert_image(&mut tx, repo_id, &dockerfile, &container_image).await?;
            tx.commit().await?;
        }
        Some(("upsert", sub_m)) => {
            let image = sub_m.get_one::<String>("image").ok_or_else(|| {
                anyhow!("No image specified. Use --image <image> to specify an image to insert.")
            })?;
            let dockerfile = sub_m.get_one::<String>("dockerfile").ok_or_else(|| {
                anyhow!("No dockerfile specified. Use --dockerfile <dockerfile> to specify a dockerfile to insert.")
            })?;
            let service = sub_m.get_one::<String>("service").ok_or_else(|| {
                anyhow!(
                    "No service specified. Use --service <service> to specify a service to insert."
                )
            })?;
            upsert_image(&pool, &image, &service, &dockerfile).await?;
        }
        Some(("upsert-builds", sub_m)) => {
            let builds_dir = sub_m.get_one::<String>("builds-dir").ok_or_else(|| {
                anyhow!("No builds-dir specified. Use --builds-dir <builds-dir> to specify a builds-dir to insert.")
            })?;
            let force_insert = sub_m.get_flag("force-insert");
            upsert_builds(&pool, &builds_dir, force_insert).await?;
        }
        Some(("delete", sub_m)) => {
            let id = sub_m.get_one::<i32>("id").ok_or_else(|| {
                anyhow!("No id specified. Use --id <id> to specify an id to delete.")
            })?;
            delete_image(&pool, id).await?;
        }
        Some(("list", _)) => {
            list_images(&pool).await?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
