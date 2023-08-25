//! # Database Access
//!
//! This module contains all the database access code for the application.
use sqlx::{MySql, Row, Transaction};

/// Represents a single configuration value as stored in the database.
#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigurationValue {
    pub id: Option<i64>,
    pub section: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub value_type: Option<String>,
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
    tx: &mut Transaction<'_, MySql>,
    environment: &str,
    namespace: &str,
) -> anyhow::Result<u64> {
    Ok(sqlx::query!(
            r#"
                INSERT INTO environments (name, namespace) VALUES (?, ?) ON DUPLICATE KEY UPDATE name = VALUES(name)
            "#,
            environment,
            namespace
        )
        .execute(&mut **tx)
        .await?
        .last_insert_id())
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
pub async fn list_envs(tx: &mut Transaction<'_, MySql>) -> anyhow::Result<Vec<String>> {
    let envs = sqlx::query!(
        r#"
            SELECT name FROM environments
        "#
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(envs.into_iter().filter_map(|e| e.name).collect())
}

/// Deletes an environment from the database.
///
/// # Examples
/// ```ignore
/// let mut tx = db.begin().await?;
/// let result = db::delete_env(&mut tx, "dev").await?;
/// tx.commit().await?;
/// ```
pub async fn delete_env(tx: &mut Transaction<'_, MySql>, environment: &str) -> anyhow::Result<u64> {
    Ok(sqlx::query!(
        r#"
                DELETE FROM environments WHERE name = ?
        "#,
        environment
    )
    .execute(&mut **tx)
    .await?
    .last_insert_id())
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
    tx: &mut Transaction<'_, MySql>,
    environment: &str,
) -> anyhow::Result<Option<u64>> {
    let env_id = sqlx::query!(
        r#"
                SELECT id AS `id: u64` FROM environments WHERE name = ?
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
pub async fn get_repos(tx: &mut Transaction<'_, MySql>) -> anyhow::Result<Vec<(String, String)>> {
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
            let u = r.url.unwrap_or_default();
            let n = r.name.unwrap_or_default();
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
pub async fn add_section(tx: &mut Transaction<'_, MySql>, section: &str) -> anyhow::Result<u64> {
    Ok(sqlx::query!(
        r#"
                INSERT INTO config_sections (name) VALUES (?) ON DUPLICATE KEY UPDATE id = id
        "#,
        section
    )
    .execute(&mut **tx)
    .await?
    .last_insert_id())
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
pub async fn has_section(tx: &mut Transaction<'_, MySql>, section: &str) -> anyhow::Result<bool> {
    let section = sqlx::query!(
        r#"
                SELECT id FROM config_sections WHERE name = ?
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
pub async fn delete_section(tx: &mut Transaction<'_, MySql>, section: &str) -> anyhow::Result<u64> {
    Ok(sqlx::query!(
        r#"
                DELETE FROM config_sections WHERE name = ?
        "#,
        section
    )
    .execute(&mut **tx)
    .await?
    .last_insert_id())
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
pub async fn list_sections(tx: &mut Transaction<'_, MySql>) -> anyhow::Result<Vec<String>> {
    let sections = sqlx::query!(
        r#"
                SELECT name FROM config_sections
        "#
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(sections.into_iter().filter_map(|s| s.name).collect())
}

pub async fn get_default_config_value(
    tx: &mut Transaction<'_, MySql>,
    section: &str,
    key: &str,
) -> anyhow::Result<ConfigurationValue> {
    let default = sqlx::query_as!(
        ConfigurationValue,
        r#"
                SELECT
                    config_defaults.id AS `id: i64`,
                    config_sections.name AS `section: String`,
                    config_defaults.cfg_key AS `key: String`, 
                    config_defaults.cfg_value AS `value: String`,
                    config_value_types.name AS `value_type: String`
                FROM config_defaults
                INNER JOIN config_sections ON config_defaults.section_id = config_sections.id
                INNER JOIN config_value_types ON config_defaults.value_type_id = config_value_types.id
                WHERE config_sections.name = ? AND config_defaults.cfg_key = ?
        "#,
        section,
        key
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(default)
}

pub async fn set_default_config_value(
    tx: &mut Transaction<'_, MySql>,
    section: &str,
    key: &str,
    value: &str,
    value_type: &str,
) -> anyhow::Result<u64> {
    Ok(sqlx::query!(
        r#"
                INSERT INTO config_defaults (section_id, cfg_key, cfg_value, value_type_id) VALUES (
                    (SELECT id FROM config_sections WHERE name = ?),
                    ?,
                    ?,
                    (SELECT id FROM config_value_types WHERE name = ?)
                ) ON DUPLICATE KEY UPDATE cfg_value = VALUES(cfg_value)
            "#,
        section,
        key,
        value,
        value_type
    )
    .execute(&mut **tx)
    .await?
    .last_insert_id())
}

pub async fn has_default_config_value(
    tx: &mut Transaction<'_, MySql>,
    section: &str,
    key: &str,
) -> anyhow::Result<bool> {
    let default = sqlx::query!(
        r#"
                SELECT config_defaults.id FROM config_defaults
                INNER JOIN config_sections ON config_defaults.section_id = config_sections.id
                WHERE config_sections.name = ? AND config_defaults.cfg_key = ?
        "#,
        section,
        key
    )
    .fetch_optional(&mut **tx)
    .await?;

    Ok(default.is_some())
}

pub async fn delete_default_config_value(
    tx: &mut Transaction<'_, MySql>,
    section: &str,
    key: &str,
) -> anyhow::Result<u64> {
    Ok(sqlx::query!(
        r#"
                DELETE config_defaults FROM config_defaults
                INNER JOIN config_sections ON config_defaults.section_id = config_sections.id
                WHERE config_sections.name = ? AND config_defaults.cfg_key = ?
            "#,
        section,
        key
    )
    .execute(&mut **tx)
    .await?
    .last_insert_id())
}

pub async fn list_default_config_values(
    tx: &mut Transaction<'_, MySql>,
    section: Option<&str>,
    key: Option<&str>,
) -> anyhow::Result<Vec<ConfigurationValue>> {
    let query = String::from(
        r#"
SELECT 
    config_defaults.id AS id,
    config_sections.name AS section,
    config_defaults.cfg_key AS 'key',
    config_defaults.cfg_value AS 'value',
    config_value_types.name AS value_type
FROM config_defaults
INNER JOIN config_sections ON config_defaults.section_id = config_sections.id
INNER JOIN config_value_types ON config_defaults.value_type_id = config_value_types.id"#,
    );

    let mut builder: sqlx::QueryBuilder<MySql> = sqlx::QueryBuilder::new(query);

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
            id: Some(r.get("id")),
            section: Some(r.get("section")),
            key: Some(r.get("key")),
            value: Some(r.get("value")),
            value_type: Some(r.get("value_type")),
        })
        .collect();

    Ok(results)
}

pub async fn set_config_value(
    tx: &mut Transaction<'_, MySql>,
    section: &str,
    key: &str,
    value: &str,
    value_type: &str,
) -> anyhow::Result<u64> {
    let section_record = sqlx::query!(
        r#"
                SELECT id as `id: u64` FROM config_sections WHERE name = ?
        "#,
        section
    )
    .fetch_one(&mut **tx)
    .await?;

    let section_id = section_record
        .id
        .ok_or_else(|| anyhow::anyhow!("Failed to get section id for section {}", section))?;

    Ok(sqlx::query!(
        r#"
                INSERT INTO config_values
                    (section_id, cfg_key, cfg_value, value_type_id, default_id) 
                VALUES (
                    ?,
                    ?,
                    ?,
                    (SELECT id FROM config_value_types WHERE name = ?),
                    (SELECT id FROM config_defaults WHERE cfg_key = ? AND section_id = ?)
                )
            "#,
        section_id,
        key,
        value,
        value_type,
        key,
        section_id,
    )
    .execute(&mut **tx)
    .await?
    .last_insert_id())
}

pub async fn update_config_value(
    tx: &mut Transaction<'_, MySql>,
    section: &str,
    key: &str,
    value: &str,
    value_type: &str,
) -> anyhow::Result<u64> {
    Ok(sqlx::query!(
        r#"
                UPDATE config_values 
                SET cfg_value = ?, 
                    value_type_id = (
                        SELECT id 
                        FROM config_value_types 
                        WHERE name = ?
                    ) 
                WHERE cfg_key = ? 
                AND section_id = (
                    SELECT id 
                    FROM config_sections 
                    WHERE name = ?
                )
            "#,
        value,
        value_type,
        key,
        section
    )
    .execute(&mut **tx)
    .await?
    .last_insert_id())
}

pub async fn has_config_value(
    tx: &mut Transaction<'_, MySql>,
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
                WHERE environments.name = ? AND config_sections.name = ? AND config_values.cfg_key = ?
        "#,
        environment,
        section,
        key
    )
    .fetch_optional(&mut **tx)
    .await?;

    Ok(default.is_some())
}

pub async fn get_config_value(
    tx: &mut Transaction<'_, MySql>,
    environment: &str,
    section: &str,
    key: &str,
) -> anyhow::Result<ConfigurationValue> {
    let cfg = sqlx::query_as!(
        ConfigurationValue,
        r#"
                SELECT 
                    config_values.id AS `id: i64`,
                    config_sections.name AS `section: String`,
                    config_values.cfg_key AS `key: String`,
                    config_values.cfg_value AS `value: String`,
                    config_value_types.name AS `value_type: String`
                FROM environments
                INNER JOIN environments_config_values ON environments.id = environments_config_values.environment_id
                INNER JOIN config_values ON environments_config_values.config_value_id = config_values.id
                INNER JOIN config_sections ON config_values.section_id = config_sections.id
                INNER JOIN config_value_types ON config_values.value_type_id = config_value_types.id
                WHERE environments.name = ? AND config_sections.name = ? AND config_values.cfg_key = ?
        "#,
        environment,
        section,
        key
    )
    .fetch_one(&mut **tx)
    .await?;
    Ok(cfg)
}

pub async fn delete_config_value(
    tx: &mut Transaction<'_, MySql>,
    environment: &str,
    section: &str,
    key: &str,
) -> anyhow::Result<u64> {
    Ok(sqlx::query!(
            r#"
                DELETE config_values FROM environments
                INNER JOIN environments_config_values ON environments.id = environments_config_values.environment_id
                INNER JOIN config_values ON environments_config_values.config_value_id = config_values.id
                INNER JOIN config_sections ON config_values.section_id = config_sections.id
                WHERE environments.name = ? AND config_sections.name = ? AND config_values.cfg_key = ?
            "#,
            environment,
            section,
            key
        )
        .execute(&mut **tx)
        .await?
        .last_insert_id())
}

pub async fn list_config_values(
    tx: &mut Transaction<'_, MySql>,
    environment: Option<&str>,
    section: Option<&str>,
    key: Option<&str>,
) -> anyhow::Result<Vec<ConfigurationValue>> {
    let query = String::from(
        r#"
SELECT 
    config_values.id AS id,
    config_sections.name AS section,
    config_values.cfg_key AS 'key',
    config_values.cfg_value AS 'value',
    config_value_types.name AS value_type
FROM environments
INNER JOIN environments_config_values ON environments.id = environments_config_values.environment_id
INNER JOIN config_values ON environments_config_values.config_value_id = config_values.id
INNER JOIN config_sections ON config_values.section_id = config_sections.id
INNER JOIN config_value_types ON config_values.value_type_id = config_value_types.id"#,
    );

    let mut builder: sqlx::QueryBuilder<MySql> = sqlx::QueryBuilder::new(query);

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

pub async fn add_env_cfg_value(
    tx: &mut Transaction<'_, MySql>,
    env_id: u64,
    cfg_id: u64,
) -> anyhow::Result<u64> {
    Ok(sqlx::query!(
            r#"
                INSERT INTO environments_config_values (environment_id, config_value_id) VALUES (?, ?)
            "#,
            env_id,
            cfg_id
        )
        .execute(&mut **tx)
        .await?
        .last_insert_id())
}

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Service {
    pub name: Option<String>,
}

pub async fn list_affected_services(
    tx: &mut Transaction<'_, MySql>,
    environment: &str,
    cfg_id: u64,
) -> anyhow::Result<Vec<Service>> {
    let services = sqlx::query_as!(
        Service,
        r#"
                SELECT services.name AS `name: String` FROM environments
                INNER JOIN environments_services ON environments.id = environments_services.environment_id
                INNER JOIN services ON environments_services.service_id = services.id
                INNER JOIN environments_config_values ON environments.id = environments_config_values.environment_id
                INNER JOIN config_values ON environments_config_values.config_value_id = config_values.id
                INNER JOIN environments_services_config_values a ON environments_services.id = a.environment_service_id
                INNER JOIN environments_services_config_values b ON environments_config_values.id = b.environment_config_value_id
                WHERE environments.name = ? AND config_values.id = ?
        "#,
        environment,
        cfg_id
    )
    .fetch_all(&mut **tx)
    .await?;

    Ok(services)
}
