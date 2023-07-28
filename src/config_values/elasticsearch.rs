use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ElasticSearch {
    #[serde(skip)]
    section: String,

    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,

    username: String,
    password: String,
    index: String,
}

impl Default for ElasticSearch {
    fn default() -> Self {
        ElasticSearch {
            section: "Elasticsearch".to_string(),
            base_uri: None,
            username: String::new(),
            password: String::new(),
            index: String::new(),
        }
    }
}

impl LoadFromConfiguration for ElasticSearch {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "BaseURI" => self.base_uri = Url::parse(&value).ok(),
                "Username" => self.username = value,
                "Password" => self.password = value,
                "Index" => self.index = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<ElasticSearch> for Vec<db::Configuration> {
    fn from(es: ElasticSearch) -> Vec<db::Configuration> {
        let mut vec: Vec<db::Configuration> = Vec::new();
        let section: String;

        if es.section.is_empty() {
            section = "Elasticsearch".to_string();
        } else {
            section = es.section.clone();
        }

        if let Some(base_uri) = es.base_uri {
            vec.push(db::Configuration {
                id: None,
                section: Some(section.clone()),
                key: Some("BaseURI".to_string()),
                value: Some(base_uri.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        vec.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("Username".to_string()),
            value: Some(es.username),
            value_type: Some("string".to_string()),
        });

        vec.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("Password".to_string()),
            value: Some(es.password),
            value_type: Some("string".to_string()),
        });

        vec.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("Index".to_string()),
            value: Some(es.index),
            value_type: Some("string".to_string()),
        });

        vec
    }
}

impl ElasticSearch {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Base URI")
            .default("http://elasticsearch:9200".into())
            .interact()?;

        let username = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Username")
            .allow_empty(true)
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("ElasticSearch Password")
            .allow_empty_password(true)
            .interact()?;

        let index = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Index")
            .default("data".into())
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

        Ok(())
    }
}
