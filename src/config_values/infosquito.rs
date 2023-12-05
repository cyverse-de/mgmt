use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
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
            day_num: None,
            prefix_length: None,
        }
    }
}

impl LoadFromDatabase for Infosquito {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "DayNum" => self.day_num = Some(value.parse::<u8>()?),
            "PrefixLength" => self.prefix_length = Some(value.parse::<u32>()?),
            _ => (),
        }

        Ok(())
    }
}

impl From<Infosquito> for Vec<db::ConfigurationValue> {
    fn from(i: Infosquito) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if i.section.is_empty() {
            section = "Infosquito".to_string();
        } else {
            section = i.section.clone();
        }

        if let Some(day_num) = i.day_num {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DayNum".to_string(),
                value: day_num.to_string(),
                value_type: "int".to_string(),
            });
        }

        if let Some(prefix_length) = i.prefix_length {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "PrefixLength".to_string(),
                value: prefix_length.to_string(),
                value_type: "int".to_string(),
            });
        }

        vec
    }
}

impl Infosquito {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
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
            set_config_value(tx, "Infosquito", "DayNum", &day_num.to_string(), "int").await?;
        add_env_cfg_value(tx, env_id, day_num_id).await?;
        self.day_num = Some(day_num);

        let prefix_length_id = set_config_value(
            tx,
            "Infosquito",
            "PrefixLength",
            &prefix_length.to_string(),
            "int",
        )
        .await?;
        add_env_cfg_value(tx, env_id, prefix_length_id).await?;
        self.prefix_length = Some(prefix_length);

        Ok(())
    }
}
