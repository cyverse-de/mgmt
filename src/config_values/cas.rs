use crate::db::{add_env_cfg_value, set_config_value, ConfigurationValue, LoadFromDatabase};
use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CAS {
    #[serde(skip)]
    section: String,

    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,

    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,
}

impl Default for CAS {
    fn default() -> Self {
        Self {
            section: "CAS".to_string(),
            base_uri: None,
            client_id: String::new(),
            client_secret: String::new(),
        }
    }
}

impl LoadFromDatabase for CAS {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &ConfigurationValue) -> Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "BaseURI" => self.base_uri = Url::parse(&value).ok(),
                "ClientID" => self.client_id = value,
                "ClientSecret" => self.client_secret = value,
                _ => (),
            }
        }

        Ok(())
    }
}

impl From<CAS> for Vec<ConfigurationValue> {
    fn from(cas: CAS) -> Self {
        let mut cfgs = Vec::new();
        let section: String;

        if cas.section.is_empty() {
            section = "CAS".to_string();
        } else {
            section = cas.section.clone();
        }

        if let Some(base_uri) = cas.base_uri {
            cfgs.push(ConfigurationValue {
                id: None,
                section: Some(section.to_string()),
                key: Some("BaseURI".to_string()),
                value: Some(base_uri.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        cfgs.push(ConfigurationValue {
            id: None,
            section: Some(section.to_string()),
            key: Some("ClientID".to_string()),
            value: Some(cas.client_id.to_string()),
            value_type: Some("string".to_string()),
        });

        cfgs.push(ConfigurationValue {
            id: None,
            section: Some(section.to_string()),
            key: Some("ClientSecret".to_string()),
            value: Some(cas.client_secret.to_string()),
            value_type: Some("string".to_string()),
        });

        cfgs
    }
}

impl CAS {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("CAS Base URI")
            .interact()
            .context("Failed to get Base URI")?;

        let base_uri_id = set_config_value(tx, "CAS", "BaseURI", &base_uri, "string").await?;
        add_env_cfg_value(tx, env_id, base_uri_id).await?;
        self.base_uri = Some(Url::parse(&base_uri)?);

        let client_id = Input::<String>::with_theme(theme)
            .with_prompt("CAS Client ID")
            .interact()
            .context("Failed to get Client ID")?;

        let client_id_id = set_config_value(tx, "CAS", "ClientID", &client_id, "string").await?;
        add_env_cfg_value(tx, env_id, client_id_id).await?;
        self.client_id = client_id;

        let client_secret = Input::<String>::with_theme(theme)
            .with_prompt("CAS Client Secret")
            .interact()
            .context("Failed to get Client Secret")?;

        let client_secret_id =
            set_config_value(tx, "CAS", "ClientSecret", &client_secret, "string").await?;
        add_env_cfg_value(tx, env_id, client_secret_id).await?;
        self.client_secret = client_secret;

        Ok(())
    }
}
