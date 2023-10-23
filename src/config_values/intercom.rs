use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Intercom {
    #[serde(skip)]
    section: String,

    enabled: bool,

    #[serde(rename = "AppID")]
    app_id: String,

    #[serde(rename = "CompanyID")]
    company_id: String,

    company_name: String,
}

// We're manually implementing Default so that we can set the section.
impl Default for Intercom {
    fn default() -> Self {
        Intercom {
            section: "Intercom".to_string(),
            enabled: false,
            app_id: String::new(),
            company_id: String::new(),
            company_name: String::new(),
        }
    }
}

impl From<Intercom> for Vec<db::ConfigurationValue> {
    fn from(i: Intercom) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if i.section.is_empty() {
            section = "Intercom".to_string();
        } else {
            section = i.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Enabled".to_string()),
            value: Some(i.enabled.to_string()),
            value_type: Some("bool".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("AppID".to_string()),
            value: Some(i.app_id),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("CompanyID".to_string()),
            value: Some(i.company_id),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("CompanyName".to_string()),
            value: Some(i.company_name),
            value_type: Some("string".to_string()),
        });

        vec
    }
}

impl LoadFromDatabase for Intercom {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Enabled" => self.enabled = value.parse::<bool>()?,
                "AppID" => self.app_id = value,
                "CompanyID" => self.company_id = value,
                "CompanyName" => self.company_name = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl Intercom {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let enabled = Select::with_theme(theme)
            .with_prompt("Intercom Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        let app_id = Input::<String>::with_theme(theme)
            .with_prompt("Intercom App ID")
            .interact()?;

        let company_id = Input::<String>::with_theme(theme)
            .with_prompt("Intercom Company ID")
            .interact()?;

        let company_name = Input::<String>::with_theme(theme)
            .with_prompt("Intercom Company Name")
            .interact()?;

        let enabled_id = set_config_value(
            tx,
            "Intercom",
            "Enabled",
            &format!("{}", enabled == 0),
            "bool",
        )
        .await?;
        add_env_cfg_value(tx, env_id, enabled_id).await?;
        self.enabled = enabled == 0;

        let app_id_id = set_config_value(tx, "Intercom", "AppID", &app_id, "string").await?;
        add_env_cfg_value(tx, env_id, app_id_id).await?;
        self.app_id = app_id;

        let company_id_id =
            set_config_value(tx, "Intercom", "CompanyID", &company_id, "string").await?;
        add_env_cfg_value(tx, env_id, company_id_id).await?;
        self.company_id = company_id;

        let company_name_id =
            set_config_value(tx, "Intercom", "CompanyName", &company_name, "string").await?;
        add_env_cfg_value(tx, env_id, company_name_id).await?;
        self.company_name = company_name;

        Ok(())
    }
}
