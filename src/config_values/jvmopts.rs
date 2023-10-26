use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", rename = "JVMOpts")]
pub struct JVMOpts {
    #[serde(skip)]
    section: String,

    high: Option<String>,
    low: Option<String>,

    #[serde(rename = "UI")]
    ui: Option<String>,
}

impl Default for JVMOpts {
    fn default() -> Self {
        JVMOpts {
            section: "JVMOpts".to_string(),
            high: None,
            low: None,
            ui: None,
        }
    }
}

impl LoadFromDatabase for JVMOpts {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "High" => self.high = Some(value),
                "Low" => self.low = Some(value),
                "UI" => self.ui = Some(value),
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<JVMOpts> for Vec<db::ConfigurationValue> {
    fn from(jvmopts: JVMOpts) -> Self {
        let mut cfgs: Vec<db::ConfigurationValue> = Vec::new();

        let section: String;
        if jvmopts.section.is_empty() {
            section = "JVMOpts".to_string();
        } else {
            section = jvmopts.section.clone();
        }

        if let Some(high) = jvmopts.high {
            cfgs.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("High".to_string()),
                value: Some(high.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(low) = jvmopts.low {
            cfgs.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Low".to_string()),
                value: Some(low.to_string()),
                value_type: Some("string".to_string()),
            })
        }

        if let Some(ui) = jvmopts.ui {
            cfgs.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("UI".to_string()),
                value: Some(ui.to_string()),
                value_type: Some("string".to_string()),
            })
        }

        cfgs
    }
}

impl JVMOpts {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let high = Input::<String>::with_theme(theme)
            .with_prompt("High")
            .interact()?;

        let low = Input::<String>::with_theme(theme)
            .with_prompt("Low")
            .interact()?;

        let ui = Input::<String>::with_theme(theme)
            .with_prompt("UI")
            .interact()?;

        let high_id = set_config_value(tx, &self.get_section(), "High", &high, "string").await?;
        let low_id = set_config_value(tx, &self.get_section(), "Low", &low, "string").await?;
        let ui_id = set_config_value(tx, &self.get_section(), "UI", &ui, "string").await?;

        add_env_cfg_value(tx, env_id, high_id).await?;
        add_env_cfg_value(tx, env_id, low_id).await?;
        add_env_cfg_value(tx, env_id, ui_id).await?;

        self.high = Some(high);
        self.low = Some(low);
        self.ui = Some(ui);

        Ok(())
    }
}
