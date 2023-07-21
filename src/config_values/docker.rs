use crate::db::{add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Docker {
    trusted_registries: Option<Vec<String>>,
    tag: String,
}

impl LoadFromConfiguration for Docker {
    fn get_section(&self) -> String {
        "Docker".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
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
        }
        Ok(())
    }
}

impl Default for Docker {
    fn default() -> Self {
        Docker {
            tag: String::from("latest"),
            trusted_registries: Some(vec![
                String::from("harbor.cyverse.org"),
                String::from("docker.cyverse.org"),
            ]),
        }
    }
}

impl Docker {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
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
