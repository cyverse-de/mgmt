use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "AMQP")]
pub struct Amqp {
    #[serde(skip)]
    section: String,
    user: String,
    password: String,
    host: String,
    port: u16,
    vhost: String,
}

impl From<Amqp> for Vec<db::Configuration> {
    fn from(amqp: Amqp) -> Vec<db::Configuration> {
        let mut vec: Vec<db::Configuration> = Vec::new();
        let section = Some(amqp.section.to_string());

        // Add User configuration.
        vec.push(db::Configuration {
            id: None,
            section: section.clone(),
            key: Some("User".to_string()),
            value: Some(amqp.user),
            value_type: Some("string".to_string()),
        });

        // Add password configuration
        vec.push(db::Configuration {
            id: None,
            section: section.clone(),
            key: Some("Password".to_string()),
            value: Some(amqp.password),
            value_type: Some("string".to_string()),
        });

        // Add host configuration
        vec.push(db::Configuration {
            id: None,
            section: section.clone(),
            key: Some("Host".to_string()),
            value: Some(amqp.host),
            value_type: Some("string".to_string()),
        });

        // Add port configuration
        vec.push(db::Configuration {
            id: None,
            section: section.clone(),
            key: Some("Port".to_string()),
            value: Some(amqp.port.to_string()),
            value_type: Some("integer".to_string()),
        });

        // Add vhost configuration
        vec.push(db::Configuration {
            id: None,
            section: section.clone(),
            key: Some("Vhost".to_string()),
            value: Some(amqp.vhost),
            value_type: Some("string".to_string()),
        });

        vec
    }
}

impl LoadFromConfiguration for Amqp {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "User" => self.user = value,
                "Password" => self.password = value,
                "Host" => self.host = value,
                "Port" => self.port = value.parse::<u16>()?,
                "Vhost" => self.vhost = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl Amqp {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
        prefix: &str,
    ) -> anyhow::Result<()> {
        let user = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} AMQP User", prefix))
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt(format!("{} AMQP Password", prefix))
            .interact()?;

        let host = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} AMQP Host", prefix))
            .interact()?;

        let port = Input::<u16>::with_theme(theme)
            .with_prompt(format!("{} AMQP Port", prefix))
            .default(5672)
            .interact()?;

        let vhost = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} AMQP VHost", prefix))
            .interact()?;

        let user_id = set_config_value(tx, "AMQP", "User", &user, "string").await?;
        add_env_cfg_value(tx, env_id, user_id).await?;
        self.user = user;

        let password_id = set_config_value(tx, "AMQP", "Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        let host_id = set_config_value(tx, "AMQP", "Host", &host, "string").await?;
        add_env_cfg_value(tx, env_id, host_id).await?;
        self.host = host;

        let port_id = set_config_value(tx, "AMQP", "Port", &port.to_string(), "integer").await?;
        add_env_cfg_value(tx, env_id, port_id).await?;
        self.port = port;

        let vhost_id = set_config_value(tx, "AMQP", "Vhost", &vhost, "string").await?;
        add_env_cfg_value(tx, env_id, vhost_id).await?;
        self.vhost = vhost;

        Ok(())
    }
}
