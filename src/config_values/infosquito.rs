use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Infosquito {
    #[serde(skip)]
    section: String,

    day_num: Option<u8>,
    prefix_length: Option<u32>,
}

impl Default for Infosquito {
    fn default() -> Self {
        Infosquito {
            section: "Infosquito".to_string(),
            day_num: Some(4),
            prefix_length: Some(4),
        }
    }
}

impl LoadFromConfiguration for Infosquito {
    fn get_section(&self) -> String {
        self.section.to_string()
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

impl From<Infosquito> for Vec<db::Configuration> {
    fn from(i: Infosquito) -> Vec<db::Configuration> {
        let mut vec: Vec<db::Configuration> = Vec::new();
        let section = i.section.clone();

        vec.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("DayNum".to_string()),
            value: Some(i.day_num.unwrap().to_string()),
            value_type: Some("integer".to_string()),
        });

        vec.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("PrefixLength".to_string()),
            value: Some(i.prefix_length.unwrap().to_string()),
            value_type: Some("integer".to_string()),
        });

        vec
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
