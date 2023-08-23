use crate::config_values::config;
use crate::db::{self, Configuration, LoadFromConfiguration};
use crate::git;
use anyhow::{anyhow, Context};
use sqlx::{MySql, Pool};
use std::path::{Path, PathBuf};

pub fn create_project_dir(project_name: &str) -> anyhow::Result<PathBuf> {
    let project_dir = Path::new(project_name);
    if project_dir.exists() {
        anyhow::bail!("Project directory already exists");
    }
    std::fs::create_dir(project_dir)?;
    Ok(project_dir.to_path_buf())
}

pub fn clone_repos(project_dir: &Path, repos: &[url::Url]) -> anyhow::Result<()> {
    for repo_url in repos {
        let repo_name = repo_url
            .path_segments()
            .context("missing path for repo url")?
            .last()
            .context("empty path for repo url")?;

        let repo_dir = project_dir.join(repo_name);
        if repo_dir.exists() {
            continue;
        }

        git::clone(
            repo_url.as_str(),
            &repo_dir
                .to_str()
                .context("failed to create repo dir string")?,
        )?;
    }
    Ok(())
}

/**
 * Handler for the `mgmt-configs env populate` command.
 */
pub async fn populate_env(pool: &Pool<MySql>) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let mut env_config = config::ConfigValues::default();
    env_config.ask_for_info(&mut tx).await?;
    tx.commit().await?;
    Ok(())
}

/**
 * Handler for the `mgmt-configs sections add` command.
 */
pub async fn add_section(pool: &Pool<MySql>, section: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    db::add_section(&mut tx, &section).await?;
    tx.commit().await?;
    Ok(())
}
/**
 * Handler for the `mgmt-configs sections delete` command.
 */
pub async fn delete_section(pool: &Pool<MySql>, section: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    db::delete_section(&mut tx, &section).await?;
    tx.commit().await?;
    Ok(())
}

/**
 * Handler for the `mgmt-configs sections list` command.
 */
pub async fn list_sections(pool: &Pool<MySql>) -> anyhow::Result<()> {
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

/**
 * Handler for the `mgmt-configs defaults get` command.
 */
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

/**
 * Handler for the `mgmt-configs defaults delete` command.
 */
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

/**
 * Handler for the `mgmt-configs defaults list` command.
 */
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

/**
 * Handler for the `mgmt-configs defaults render` command.
 */
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

/**
 * Handler for the `mgmt-configs values set` command.
 */
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

/**
 * Handler for the `mgmt-configs values get` command.
 */
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

/**
 * Handler for the `mgmt-configs values delete` command.
 */
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

/**
 * Handler for the `mgmt-configs values list` command.
 */
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

/**
 * Handler  for the `mgmt-configs values render` command.
 */
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

/**
 * Handler for importing files.
 */
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
