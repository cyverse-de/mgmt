use sqlx::{MySql, Row, Transaction};

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

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub id: Option<i64>,
    pub section: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub value_type: Option<String>,
}

pub async fn get_default_config_value(
    tx: &mut Transaction<'_, MySql>,
    section: &str,
    key: &str,
) -> anyhow::Result<Configuration> {
    let default = sqlx::query_as!(
        Configuration,
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
) -> anyhow::Result<Vec<Configuration>> {
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
        .map(|r| Configuration {
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
) -> anyhow::Result<Configuration> {
    let cfg = sqlx::query_as!(
        Configuration,
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
) -> anyhow::Result<Vec<Configuration>> {
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
        .map(|r| Configuration {
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
