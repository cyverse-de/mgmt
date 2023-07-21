use crate::db::{add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Infosquito {
    day_num: Option<u8>,
    prefix_length: Option<u32>,
}

impl Default for Infosquito {
    fn default() -> Self {
        Infosquito {
            day_num: Some(4),
            prefix_length: Some(4),
        }
    }
}

impl LoadFromConfiguration for Infosquito {
    fn get_section(&self) -> String {
        "Infosquito".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "DayNum" => self.day_num = Some(value.parse::<u8>()?),
                "PrefixLength" => self.prefix_length = Some(value.parse::<u32>()?),
                _ => (),
            }
        }
        Ok(())
    }
}

impl Infosquito {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let day_num = Input::<u8>::with_theme(theme)
            .with_prompt("Infosquito Day Number")
            .default(4)
            .interact()?;

        let prefix_length = Input::<u32>::with_theme(theme)
            .with_prompt("Infosquito Prefix Length")
            .default(4)
            .interact()?;

        let day_num_id =
            set_config_value(tx, "Infosquito", "DayNum", &day_num.to_string(), "integer").await?;
        add_env_cfg_value(tx, env_id, day_num_id).await?;
        self.day_num = Some(day_num);

        let prefix_length_id = set_config_value(
            tx,
            "Infosquito",
            "PrefixLength",
            &prefix_length.to_string(),
            "integer",
        )
        .await?;
        add_env_cfg_value(tx, env_id, prefix_length_id).await?;
        self.prefix_length = Some(prefix_length);

        Ok(())
    }
}
