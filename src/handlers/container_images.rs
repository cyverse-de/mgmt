use anyhow::{anyhow, Result};
use serde::Deserialize;
use sqlx::mysql::MySql;
use sqlx::{Pool, Transaction};
use std::fs;
use std::path::PathBuf;

pub struct ContainerImageParts {
    name: String,
    tag: String,
    digest: String,
}

/*
 * Parses a container image string into its name, tag, and digest parts.
 */
pub fn parse_container_image(image: &str) -> Result<ContainerImageParts> {
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

/**
 * Returns the repo_id for a given service.
 */
pub async fn get_service_repo_id(tx: &mut Transaction<'_, MySql>, service: &str) -> Result<i32> {
    let repo_id = sqlx::query!(
        r#"
        SELECT repo_id FROM services WHERE name = (?)
        "#,
        service,
    )
    .fetch_one(&mut **tx)
    .await?
    .repo_id
    .ok_or_else(|| anyhow::anyhow!("repo_id not found"))?;

    Ok(repo_id)
}

/**
 * Returns true if the service exists in the database.
 */
async fn service_exists(tx: &mut Transaction<'_, MySql>, service: &str) -> Result<bool> {
    let services = sqlx::query!(
        r#"
        SELECT COUNT(*) AS count FROM services WHERE name = (?)
        "#,
        service,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(services.count.unwrap_or(0) > 0)
}

/**
 * Returns true if the container image exists in the database.
 */
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
    .fetch_one(&mut **tx)
    .await?;

    Ok(images.count.unwrap_or(0) > 0)
}

/**
 * Updates an image in the database.
 */
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
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/**
 * Inserts an image into the database.
 */
pub async fn insert_image(
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
    .execute(&mut **tx)
    .await?
    .last_insert_id();

    Ok(image_id)
}

/**
 * Lists the images in the database.
 */
pub async fn list_images(pool: &Pool<MySql>) -> Result<()> {
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

/**
 * Contains the tag portion of a JSON file in a builds directory.
 */
#[derive(Debug, Deserialize)]
struct BuildImage {
    tag: String,
}

/**
 * Contains the contents of a JSON file from a builds directory.
 */
#[derive(Debug, Deserialize)]
struct BuildsData {
    builds: Vec<BuildImage>,
}

#[derive(thiserror::Error, Debug)]
enum BuildFileError {
    #[error("Empty file")]
    EmptyFile,
}

pub async fn upsert_build(
    tx: &mut Transaction<'_, MySql>,
    builds_file: &PathBuf,
    service_name: &str,
    force_insert: bool,
) -> Result<()> {
    let data: String = fs::read_to_string(builds_file)?;

    let parsed: BuildsData = serde_json::from_str(&data)?;
    if parsed.builds.is_empty() {
        return Err(BuildFileError::EmptyFile.into());
    }

    let build_image = &parsed.builds[0];
    let image = build_image.tag.clone();

    println!("  image: {}", image);

    let repo_id = get_service_repo_id(tx, service_name).await?;
    let container_image = parse_container_image(&image)?;

    if !image_exists(tx, &container_image).await? || force_insert {
        let last_id = insert_image(tx, repo_id.clone(), "Dockerfile", &container_image).await?;
        println!(
            "  inserted image for service {} with id {}",
            service_name, last_id
        );
    } else {
        update_image(tx, repo_id.clone(), "Dockerfile", &container_image).await?;
        println!("  updated image for service {}", service_name);
    }

    Ok(())
}

/**
 * Upserts the container images in the database based on the contents of the JSON files
 * in the given directory. Optionally can be forced to insert the images.
 */
pub async fn upsert_builds(pool: &Pool<MySql>, builds_dir: &str, force_insert: bool) -> Result<()> {
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

        let result = upsert_build(&mut tx, &entry.path(), service_name, force_insert).await;
        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Error upserting build: {}", e);
                continue;
            }
        }

        let data: String = fs::read_to_string(entry.path())?;

        let parsed: BuildsData = serde_json::from_str(&data)?;
        if parsed.builds.is_empty() {
            continue;
        }
    }

    tx.commit().await?;

    Ok(())
}

/**
 * Upserts a single image to the database.
 */
pub async fn upsert_image(
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

/**
 * Deletes an image from the database.
 */
pub async fn delete_image(pool: &Pool<MySql>, id: &i32) -> Result<()> {
    let mut tx = pool.begin().await?;
    println!("Deleting image with id {}", id);

    sqlx::query!(
        r#"
        DELETE FROM container_images WHERE id = (?)
        "#,
        *id,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
