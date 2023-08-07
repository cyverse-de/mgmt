use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Email {
    #[serde(skip)]
    section: String,

    src: String,
    dest: String,

    #[serde(rename = "PermIDRequestDest")]
    perm_id_request_dest: String,

    support_dest: String,
}

impl Default for Email {
    fn default() -> Self {
        Email {
            section: "Email".to_string(),
            src: String::new(),
            dest: String::new(),
            perm_id_request_dest: String::new(),
            support_dest: String::new(),
        }
    }
}

impl LoadFromConfiguration for Email {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Src" => self.src = value,
                "Dest" => self.dest = value,
                "PermIDRequestDest" => self.perm_id_request_dest = value,
                "SupportDest" => self.support_dest = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<Email> for Vec<db::Configuration> {
    fn from(email: Email) -> Vec<db::Configuration> {
        let mut cfgs = Vec::new();
        let section: String;

        if email.section.is_empty() {
            section = "Email".to_string();
        } else {
            section = email.section.clone();
        }

        cfgs.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("Src".to_string()),
            value: Some(email.src),
            value_type: Some("string".to_string()),
        });
        cfgs.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("Dest".to_string()),
            value: Some(email.dest),
            value_type: Some("string".to_string()),
        });
        cfgs.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("PermIDRequestDest".to_string()),
            value: Some(email.perm_id_request_dest),
            value_type: Some("string".to_string()),
        });
        cfgs.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("SupportDest".to_string()),
            value: Some(email.support_dest),
            value_type: Some("string".to_string()),
        });
        cfgs
    }
}

impl Email {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let src = Input::<String>::with_theme(theme)
            .with_prompt("Email Source")
            .interact()?;

        let dest = Input::<String>::with_theme(theme)
            .with_prompt("Email Destination")
            .interact()?;

        let perm_id_default = dest.clone();
        let perm_id_request_dest = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID Request Destination")
            .default(perm_id_default)
            .interact()?;

        let support_dest_default = dest.clone();
        let support_dest = Input::<String>::with_theme(theme)
            .with_prompt("Support Destination")
            .default(support_dest_default)
            .interact()?;

        let src_id = set_config_value(tx, "Email", "Src", &src, "string").await?;
        add_env_cfg_value(tx, env_id, src_id).await?;
        self.src = src;

        let dest_id = set_config_value(tx, "Email", "Dest", &dest, "string").await?;
        add_env_cfg_value(tx, env_id, dest_id).await?;
        self.dest = dest;

        let perm_id_request_dest_id = set_config_value(
            tx,
            "Email",
            "PermIDRequestDest",
            &perm_id_request_dest,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, perm_id_request_dest_id).await?;
        self.perm_id_request_dest = perm_id_request_dest;

        let support_dest_id =
            set_config_value(tx, "Email", "SupportDest", &support_dest, "string").await?;
        add_env_cfg_value(tx, env_id, support_dest_id).await?;
        self.support_dest = support_dest;

        Ok(())
    }
}
