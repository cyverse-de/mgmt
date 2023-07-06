use crate::db::{add_env_cfg_value, set_config_value};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ElasticSearch {
    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,

    username: String,
    password: String,
    index: String,
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
            set_config_value(tx, "ElasticSearch", "BaseURI", &base_uri, "string").await?;
        add_env_cfg_value(tx, env_id, base_uri_id).await?;
        self.base_uri = Url::parse(&base_uri).ok();

        let username_id =
            set_config_value(tx, "ElasticSearch", "Username", &username, "string").await?;
        add_env_cfg_value(tx, env_id, username_id).await?;
        self.username = username;

        let password_id =
            set_config_value(tx, "ElasticSearch", "Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        let index_id = set_config_value(tx, "ElasticSearch", "Index", &index, "string").await?;
        add_env_cfg_value(tx, env_id, index_id).await?;
        self.index = index;

        Ok(())
    }
}
