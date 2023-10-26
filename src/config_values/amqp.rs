use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "AMQP")]
pub struct Amqp {
    #[serde(skip)]
    section: String,
    user: String,
    password: String,
    host: String,
    port: Option<u16>,
    vhost: String,
}

// We're implementing default so the section is set.
impl Default for Amqp {
    fn default() -> Self {
        Amqp {
            section: "AMQP".to_string(),
            user: String::new(),
            password: String::new(),
            host: String::new(),
            port: None,
            vhost: String::new(),
        }
    }
}

impl From<Amqp> for Vec<db::ConfigurationValue> {
    fn from(amqp: Amqp) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;
        if !amqp.section.is_empty() {
            section = amqp.section;
        } else {
            section = "AMQP".to_string();
        }

        // Add User configuration.
        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("AMQP.User".to_string()),
            value: Some(amqp.user),
            value_type: Some("string".to_string()),
        });

        // Add password configuration
        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("AMQP.Password".to_string()),
            value: Some(amqp.password),
            value_type: Some("string".to_string()),
        });

        // Add host configuration
        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("AMQP.Host".to_string()),
            value: Some(amqp.host),
            value_type: Some("string".to_string()),
        });

        // Add port configuration
        if let Some(port) = amqp.port {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("AMQP.Port".to_string()),
                value: Some(port.to_string()),
                value_type: Some("int".to_string()),
            });
        }
        // Add vhost configuration
        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("AMQP.Vhost".to_string()),
            value: Some(amqp.vhost),
            value_type: Some("string".to_string()),
        });

        vec
    }
}

impl LoadFromDatabase for Amqp {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "AMQP.User" => self.user = value,
                "AMQP.Password" => self.password = value,
                "AMQP.Host" => self.host = value,
                "AMQP.Port" => self.port = Some(value.parse::<u16>()?),
                "AMQP.Vhost" => self.vhost = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl Amqp {
    pub fn set_section(&mut self, new_section: &str) -> anyhow::Result<()> {
        self.section = new_section.to_string();
        Ok(())
    }

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

        let user_id = set_config_value(tx, &self.section, "AMQP.User", &user, "string").await?;
        add_env_cfg_value(tx, env_id, user_id).await?;
        self.user = user;

        let password_id =
            set_config_value(tx, &self.section, "AMQP.Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        let host_id = set_config_value(tx, &self.section, "AMQP.Host", &host, "string").await?;
        add_env_cfg_value(tx, env_id, host_id).await?;
        self.host = host;

        let port_id =
            set_config_value(tx, &self.section, "AMQP.Port", &port.to_string(), "int").await?;
        add_env_cfg_value(tx, env_id, port_id).await?;
        self.port = Some(port);

        let vhost_id = set_config_value(tx, &self.section, "AMQP.Vhost", &vhost, "string").await?;
        add_env_cfg_value(tx, env_id, vhost_id).await?;
        self.vhost = vhost;

        Ok(())
    }
}
