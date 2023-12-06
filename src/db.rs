//! # Database Access
//!
//! This module contains all the database access code for the application.
use anyhow::Context;
use sqlx::{Postgres, Row, Transaction};
use std::path::PathBuf;

/// Represents a single configuration value as stored in the database.
#[derive(
    sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct ConfigurationValue {
    pub id: i32,
    pub section: String,
    pub key: String,
    pub value: String,
    pub value_type: String,
}

/// The trait that all domain objects need to implement so they can load
/// their state from configuration values retrieved from the database.
pub trait LoadFromDatabase {
    /// Returns the configuration section name for the domain object. Nested configuration blocks
    /// should return the name of the outermost block. For example: `DashboardAggregator` instead of
    /// `DashboardAggregator.Website` or `Website`.
    fn get_section(&self) -> String;

    /// Sets a field on the domain object to the configuration value retrieved from the database.
    fn cfg_set_key(&mut self, cfg: &ConfigurationValue) -> anyhow::Result<()>;

    /// Sets all the fields on the domain object to the configuration values retrieved from the database.
    /// The default implementation is usually sufficient for the domain objects, but a couple may need
    /// custom logic.
    fn cfg_set_keys(&mut self, cfgs: Vec<ConfigurationValue>) -> anyhow::Result<()> {
        cfgs.into_iter()
            .try_for_each(|cfg| self.cfg_set_key(&cfg))?;
        Ok(())
    }
}

/// Updates or inserts an environment into the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::upsert_environment(&mut tx, "dev", "dev").await?;
/// tx.commit().await?;
/// ```
pub async fn upsert_environment(
    tx: &mut Transaction<'_, Postgres>,
    environment: &str,
    namespace: &str,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
                INSERT INTO environments 
                    (name, namespace) 
                VALUES 
                    ($1, $2) 
                ON CONFLICT (name) DO UPDATE SET name = $1
                RETURNING id
            "#,
        environment,
        namespace
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Returns a listing of the environments stored in the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::list_envs(&mut tx).await?;
/// tx.commit().await?;
///
/// for env in result {
///     println!("{}", env);
/// }
/// ```
pub async fn list_envs(tx: &mut Transaction<'_, Postgres>) -> anyhow::Result<Vec<String>> {
    let envs = sqlx::query!(
        r#"
            SELECT name FROM environments
        "#
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(envs.into_iter().map(|e| e.name).collect())
}

/// Deletes an environment from the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::delete_env(&mut tx, "dev").await?;
/// tx.commit().await?;
/// ```
pub async fn delete_env(
    tx: &mut Transaction<'_, Postgres>,
    environment: &str,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
                DELETE FROM environments WHERE name = $1 RETURNING id
        "#,
        environment
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Returns the primary key of the environment from the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::get_env_id(&mut tx, "dev").await?;
/// tx.commit().await?;
/// ```
pub async fn get_env_id(
    tx: &mut Transaction<'_, Postgres>,
    environment: &str,
) -> anyhow::Result<i32> {
    let env_id = sqlx::query!(
        r#"
                SELECT id FROM environments WHERE name = $1
        "#,
        environment
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(env_id.id)
}

/// Returns a listing of the url and name of the repositories stored in the
/// database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::get_repos(&mut tx).await?;
/// tx.commit().await?;
/// ```
pub async fn get_repos(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<(String, String)>> {
    let repos = sqlx::query!(
        r#"
            SELECT url, name FROM repos
        "#
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(repos
        .into_iter()
        .filter_map(|r| {
            let u = r.url;
            let n = r.name;
            if !u.is_empty() && !n.is_empty() {
                Some((u, n))
            } else {
                None
            }
        })
        .collect())
}

/// Adds a new configuration section to the database. Returns the primary key
/// of the new section.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::add_section(&mut tx, "DashboardAggregator").await?;
/// tx.commit().await?;
/// ```
pub async fn add_section(tx: &mut Transaction<'_, Postgres>, section: &str) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
                INSERT INTO config_sections (name) VALUES ($1) ON CONFLICT (name) DO NOTHING RETURNING id
        "#,
        section
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Returns whether the section exists in the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::has_section(&mut tx, "DashboardAggregator").await?;
/// tx.commit().await?;
///
/// assert!(result);
/// ```
pub async fn has_section(
    tx: &mut Transaction<'_, Postgres>,
    section: &str,
) -> anyhow::Result<bool> {
    let section = sqlx::query!(
        r#"
                SELECT id FROM config_sections WHERE name = $1
        "#,
        section
    )
    .fetch_optional(&mut **tx)
    .await?;

    Ok(section.is_some())
}

/// Deletes a configuration section from the database. Returns the primary key
/// of the deleted section.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::delete_section(&mut tx, "DashboardAggregator").await?;
/// tx.commit().await?;
/// ```
pub async fn delete_section(
    tx: &mut Transaction<'_, Postgres>,
    section: &str,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
                DELETE FROM config_sections WHERE name = $1 RETURNING id
        "#,
        section
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Returns a listing of the configuration sections stored in the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::list_sections(&mut tx).await?;
/// tx.commit().await?;
///
/// for section in result {
///    println!("{}", section);
/// }
/// ```
pub async fn list_sections(tx: &mut Transaction<'_, Postgres>) -> anyhow::Result<Vec<String>> {
    let sections = sqlx::query!(
        r#"
                SELECT name FROM config_sections
        "#
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(sections.into_iter().map(|s| s.name).collect())
}

/// Returns a default configuration value from the database based on the
/// section and key.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::get_default_config_value(&mut tx, "DashboardAggregator", "Website.URL").await?;
/// tx.commit().await?;
/// ```
pub async fn get_default_config_value(
    tx: &mut Transaction<'_, Postgres>,
    section: &str,
    key: &str,
) -> anyhow::Result<ConfigurationValue> {
    let default = sqlx::query_as!(
        ConfigurationValue,
        r#"
                SELECT
                    config_defaults.id AS id,
                    config_sections.name AS section,
                    config_defaults.cfg_key AS key, 
                    config_defaults.cfg_value AS value,
                    config_value_types.name AS value_type
                FROM config_defaults
                INNER JOIN config_sections ON config_defaults.section_id = config_sections.id
                INNER JOIN config_value_types ON config_defaults.value_type_id = config_value_types.id
                WHERE config_sections.name = $1 AND config_defaults.cfg_key = $2
        "#,
        section,
        key
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(default)
}

/// Updates or adds a default configuration value to the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::set_default_config_value(&mut tx, "DashboardAggregator", "Website.URL", "https://example.com", "string").await?;
/// tx.commit().await?;
/// ```
pub async fn set_default_config_value(
    tx: &mut Transaction<'_, Postgres>,
    section: &str,
    key: &str,
    value: &str,
    value_type: &str,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
                INSERT INTO config_defaults (section_id, cfg_key, cfg_value, value_type_id) VALUES (
                    (SELECT id FROM config_sections WHERE name = $1),
                    $2,
                    $3,
                    (SELECT id FROM config_value_types WHERE name = $4)
                ) ON CONFLICT (id) DO UPDATE SET cfg_value = $3
                RETURNING id
            "#,
        section,
        key,
        value,
        value_type
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Returns whether a default configuration value exists in the database
/// associated with the given section and key.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::has_default_config_value(&mut tx, "DashboardAggregator", "Website.URL").await?;
/// tx.commit().await?;
/// ```
pub async fn has_default_config_value(
    tx: &mut Transaction<'_, Postgres>,
    section: &str,
    key: &str,
) -> anyhow::Result<bool> {
    let default = sqlx::query!(
        r#"
                SELECT config_defaults.id FROM config_defaults
                INNER JOIN config_sections ON config_defaults.section_id = config_sections.id
                WHERE config_sections.name = $1 AND config_defaults.cfg_key = $2
        "#,
        section,
        key
    )
    .fetch_optional(&mut **tx)
    .await?;

    Ok(default.is_some())
}

/// Delete a default configuration value from the database based on the
/// section and key.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::delete_default_config_value(&mut tx, "DashboardAggregator", "Website.URL").await?;
/// tx.commit().await?;
/// ```
pub async fn delete_default_config_value(
    tx: &mut Transaction<'_, Postgres>,
    section: &str,
    key: &str,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
                DELETE FROM config_defaults WHERE config_defaults.section_id = (
                    SELECT id FROM config_sections WHERE name = $1
                ) AND config_defaults.cfg_key = $2
                RETURNING id
            "#,
        section,
        key
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Returns a listing of the default configuration values stored in the
/// database based on the section and key. If no section or key is provided,
/// all default configuration values are returned. If a section is provided
/// but no key, all default configuration values for that section are returned.
/// If a section and key are provided, only the default configuration value
/// for that section and key is returned. If no section but a key is provided,
/// all default configuration values for that key are returned.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::list_default_config_values(&mut tx, None, None).await?;
/// tx.commit().await?;
///
/// for cfg in result {
///    println!("{:?}", cfg);
/// }
/// ```
pub async fn list_default_config_values(
    tx: &mut Transaction<'_, Postgres>,
    section: Option<&str>,
    key: Option<&str>,
) -> anyhow::Result<Vec<ConfigurationValue>> {
    let query = String::from(
        r#"
            SELECT 
                config_defaults.id AS id,
                config_sections.name AS section,
                config_defaults.cfg_key AS key,
                config_defaults.cfg_value AS value,
                config_value_types.name AS value_type
            FROM config_defaults
            INNER JOIN config_sections ON config_defaults.section_id = config_sections.id
            INNER JOIN config_value_types ON config_defaults.value_type_id = config_value_types.id
        "#,
    );

    let mut builder: sqlx::QueryBuilder<Postgres> = sqlx::QueryBuilder::new(query);

    if let Some(section) = section {
        builder.push("\nWHERE config_sections.name = ");
        builder.push_bind(section);
    }

    if let Some(key) = key {
        if !builder.sql().contains("WHERE") {
            builder.push("\nWHERE config_defaults.cfg_key = ");
        } else {
            builder.push(" AND config_defaults.cfg_key = ");
        }
        builder.push_bind(key);
    }

    builder.push("\nORDER BY config_sections.name, config_defaults.cfg_key ASC;");

    let defaults = builder.build();

    let results = defaults
        .fetch_all(&mut **tx)
        .await?
        .iter()
        .map(|r| ConfigurationValue {
            id: r.get("id"),
            section: r.get("section"),
            key: r.get("key"),
            value: r.get("value"),
            value_type: r.get("value_type"),
        })
        .collect();

    Ok(results)
}

/// Sets a configuration value in the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::set_config_value(&mut tx, "DashboardAggregator", "Website.URL", "https://example.com", "string").await?;
/// tx.commit().await?;
/// ```
pub async fn set_config_value(
    tx: &mut Transaction<'_, Postgres>,
    section: &str,
    key: &str,
    value: &str,
    value_type: &str,
) -> anyhow::Result<i32> {
    let section_record = sqlx::query!(
        r#"
                SELECT id FROM config_sections WHERE name = $1
        "#,
        section
    )
    .fetch_one(&mut **tx)
    .await?;

    let section_id = section_record.id;

    Ok(sqlx::query!(
        r#"
            INSERT INTO config_values
                (section_id, cfg_key, cfg_value, value_type_id, default_id) 
            VALUES (
                $1,
                $2,
                $3,
                (SELECT id FROM config_value_types WHERE name = $4),
                (SELECT id FROM config_defaults WHERE cfg_key = $5 AND section_id = $6)
            )
            RETURNING id
        "#,
        section_id,
        key,
        value,
        value_type,
        key,
        section_id,
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Updates or inserts a configuration value in the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::update_config_value(&mut tx, "DashboardAggregator", "Website.URL", "https://example.com", "string").await?;
/// tx.commit().await?;
/// ```
pub async fn update_config_value(
    tx: &mut Transaction<'_, Postgres>,
    section: &str,
    key: &str,
    value: &str,
    value_type: &str,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
                UPDATE config_values 
                SET cfg_value = $1, 
                    value_type_id = (
                        SELECT id 
                        FROM config_value_types 
                        WHERE name = $2
                    ) 
                WHERE cfg_key = $3 
                AND section_id = (
                    SELECT id 
                    FROM config_sections 
                    WHERE name = $4
                )
                RETURNING id
            "#,
        value,
        value_type,
        key,
        section
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Returns whether a configuration value exists in the database associated
/// with the given environment, section, and key.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::has_config_value(&mut tx, "dev", "DashboardAggregator", "Website.URL").await?;
/// tx.commit().await?;
/// ```
pub async fn has_config_value(
    tx: &mut Transaction<'_, Postgres>,
    environment: &str,
    section: &str,
    key: &str,
) -> anyhow::Result<bool> {
    let default = sqlx::query!(
        r#"
                SELECT config_values.id 
                FROM environments
                INNER JOIN environments_config_values ON environments.id = environments_config_values.environment_id
                INNER JOIN config_values ON environments_config_values.config_value_id = config_values.id
                INNER JOIN config_sections ON config_values.section_id = config_sections.id
                WHERE environments.name = $1 AND config_sections.name = $2 AND config_values.cfg_key = $3
        "#,
        environment,
        section,
        key
    )
    .fetch_optional(&mut **tx)
    .await?;

    Ok(default.is_some())
}

/// Returns a configuration value from the database based on the
/// environment, section, and key.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::get_config_value(&mut tx, "dev", "DashboardAggregator", "Website.URL").await?;
/// tx.commit().await?;
/// ```
pub async fn get_config_value(
    tx: &mut Transaction<'_, Postgres>,
    environment: &str,
    section: &str,
    key: &str,
) -> anyhow::Result<ConfigurationValue> {
    let cfg = sqlx::query_as!(
        ConfigurationValue,
        r#"
                SELECT 
                    config_values.id AS id,
                    config_sections.name AS section,
                    config_values.cfg_key AS key,
                    config_values.cfg_value AS value,
                    config_value_types.name AS value_type
                FROM environments
                INNER JOIN environments_config_values ON environments.id = environments_config_values.environment_id
                INNER JOIN config_values ON environments_config_values.config_value_id = config_values.id
                INNER JOIN config_sections ON config_values.section_id = config_sections.id
                INNER JOIN config_value_types ON config_values.value_type_id = config_value_types.id
                WHERE environments.name = $1 AND config_sections.name = $2 AND config_values.cfg_key = $3
        "#,
        environment,
        section,
        key
    )
    .fetch_one(&mut **tx)
    .await?;
    Ok(cfg)
}

/// Deletes a configuration value from the database based on the
/// environment, section, and key.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::delete_config_value(&mut tx, "dev", "DashboardAggregator", "Website.URL").await?;
/// tx.commit().await?;
/// ```
pub async fn delete_config_value(
    tx: &mut Transaction<'_, Postgres>,
    environment: &str,
    section: &str,
    key: &str,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
            r#"
                DELETE FROM environments
                WHERE environments.id = (
                    SELECT environments.id
                    FROM environments
                    INNER JOIN environments_config_values ON environments.id = environments_config_values.environment_id
                    INNER JOIN config_values ON environments_config_values.config_value_id = config_values.id
                    INNER JOIN config_sections ON config_values.section_id = config_sections.id
                    WHERE environments.name = $1 AND config_sections.name = $2 AND config_values.cfg_key = $3
                )
                RETURNING id
            "#,
            environment,
            section,
            key
        )
        .fetch_one(&mut **tx)
        .await?
        .id)
}

/// Returns a listing of the configuration values stored in the
/// database based on the environment, section, and key. If no environment,
/// section, or key is provided, all configuration values are returned. If an
/// environment is provided but no section or key, all configuration values for
/// that environment are returned. If an environment and section are provided
/// but no key, all configuration values for that environment and section are
/// returned. If an environment, section, and key are provided, only the
/// configuration value for that environment, section, and key is returned.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::list_config_values(&mut tx, None, None, None).await?;
/// tx.commit().await?;
///
/// for cfg in result {
///   println!("{:?}", cfg);
/// }
/// ```
pub async fn list_config_values(
    tx: &mut Transaction<'_, Postgres>,
    environment: Option<&str>,
    section: Option<&str>,
    key: Option<&str>,
) -> anyhow::Result<Vec<ConfigurationValue>> {
    let query = String::from(
        r#"
SELECT 
    config_values.id AS id,
    config_sections.name AS section,
    config_values.cfg_key AS key,
    config_values.cfg_value AS value,
    config_value_types.name AS value_type
FROM environments
INNER JOIN environments_config_values ON environments.id = environments_config_values.environment_id
INNER JOIN config_values ON environments_config_values.config_value_id = config_values.id
INNER JOIN config_sections ON config_values.section_id = config_sections.id
INNER JOIN config_value_types ON config_values.value_type_id = config_value_types.id"#,
    );

    let mut builder: sqlx::QueryBuilder<Postgres> = sqlx::QueryBuilder::new(query);

    if let Some(environment) = environment {
        builder.push("\nWHERE environments.name = ");
        builder.push_bind(environment);
    }

    if let Some(section) = section {
        if !builder.sql().contains("WHERE") {
            builder.push("\nWHERE config_sections.name = ");
        } else {
            builder.push(" AND config_sections.name = ");
        }
        builder.push_bind(section);
    }

    if let Some(key) = key {
        if builder.sql().contains("WHERE") {
            builder.push("\nWHERE config_values.cfg_key = ");
        } else {
            builder.push(" AND config_values.cfg_key = ");
        }
        builder.push_bind(key);
    }

    let cfgs = builder.build();

    let results = cfgs
        .fetch_all(&mut **tx)
        .await?
        .iter()
        .map(|r| ConfigurationValue {
            id: r.get("id"),
            section: r.get("section"),
            key: r.get("key"),
            value: r.get("value"),
            value_type: r.get("value_type"),
        })
        .collect();

    Ok(results)
}

/// Adds a configuration value to an environment in the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::add_env_cfg_value(&mut tx, 1, 1).await?;
/// tx.commit().await?;
/// ```
pub async fn add_env_cfg_value(
    tx: &mut Transaction<'_, Postgres>,
    env_id: i32,
    cfg_id: i32,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
            r#"
                INSERT INTO environments_config_values (environment_id, config_value_id) VALUES ($1, $2) RETURNING id
            "#,
            env_id,
            cfg_id
        )
        .fetch_one(&mut **tx)
        .await?
        .id)
}

/// Updates a configuration value in an environment.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::update_env_cfg_value(&mut tx, 1, 1).await?;
/// tx.commit().await?;
/// ```
pub async fn update_env_cfg_value(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
    section: &str,
    key: &str,
    value: &str,
    val_type: &str,
) -> anyhow::Result<()> {
    let cfg_id = get_config_value(tx, env, section, key).await?.id;

    sqlx::query!(
        r#"
            UPDATE config_values
            SET
                cfg_value = $1,
                value_type_id = (SELECT id FROM config_value_types WHERE name = $2)
            WHERE id = $3
        "#,
        value,
        val_type,
        cfg_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// Represents a single service as stored in the database.
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Service {
    pub name: String,
    pub id: i32,
    pub repo_id: i32,
}

/// Returns a listing of the services stored in the database that are affected
/// by changes to the given configuration value in an environment.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::list_affected_services(&mut tx, "dev", 1).await?;
/// tx.commit().await?;
///
/// for service in result {
///    println!("{}", service);
/// }
/// ```
pub async fn list_affected_services(
    tx: &mut Transaction<'_, Postgres>,
    environment: &str,
    cfg_id: i32,
) -> anyhow::Result<Vec<Service>> {
    let services = sqlx::query_as!(
        Service,
        r#"
                SELECT 
                    services.name AS name,
                    services.id AS id,
                    services.repo_id AS repo_id
                FROM environments
                INNER JOIN environments_services ON environments.id = environments_services.environment_id
                INNER JOIN services ON environments_services.service_id = services.id
                INNER JOIN environments_config_values ON environments.id = environments_config_values.environment_id
                INNER JOIN config_values ON environments_config_values.config_value_id = config_values.id
                INNER JOIN environments_services_config_values a ON environments_services.id = a.environment_service_id
                INNER JOIN environments_services_config_values b ON environments_config_values.id = b.environment_config_value_id
                WHERE environments.name = $1 AND config_values.id = $2
        "#,
        environment,
        cfg_id
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(services)
}

/// Lists services in the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::list_services(&mut tx).await?;
/// tx.commit().await?;
///
/// for service in result {
///    println!("{}", service);
/// }
/// ```
pub async fn list_services(
    tx: &mut Transaction<'_, Postgres>,
    environment: &str,
) -> anyhow::Result<Vec<Service>> {
    let services = sqlx::query_as!(
        Service,
        r#"
                SELECT 
                    services.name AS name,
                    services.id AS id,
                    services.repo_id AS repo_id
                FROM environments
                INNER JOIN environments_services ON environments.id = environments_services.environment_id
                INNER JOIN services ON environments_services.service_id = services.id
                WHERE environments.name = $1
        "#,
        environment
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(services)
}

/// Returns the services for an environment.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::get_services(&mut tx, "dev").await?;
/// tx.commit().await?;
///
/// for service in result {
///   println!("{}", service.name);
/// }
/// ```
pub async fn get_services(
    tx: &mut Transaction<'_, Postgres>,
    environment: &str,
) -> anyhow::Result<Vec<Service>> {
    let services = sqlx::query_as!(
        Service,
        r#"
            SELECT 
                services.id AS id, 
                services.name AS name, 
                services.repo_id AS repo_id
            FROM environments
            INNER JOIN environments_services ON environments.id = environments_services.environment_id
            INNER JOIN services ON environments_services.service_id = services.id
            WHERE environments.name = $1
        "#,
        environment
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(services)
}

/// Returns a listing of all of the services.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::get_all_services(&mut tx).await?;
///
/// for service in result {
///    println!("{}", service.name);
/// }
pub async fn get_all_services(tx: &mut Transaction<'_, Postgres>) -> anyhow::Result<Vec<Service>> {
    let services = sqlx::query_as!(
        Service,
        r#"
            SELECT 
                services.id AS id, 
                services.name AS name, 
                services.repo_id AS repo_id
            FROM services
        "#
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(services)
}

/// Adds a service to an environment.
/// Returns the primary key of the environments_services record that is added.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::add_service_to_env(&mut tx, "dev", "dashboard-aggregator").await?;
/// tx.commit().await?;
/// ```
pub async fn add_service_to_env(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
    service_name: &str,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
            INSERT INTO 
                environments_services (environment_id, service_id) 
            VALUES 
                (
                    (SELECT id FROM environments WHERE name = $1),
                    (SELECT id FROM services WHERE name = $2)
                )
            RETURNING id
        "#,
        env,
        service_name
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Removes a service from an environment.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::remove_service_from_env(&mut tx, "dev", "dashboard-aggregator").await?;
/// tx.commit().await?;
/// ```
pub async fn remove_service_from_env(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
    service_name: &str,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM environments_services
            WHERE environment_id = (SELECT id FROM environments WHERE name = $1)
            AND service_id = (SELECT id FROM services WHERE name = $2)
        "#,
        env,
        service_name
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Repository {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub revision: String,
}

/// Returns the repository by its primary key.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::get_repo_by_id(&mut tx, 1).await?;
/// tx.commit().await?;
///
/// println!("{}", result.id);
/// println!("{}", result.name);
/// println!("{}", result.url);
/// println!("{}", result.revision);
/// ```
pub async fn get_repo_by_id(
    tx: &mut Transaction<'_, Postgres>,
    id: i32,
) -> anyhow::Result<Repository> {
    let repo = sqlx::query_as!(
        Repository,
        r#"
            SELECT 
                repos.id AS id, 
                repos.name AS name, 
                repos.url AS url, 
                repos.revision AS revision
            FROM repos
            WHERE repos.id = $1
        "#,
        id
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(repo)
}

/// Returns the namespace for the provided environment.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::get_namespace(&mut tx, "dev").await?;
/// tx.commit().await?;
///
/// println!("{}", result);
/// ```
pub async fn get_namespace(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
) -> anyhow::Result<String> {
    let namespace = sqlx::query!(
        r#"
            SELECT environments.namespace AS namespace
            FROM environments
            WHERE environments.name = $1
        "#,
        env
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(namespace.namespace)
}

/// Represents a set of feature flags for an environment as returned from
/// the database.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeatureFlags {
    pub administration: bool,
    pub analytics: bool,
    pub agave: bool,
    pub base_urls: bool,
    pub cas: bool,
    pub docker: bool,
    pub infosquito: bool,
    pub intercom: bool,
    pub jaeger: bool,
    pub jobs: bool,
    pub jvmopts: bool,
    pub permanent_id: bool,
    pub qa: bool,
    pub qms: bool,
    pub unleash: bool,
}

/// Returns the feature flag settings for the provided environment.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::get_feature_flags(&mut tx, "dev").await?;
/// tx.commit().await?;
///
/// for flag in result {
///    println!("{}", flag);
/// }
/// ```
pub async fn get_feature_flags(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
) -> anyhow::Result<FeatureFlags> {
    Ok(sqlx::query_as!(
        FeatureFlags,
        r#"
            SELECT 
                environments_features.administration AS administration,
                environments_features.analytics AS analytics,
                environments_features.agave AS agave,
                environments_features.base_urls AS base_urls,
                environments_features.cas AS cas,
                environments_features.docker AS docker,
                environments_features.infosquito AS infosquito,
                environments_features.intercom AS intercom,
                environments_features.jaeger AS jaeger,
                environments_features.jobs AS jobs,
                environments_features.jvmopts AS jvmopts,
                environments_features.permanent_id AS permanent_id,
                environments_features.qa AS qa,
                environments_features.qms AS qms,
                environments_features.unleash AS unleash
            FROM environments_features
            INNER JOIN environments ON environments.id = environments_features.environment_id
            WHERE environments.name = $1
        "#,
        env
    )
    .fetch_one(&mut **tx)
    .await?)
}

/// Upserts feature flags for an environment.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::upsert_feature_flags(tx, "dev", &FeatureFlags).await?;
/// tx.commit().await?;
/// ```
pub async fn upsert_feature_flags(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
    flags: &FeatureFlags,
) -> anyhow::Result<bool> {
    Ok(sqlx::query!(
        r#"
            INSERT INTO environments_features (
                environment_id, 
                administration, 
                analytics, 
                agave, 
                base_urls, 
                cas, docker, 
                infosquito, 
                intercom, 
                jaeger, 
                jobs, 
                jvmopts, 
                permanent_id, 
                qa, 
                qms, 
                unleash
            ) VALUES (
                (SELECT id FROM environments WHERE name = $1),
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9,
                $10,
                $11,
                $12,
                $13,
                $14,
                $15,
                $16
            ) ON CONFLICT (environment_id) DO UPDATE
            SET
                administration = $2,
                analytics = $3,
                agave = $4,
                base_urls = $5,
                cas = $6,
                docker = $7,
                infosquito = $8,
                intercom = $9,
                jaeger = $10,
                jobs = $11,
                jvmopts = $12,
                permanent_id = $13,
                qa = $14,
                qms = $15,
                unleash = $16
        "#,
        env,
        flags.administration,
        flags.analytics,
        flags.agave,
        flags.base_urls,
        flags.cas,
        flags.docker,
        flags.infosquito,
        flags.intercom,
        flags.jaeger,
        flags.jobs,
        flags.jvmopts,
        flags.permanent_id,
        flags.qa,
        flags.qms,
        flags.unleash
    )
    .execute(&mut **tx)
    .await?
    .rows_affected()
        > 0)
}

/// Sets a value for a feature flag. Returns a result with the number of rows affected.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::set_feature_flag(&mut tx, "dev", "administration", true).await?;
/// tx.commit().await?;
/// ```
pub async fn set_feature_flag(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
    flag: &str,
    value: &str,
) -> anyhow::Result<u64> {
    // I'm unsure about this, but it should be safe for now since the flag and the value are checked against a whitelist
    // set in clap. If this changes, this should be revisited.
    let flag = flag.to_lowercase();

    let query = format!(
        r#"
            UPDATE environments_features
            INNER JOIN environments ON environments.id = environments_features.environment_id
            SET environments_features.{} = "#,
        flag,
    );

    let mut builder: sqlx::QueryBuilder<Postgres> = sqlx::QueryBuilder::new(query);
    builder.push_bind(value);
    builder.push("WHERE environments.name = ");
    builder.push_bind(env);

    let result = builder.build().execute(&mut **tx).await?;

    Ok(result.rows_affected())
}

/// Returns a listing of the configuration templates in use by services
/// in the provided environment.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::list_templates(&mut tx, "dev").await?;
/// tx.commit().await?;
///
/// for template in result {
///    println!("{}", template);
/// }
/// ```
pub async fn list_templates(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
) -> anyhow::Result<Vec<String>> {
    let templates = sqlx::query!(
        r#"
            SELECT DISTINCT ct.path AS path
            FROM config_templates ct
            JOIN environments_services_config_templates ect ON ect.config_template_id = ct.id
            JOIN environments_services es ON es.id = ect.environment_service_id
            JOIN environments e ON e.id = es.environment_id
            WHERE e.name = $1
        "#,
        env
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(templates.into_iter().map(|t| t.path).collect())
}

pub async fn list_template_ids(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
) -> anyhow::Result<Vec<i32>> {
    Ok(sqlx::query!(
        r#"
            SELECT ct.id AS id
            FROM config_templates ct
            JOIN environments_services_config_templates ect ON ect.config_template_id = ct.id
            JOIN environments_services es ON es.id = ect.environment_service_id
            JOIN environments e ON e.id = es.environment_id
            WHERE e.name = $1
        "#,
        env
    )
    .fetch_all(&mut **tx)
    .await?
    .into_iter()
    .map(|t| t.id)
    .collect())
}

pub async fn list_service_templates(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
    service_name: &str,
) -> anyhow::Result<Vec<i32>> {
    Ok(sqlx::query!(
        r#"
            SELECT ct.id AS id
            FROM config_templates ct
            JOIN environments_services_config_templates ect ON ect.config_template_id = ct.id
            JOIN environments_services es ON es.id = ect.environment_service_id
            JOIN environments e ON e.id = es.environment_id
            JOIN services s ON s.id = es.service_id
            WHERE e.name = $1 AND s.name = $2
        "#,
        env,
        service_name
    )
    .fetch_all(&mut **tx)
    .await?
    .into_iter()
    .map(|t| t.id)
    .collect())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ESCT {
    id: i32,
    environment_service_id: i32,
    config_template_id: i32,
    path: String,
}

/// Adds a configuration template for a service to an environment.
pub async fn copy_service_template_to_env(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
    from: &str,
    service_name: &str,
    config_template_id: i32,
) -> anyhow::Result<()> {
    // Get the ESCTs for the from environment. Basically just need the path.
    let from_esct: ESCT = sqlx::query_as!(
        ESCT,
        r#"
            SELECT 
                environments_services_config_templates.id AS id,
                environments_services_config_templates.environment_service_id AS environment_service_id,
                environments_services_config_templates.config_template_id AS config_template_id,
                config_templates.path AS path
            FROM environments_services_config_templates
            JOIN environments_services ON environments_services.id = environments_services_config_templates.environment_service_id
            JOIN environments ON environments.id = environments_services.environment_id
            JOIN config_templates ON config_templates.id = environments_services_config_templates.config_template_id
            JOIN services ON services.id = environments_services.service_id
            WHERE environments.name = $1 AND services.name = $2 AND config_templates.id = $3
        "#,
        from,
        service_name,
        config_template_id
    ).fetch_one(&mut **tx).await?;

    sqlx::query!(
        r#"
            INSERT INTO environments_services_config_templates
                (environment_service_id, config_template_id, path)
            VALUES
                (
                    (
                        SELECT id 
                        FROM environments_services
                        WHERE environment_id = (SELECT id FROM environments WHERE name = $1)
                        AND service_id = (SELECT id FROM services WHERE name = $2)
                    ),
                    $3,
                    $4
                )
        "#,
        env,
        service_name,
        config_template_id,
        from_esct.path
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// Adds a configuration template to the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::add_template(&mut tx, repo_id: i32, path: &PathBuf).await?;
/// tx.commit().await?;
/// ```
pub async fn add_template(
    tx: &mut Transaction<'_, Postgres>,
    repo_id: i32,
    path: &PathBuf,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
            INSERT INTO config_templates (repo_id, path) VALUES ($1, $2) RETURNING id
        "#,
        repo_id,
        path.to_str().context("Failed to convert path to string")?
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Adds a configuration template to a service in an environment.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::add_template_to_service(&mut tx, "dev", "dashboard-aggregator", 1).await?;
/// tx.commit().await?;
/// ```
pub async fn add_template_to_service(
    tx: &mut Transaction<'_, Postgres>,
    env: &str,
    service_name: &str,
    config_template_id: i32,
    render_path: &str,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
            INSERT INTO environments_services_config_templates
                (environment_service_id, config_template_id, path)
            VALUES
                (
                    (
                        SELECT id 
                        FROM environments_services
                        WHERE environment_id = (SELECT id FROM environments WHERE name = $1)
                        AND service_id = (SELECT id FROM services WHERE name = $2)
                    ),
                    $3,
                    $4
                )
            RETURNING id
        "#,
        env,
        service_name,
        config_template_id,
        render_path
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

#[derive(tabled::Tabled, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct TemplateInfo {
    pub environment: String,
    pub service_id: i32,
    pub service_name: String,
    pub repo_id: i32,
    pub repo_name: String,
    pub repo_url: String,
    pub repo_revision: String,
    pub template_id: i32,
    pub template_path: String,
}

/// Returns a listing of the configuration templates in the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let mut templates: Vec<String> = Vec::new();
/// templates.push(String::from("templates/website.conf"));
/// templates.push(String::from("templates/website2.conf"));
/// let result = db::list_template_info(tx, templates).await?;
/// tx.commit().await?;
///
/// for template in result {
///   println!("{:#?}", template);
/// }
/// ```
pub async fn list_template_info(
    tx: &mut Transaction<'_, Postgres>,
    template_paths: &[String],
) -> anyhow::Result<Vec<TemplateInfo>> {
    let mut builder: sqlx::QueryBuilder<Postgres> = sqlx::QueryBuilder::new(String::from(
        r#"
            SELECT
                e.name AS environment,
                s.id AS service_id,
                s.name AS service_name,
                r.id AS repo_id,
                r.name AS repo_name,
                r.url AS repo_url,
                r.revision AS repo_revision,
                ct.id AS template_id,
                ct.path AS template_path
            FROM environments e
            INNER JOIN environments_services es ON e.id = es.environment_id
            INNER JOIN services s ON es.service_id = s.id
            INNER JOIN repos r ON s.repo_id = r.id
            INNER JOIN environments_services_config_templates esct ON es.id = esct.environment_service_id
            INNER JOIN config_templates ct ON esct.config_template_id = ct.id
            WHERE ct.path IN (
        "#,
    ));

    template_paths
        .into_iter()
        .enumerate()
        .for_each(|(i, template_path)| {
            if i > 0 {
                builder.push(", ");
            }
            builder.push_bind(template_path);
        });

    builder.push(")");

    let query = builder.build();

    let results = query
        .fetch_all(&mut **tx)
        .await?
        .iter()
        .map(|r| TemplateInfo {
            environment: r.get("environment"),
            service_id: r.get("service_id"),
            service_name: r.get("service_name"),
            repo_id: r.get("repo_id"),
            repo_name: r.get("repo_name"),
            repo_url: r.get("repo_url"),
            repo_revision: r.get("repo_revision"),
            template_id: r.get("template_id"),
            template_path: r.get("template_path"),
        })
        .collect();

    Ok(results)
}

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub revision: String,
}

#[derive(tabled::Tabled, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableRepo {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub revision: String,
}

impl From<Repo> for TableRepo {
    fn from(repo: Repo) -> Self {
        Self {
            id: repo.id,
            name: repo.name,
            url: repo.url,
            revision: repo.revision,
        }
    }
}

/// Lists repositories contained in the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::list_repos(&mut tx).await?;
/// tx.commit().await?;
///
/// for repo in result {
///  println!("{}", repo);
/// }
/// ```
pub async fn list_repos(tx: &mut Transaction<'_, Postgres>) -> anyhow::Result<Vec<TableRepo>> {
    let repos: Vec<TableRepo> = sqlx::query_as!(
        Repo,
        r#"
            SELECT 
                repos.id AS id, 
                repos.name AS name, 
                repos.url AS url, 
                repos.revision AS revision
            FROM repos
        "#
    )
    .fetch_all(&mut **tx)
    .await?
    .into_iter()
    .map(|r| r.into())
    .collect();

    Ok(repos)
}

/// Adds a repo to the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::add_repo(&mut tx, "name", "url", "revision").await?;
/// tx.commit().await?;
///
/// println!("{}", result);
/// ```
pub async fn add_repo(
    tx: &mut Transaction<'_, Postgres>,
    name: &str,
    url: &url::Url,
    revision: &str,
) -> anyhow::Result<i32> {
    Ok(sqlx::query!(
        r#"
            INSERT INTO repos (name, url, revision) VALUES ($1, $2, $3) RETURNING id
        "#,
        name,
        url.as_str(),
        revision
    )
    .fetch_one(&mut **tx)
    .await?
    .id)
}

/// Deletes a repo from the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::delete_repo(&mut tx, 1).await?;
/// tx.commit().await?;
/// ```
pub async fn delete_repo(tx: &mut Transaction<'_, Postgres>, id: i32) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM repos WHERE id = $1
        "#,
        id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
