use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

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
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "High" => self.high = Some(value),
            "Low" => self.low = Some(value),
            "UI" => self.ui = Some(value),
            _ => (),
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
                id: 0,
                section: section.clone(),
                key: "High".to_string(),
                value: high.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(low) = jvmopts.low {
            cfgs.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Low".to_string(),
                value: low.to_string(),
                value_type: "string".to_string(),
            })
        }

        if let Some(ui) = jvmopts.ui {
            cfgs.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "UI".to_string(),
                value: ui.to_string(),
                value_type: "string".to_string(),
            })
        }

        cfgs
    }
}

impl JVMOpts {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let high = Input::<String>::with_theme(theme)
            .with_prompt("High")
            .default(String::from("-Xmx1G -Dlog4j2.formatMsgNoLookups=true"))
            .interact()?;

        let low = Input::<String>::with_theme(theme)
            .with_prompt("Low")
            .default(String::from("-Xmx512M -Dlog4j2.formatMsgNoLookups=true"))
            .interact()?;

        let ui = Input::<String>::with_theme(theme)
            .with_prompt("UI")
            .default(String::from("--Xmx1G -Djava.net.preferIPv4Stack=true"))
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
