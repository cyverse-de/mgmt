use crate::db::{add_env_cfg_value, set_config_value};
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

impl Infosquito {
    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let day_num = Input::<u8>::with_theme(theme)
            .with_prompt("Infosquito Day Number")
            .default(4)
            .interact()?;

        let prefix_length = Input::<u32>::with_theme(theme)
            .with_prompt("Infosquito Prefix Length")
            .default(4)
            .interact()?;

        self.day_num = Some(day_num);
        self.prefix_length = Some(prefix_length);

        Ok(())
    }
}
