use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "ICAT")]
pub struct Icat {
    #[serde(skip)]
    section: String,

    host: String,
    port: Option<u16>,
    user: String,
    password: String,
}

// We're implementing default so the section is set.
impl Default for Icat {
    fn default() -> Self {
        Icat {
            section: "ICAT".to_string(),
            host: String::new(),
            port: None,
            user: String::new(),
            password: String::new(),
        }
    }
}

impl LoadFromDatabase for Icat {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "Host" => self.host = value,
            "Port" => self.port = Some(value.parse::<u16>()?),
            "User" => self.user = value,
            "Password" => self.password = value,
            _ => (),
        }

        Ok(())
    }
}

impl From<Icat> for Vec<db::ConfigurationValue> {
    fn from(i: Icat) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if i.section.is_empty() {
            section = "ICAT".to_string();
        } else {
            section = i.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Host".to_string(),
            value: i.host,
            value_type: "string".to_string(),
        });

        if let Some(port) = i.port {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Port".to_string(),
                value: port.to_string(),
                value_type: "int".to_string(),
            });
        }

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "User".to_string(),
            value: i.user,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Password".to_string(),
            value: i.password,
            value_type: "string".to_string(),
        });

        vec
    }
}

impl Icat {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
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

        let password = Input::<String>::with_theme(theme)
            .with_prompt("ICAT Password")
            .interact()?;

        let host_id = set_config_value(tx, "ICAT", "Host", &host, "string").await?;
        add_env_cfg_value(tx, env_id, host_id).await?;
        self.host = host;

        let port_id = set_config_value(tx, "ICAT", "Port", &port.to_string(), "int").await?;
        add_env_cfg_value(tx, env_id, port_id).await?;
        self.port = Some(port);

        let user_id = set_config_value(tx, "ICAT", "User", &user, "string").await?;
        add_env_cfg_value(tx, env_id, user_id).await?;
        self.user = user;

        let password_id = set_config_value(tx, "ICAT", "Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        Ok(())
    }
}
