use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use sqlx::{Postgres, Transaction};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Agave {
    #[serde(skip)]
    section: String,

    key: String,
    secret: String,

    #[serde(rename = "RedirectURI")]
    redirect_uri: String,

    storage_system: String,

    #[serde(rename = "CallbackBaseURI")]
    callback_base_uri: String,

    read_timeout: Option<u32>,
    enabled: Option<bool>,
    jobs_enabled: Option<bool>,
}

impl Default for Agave {
    fn default() -> Self {
        Agave {
            section: "Agave".to_string(),
            key: String::new(),
            secret: String::new(),
            redirect_uri: String::new(),
            storage_system: String::new(),
            callback_base_uri: String::new(),
            read_timeout: Some(30000),
            enabled: Some(false),
            jobs_enabled: Some(false),
        }
    }
}

impl From<Agave> for Vec<db::ConfigurationValue> {
    fn from(agave: Agave) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;
        if agave.section.is_empty() {
            section = "Agave".to_string();
        } else {
            section = agave.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Key".to_string(),
            value: agave.key,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Secret".to_string(),
            value: agave.secret,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "RedirectURI".to_string(),
            value: agave.redirect_uri,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "StorageSystem".to_string(),
            value: agave.storage_system,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "CallbackBaseURI".to_string(),
            value: agave.callback_base_uri,
            value_type: "string".to_string(),
        });

        if let Some(rt) = agave.read_timeout {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "ReadTimeout".to_string(),
                value: format!("{}", rt),
                value_type: "int".to_string(),
            });
        }

        if let Some(enabled) = agave.enabled {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Enabled".to_string(),
                value: format!("{}", enabled),
                value_type: "bool".to_string(),
            });
        }

        if let Some(jobs_enabled) = agave.jobs_enabled {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "JobsEnabled".to_string(),
                value: format!("{}", jobs_enabled),
                value_type: "bool".to_string(),
            });
        }

        vec
    }
}

impl LoadFromDatabase for Agave {
    fn get_section(&self) -> String {
        self.section.clone()
    }

    fn cfg_set_key(&mut self, cfg: &db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "Key" => self.key = value,
            "Secret" => self.secret = value,
            "RedirectURI" => self.redirect_uri = value,
            "StorageSystem" => self.storage_system = value,
            "CallbackBaseURI" => self.callback_base_uri = value,
            "ReadTimeout" => self.read_timeout = Some(value.parse::<u32>()?),
            "Enabled" => self.enabled = Some(value.parse::<bool>()?),
            "JobsEnabled" => self.jobs_enabled = Some(value.parse::<bool>()?),
            _ => (),
        }
        Ok(())
    }
}

impl Agave {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
        base_url: &url::Url,
        irods_external: &str,
    ) -> anyhow::Result<()> {
        let df_base_url = base_url.clone().join("/de/agave-cb")?;
        let callback_base_uri = Input::<String>::with_theme(theme)
            .with_prompt("Agave Callback Base URI")
            .default(df_base_url.to_string())
            .interact()?;
        let cbu_id =
            set_config_value(tx, "Agave", "CallbackBaseURI", &callback_base_uri, "string").await?;
        add_env_cfg_value(tx, env_id, cbu_id).await?;
        self.callback_base_uri = callback_base_uri;

        let rd_uri = base_url.clone().join("/oauth/callback/agave")?;
        let redirect_uri = Input::<String>::with_theme(theme)
            .with_prompt("Agave Redirect URI")
            .default(rd_uri.to_string())
            .interact()?;
        let rdu_id = set_config_value(tx, "Agave", "RedirectURI", &redirect_uri, "string").await?;
        add_env_cfg_value(tx, env_id, rdu_id).await?;
        self.redirect_uri = redirect_uri;

        let agave_key = Input::<String>::with_theme(theme)
            .with_prompt("Agave Key")
            .interact()?;
        let key_id = set_config_value(tx, "Agave", "Key", &agave_key, "string").await?;
        add_env_cfg_value(tx, env_id, key_id).await?;
        self.key = agave_key;

        let secret = Input::<String>::with_theme(theme)
            .with_prompt("Agave Secret")
            .interact()?;
        let secret_id = set_config_value(tx, "Agave", "Secret", &secret, "string").await?;
        add_env_cfg_value(tx, env_id, secret_id).await?;
        self.secret = secret;

        let storage_system = Input::<String>::with_theme(theme)
            .with_prompt("Agave Storage System")
            .default(irods_external.into())
            .interact()?;
        let ss_id =
            set_config_value(tx, "Agave", "StorageSystem", &storage_system, "string").await?;
        add_env_cfg_value(tx, env_id, ss_id).await?;
        self.storage_system = storage_system;

        let enabled_id = set_config_value(tx, "Agave", "Enabled", "true", "bool").await?;
        add_env_cfg_value(tx, env_id, enabled_id).await?;
        self.enabled = Some(true);

        let read_timeout = Input::<u32>::with_theme(theme)
            .with_prompt("Agave Read Timeout")
            .default(30000)
            .interact()?;
        let rt_id = set_config_value(
            tx,
            "Agave",
            "ReadTimeout",
            &format!("{}", read_timeout),
            "int",
        )
        .await?;
        add_env_cfg_value(tx, env_id, rt_id).await?;
        self.read_timeout = Some(read_timeout);

        let jobs_enabled = Select::with_theme(theme)
            .with_prompt("Agave Jobs Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;
        let je_id = set_config_value(
            tx,
            "Agave",
            "JobsEnabled",
            &format!("{}", jobs_enabled == 0),
            "bool",
        )
        .await?;
        add_env_cfg_value(tx, env_id, je_id).await?;
        self.jobs_enabled = Some(jobs_enabled == 0);

        Ok(())
    }
}
