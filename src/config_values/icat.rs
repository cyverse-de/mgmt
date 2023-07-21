use crate::db::{add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "ICAT")]
pub struct Icat {
    host: String,
    port: u16,
    user: String,
    password: String,
}

impl LoadFromConfiguration for Icat {
    fn get_section(&self) -> String {
        "ICAT".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Host" => self.host = value,
                "Port" => self.port = value.parse::<u16>()?,
                "User" => self.user = value,
                "Password" => self.password = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl Icat {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let host = Input::<String>::with_theme(theme)
            .with_prompt("ICAT Host")
            .interact()?;

        let port = Input::<u16>::with_theme(theme)
            .with_prompt("ICAT Port")
            .default(1247)
            .interact()?;

        let user = Input::<String>::with_theme(theme)
            .with_prompt("ICAT User")
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("ICAT Password")
            .interact()?;

        let host_id = set_config_value(tx, "ICAT", "Host", &host, "string").await?;
        add_env_cfg_value(tx, env_id, host_id).await?;
        self.host = host;

        let port_id = set_config_value(tx, "ICAT", "Port", &port.to_string(), "integer").await?;
        add_env_cfg_value(tx, env_id, port_id).await?;
        self.port = port;

        let user_id = set_config_value(tx, "ICAT", "User", &user, "string").await?;
        add_env_cfg_value(tx, env_id, user_id).await?;
        self.user = user;

        let password_id = set_config_value(tx, "ICAT", "Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        Ok(())
    }
}
