use crate::db::{add_env_cfg_value, set_config_value};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Email {
    src: String,
    dest: String,

    #[serde(rename = "PermIDRequestDest")]
    perm_id_request_dest: String,

    support_dest: String,
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
