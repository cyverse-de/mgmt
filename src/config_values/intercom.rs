use crate::db::{add_env_cfg_value, set_config_value};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Intercom {
    enabled: bool,

    #[serde(rename = "AppID")]
    app_id: String,

    #[serde(rename = "CompanyID")]
    company_id: String,

    company_name: String,
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
