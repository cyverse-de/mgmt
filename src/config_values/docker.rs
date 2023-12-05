use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Docker {
    #[serde(skip)]
    section: String,

    trusted_registries: Option<Vec<String>>,
    tag: String,
}

impl LoadFromDatabase for Docker {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();
        match key.as_str() {
            "Tag" => self.tag = value,
            "TrustedRegistries" => {
                self.trusted_registries = Some(
                    value
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>(),
                )
            }
            _ => (),
        }
        Ok(())
    }
}

impl From<Docker> for Vec<db::ConfigurationValue> {
    fn from(docker: Docker) -> Vec<db::ConfigurationValue> {
        let mut cfgs = Vec::new();
        let section: String;

        if docker.section.is_empty() {
            section = "Docker".to_string();
        } else {
            section = docker.section.clone();
        }

        if let Some(trusted_registries) = docker.trusted_registries {
            cfgs.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "TrustedRegistries".to_string(),
                value: trusted_registries.join(","),
                value_type: "string".to_string(),
            });
        }

        cfgs.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Tag".to_string(),
            value: docker.tag,
            value_type: "string".to_string(),
        });

        cfgs
    }
}

impl Default for Docker {
    fn default() -> Self {
        Docker {
            section: "Docker".to_string(),
            tag: String::new(),
            trusted_registries: None,
        }
    }
}

impl Docker {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let tag = Input::<String>::with_theme(theme)
            .with_prompt("Docker Tag")
            .default("latest".into())
            .interact()?;
        let tag_id = set_config_value(tx, "Docker", "Tag", &tag, "string").await?;
        add_env_cfg_value(tx, env_id, tag_id).await?;
        self.tag = tag;

        let trusted_registries = Input::<String>::with_theme(theme)
            .with_prompt("Docker Trusted Registries")
            .default("harbor.cyverse.org,docker.cyverse.org".into())
            .interact()?;
        let trusted_registries_id = set_config_value(
            tx,
            "Docker",
            "TrustedRegistries",
            &trusted_registries,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, trusted_registries_id).await?;
        self.trusted_registries = Some(
            trusted_registries
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );

        Ok(())
    }
}
