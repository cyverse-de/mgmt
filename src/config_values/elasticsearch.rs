use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Elasticsearch {
    #[serde(skip)]
    section: String,

    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,

    username: String,
    password: String,
    index: String,

    enabled: bool,
}

impl Default for Elasticsearch {
    fn default() -> Self {
        Elasticsearch {
            section: "Elasticsearch".to_string(),
            base_uri: None,
            username: String::new(),
            password: String::new(),
            index: String::new(),
            enabled: true,
        }
    }
}

impl LoadFromDatabase for Elasticsearch {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "BaseURI" => self.base_uri = Url::parse(&value).ok(),
            "Username" => self.username = value,
            "Password" => self.password = value,
            "Index" => self.index = value,
            "Enabled" => self.enabled = value.parse::<bool>()?,
            _ => (),
        }

        Ok(())
    }
}

impl From<Elasticsearch> for Vec<db::ConfigurationValue> {
    fn from(es: Elasticsearch) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if es.section.is_empty() {
            section = "Elasticsearch".to_string();
        } else {
            section = es.section.clone();
        }

        if let Some(base_uri) = es.base_uri {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "BaseURI".to_string(),
                value: base_uri.to_string(),
                value_type: "string".to_string(),
            });
        }

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Username".to_string(),
            value: es.username,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Password".to_string(),
            value: es.password,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Index".to_string(),
            value: es.index,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Enabled".to_string(),
            value: es.enabled.to_string(),
            value_type: "bool".to_string(),
        });

        vec
    }
}

impl Elasticsearch {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Base URI")
            .default("http://elasticsearch:9200".into())
            .interact()?;

        let username = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Username")
            .allow_empty(true)
            .interact()?;

        let password = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Password")
            .allow_empty(true)
            .interact()?;

        let index = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Index")
            .default("data".into())
            .interact()?;

        let enabled = Input::<bool>::with_theme(theme)
            .with_prompt("ElasticSearch Enabled")
            .default(true)
            .interact()?;

        let base_uri_id =
            set_config_value(tx, "Elasticsearch", "BaseURI", &base_uri, "string").await?;
        add_env_cfg_value(tx, env_id, base_uri_id).await?;
        self.base_uri = Url::parse(&base_uri).ok();

        let username_id =
            set_config_value(tx, "Elasticsearch", "Username", &username, "string").await?;
        add_env_cfg_value(tx, env_id, username_id).await?;
        self.username = username;

        let password_id =
            set_config_value(tx, "Elasticsearch", "Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        let index_id = set_config_value(tx, "Elasticsearch", "Index", &index, "string").await?;
        add_env_cfg_value(tx, env_id, index_id).await?;
        self.index = index;

        let enabled_id =
            set_config_value(tx, "Elasticsearch", "Enabled", &enabled.to_string(), "bool").await?;
        add_env_cfg_value(tx, env_id, enabled_id).await?;
        self.enabled = enabled;

        Ok(())
    }
}
