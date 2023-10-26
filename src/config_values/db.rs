use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DatabaseConfig {
    #[serde(skip)]
    section: String,

    pub user: String,
    pub password: String,
    pub host: String,
    port: Option<u32>,
    name: Option<String>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            section: "Database".to_string(),
            user: String::new(),
            password: String::new(),
            host: String::new(),
            port: None,
            name: None,
        }
    }
}

impl LoadFromDatabase for DatabaseConfig {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value), Some(section)) =
            (cfg.key.clone(), cfg.value.clone(), cfg.section.clone())
        {
            self.section = section;

            match key.as_str() {
                "User" => self.user = value,
                "Password" => self.password = value,
                "Host" => self.host = value,
                "Port" => self.port = Some(value.parse::<u32>()?),
                "Name" => self.name = Some(value),
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<DatabaseConfig> for Vec<db::ConfigurationValue> {
    fn from(db_cfg: DatabaseConfig) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if db_cfg.section.is_empty() {
            section = "Database".to_string();
        } else {
            section = db_cfg.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("User".to_string()),
            value: Some(db_cfg.user),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Password".to_string()),
            value: Some(db_cfg.password),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Host".to_string()),
            value: Some(db_cfg.host),
            value_type: Some("string".to_string()),
        });

        if let Some(port) = db_cfg.port {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Port".to_string()),
                value: Some(format!("{}", port.to_string())),
                value_type: Some("int".to_string()),
            });
        }

        if let Some(name) = db_cfg.name {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Name".to_string()),
                value: Some(name),
                value_type: Some("string".to_string()),
            });
        }

        vec
    }
}

impl DatabaseConfig {
    pub fn set_section(&mut self, section: &str) {
        self.section = section.to_string();
    }

    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
        prefix: &str,
        section: &str,
        name: &str,
        host: &str,
        user: &str,
        pass: &str,
    ) -> anyhow::Result<()> {
        let user = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} Database User", prefix))
            .default(user.to_string())
            .interact()?;

        let password = Input::with_theme(theme)
            .with_prompt(format!("{} Database Password", prefix))
            .default(pass.to_string())
            .interact()?;

        let host = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} Database Host", prefix))
            .default(host.to_string())
            .interact()?;

        let port = Input::<u32>::with_theme(theme)
            .with_prompt(format!("{} Database Port", prefix))
            .default(5432)
            .interact()?;

        let name = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} Database Name", prefix))
            .default(name.to_string())
            .interact()?;

        let user_id = set_config_value(tx, section, "User", &user, "string").await?;
        add_env_cfg_value(tx, env_id, user_id).await?;
        self.user = user;

        let password_id = set_config_value(tx, section, "Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        let host_id = set_config_value(tx, section, "Host", &host, "string").await?;
        add_env_cfg_value(tx, env_id, host_id).await?;
        self.host = host;

        let port_id = set_config_value(tx, section, "Port", &port.to_string(), "int").await?;
        add_env_cfg_value(tx, env_id, port_id).await?;
        self.port = Some(port);

        let name_id = set_config_value(tx, section, "Name", &name, "string").await?;
        add_env_cfg_value(tx, env_id, name_id).await?;
        self.name = Some(name);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct QMSDatabaseConfig {
    #[serde(skip)]
    section: String,

    pub user: String,
    pub password: String,
    pub host: String,
    port: Option<u32>,
    name: Option<String>,
    automigrate: Option<bool>,
    reinitialize: Option<bool>,
}

impl LoadFromDatabase for QMSDatabaseConfig {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "User" => self.user = value,
                "Password" => self.password = value,
                "Host" => self.host = value,
                "Port" => self.port = Some(value.parse::<u32>()?),
                "Name" => self.name = Some(value),
                "Automigrate" => self.automigrate = Some(value.parse::<bool>()?),
                "Reinitialize" => self.reinitialize = Some(value.parse::<bool>()?),
                _ => (),
            }
        }
        Ok(())
    }
}

impl Default for QMSDatabaseConfig {
    fn default() -> Self {
        QMSDatabaseConfig {
            section: "QMSDB".to_string(),
            user: String::new(),
            password: String::new(),
            host: String::new(),
            port: None,
            name: None,
            automigrate: None,
            reinitialize: None,
        }
    }
}

impl From<QMSDatabaseConfig> for Vec<db::ConfigurationValue> {
    fn from(qmsdb: QMSDatabaseConfig) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if qmsdb.section.is_empty() {
            section = "QMSDB".to_string();
        } else {
            section = qmsdb.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("User".to_string()),
            value: Some(qmsdb.user),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Password".to_string()),
            value: Some(qmsdb.password),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Host".to_string()),
            value: Some(qmsdb.host),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Port".to_string()),
            value: Some(qmsdb.port.unwrap_or_default().to_string()),
            value_type: Some("int".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Name".to_string()),
            value: qmsdb.name,
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Automigrate".to_string()),
            value: Some(qmsdb.automigrate.unwrap_or_default().to_string()),
            value_type: Some("bool".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Reinitialize".to_string()),
            value: Some(qmsdb.reinitialize.unwrap_or_default().to_string()),
            value_type: Some("bool".to_string()),
        });

        vec
    }
}

impl QMSDatabaseConfig {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
        name: &str,
        host: &str,
        user: &str,
        pass: &str,
    ) -> anyhow::Result<()> {
        let user = Input::<String>::with_theme(theme)
            .with_prompt("QMS Database User")
            .default(user.to_string())
            .interact()?;

        let password = Input::with_theme(theme)
            .with_prompt("QMS Database Password")
            .default(pass.to_string())
            .interact()?;

        let host = Input::<String>::with_theme(theme)
            .with_prompt("QMS Database Host")
            .default(host.to_string())
            .interact()?;

        let port = Input::<u32>::with_theme(theme)
            .with_prompt("QMS Database Port")
            .default(5432)
            .interact()?;

        let name = Input::<String>::with_theme(theme)
            .with_prompt("QMS Database Name")
            .default(name.to_string())
            .interact()?;

        let automigrate = Select::with_theme(theme)
            .with_prompt("QMS Database Automigrate")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        let reinitialize = Select::with_theme(theme)
            .with_prompt("QMS Database Reinitialize")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        let user_id = set_config_value(tx, "QMSDB", "User", &user, "string").await?;
        add_env_cfg_value(tx, env_id, user_id).await?;
        self.user = user;

        let password_id = set_config_value(tx, "QMSDB", "Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        let host_id = set_config_value(tx, "QMSDB", "Host", &host, "string").await?;
        add_env_cfg_value(tx, env_id, host_id).await?;
        self.host = host;

        let port_id = set_config_value(tx, "QMSDB", "Port", &port.to_string(), "int").await?;
        add_env_cfg_value(tx, env_id, port_id).await?;
        self.port = Some(port);

        let name_id = set_config_value(tx, "QMSDB", "Name", &name, "string").await?;
        add_env_cfg_value(tx, env_id, name_id).await?;
        self.name = Some(name);

        let automigrate_id = set_config_value(
            tx,
            "QMSDB",
            "Automigrate",
            &format!("{}", automigrate == 0),
            "bool",
        )
        .await?;
        add_env_cfg_value(tx, env_id, automigrate_id).await?;
        self.automigrate = Some(automigrate == 0);

        let reinitialize_id = set_config_value(
            tx,
            "QMSDB",
            "Reinitialize",
            &format!("{}", reinitialize == 0),
            "bool",
        )
        .await?;
        add_env_cfg_value(tx, env_id, reinitialize_id).await?;
        self.reinitialize = Some(reinitialize == 0);

        Ok(())
    }
}
