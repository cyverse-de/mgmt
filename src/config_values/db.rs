use crate::db::{add_env_cfg_value, set_config_value};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    port: u32,
    name: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            user: String::new(),
            password: String::new(),
            host: String::new(),
            port: 5432,
            name: String::new(),
        }
    }
}

impl DatabaseConfig {
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

        let port_id = set_config_value(tx, section, "Port", &port.to_string(), "integer").await?;
        add_env_cfg_value(tx, env_id, port_id).await?;
        self.port = port;

        let name_id = set_config_value(tx, section, "Name", &name, "string").await?;
        add_env_cfg_value(tx, env_id, name_id).await?;
        self.name = name;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct QMSDatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    port: Option<u32>,
    name: Option<String>,
    automigrate: Option<bool>,
    reinitialize: Option<bool>,
}

impl Default for QMSDatabaseConfig {
    fn default() -> Self {
        QMSDatabaseConfig {
            user: String::new(),
            password: String::new(),
            host: String::new(),
            port: Some(5432),
            name: Some(String::from("qms")),
            automigrate: Some(false),
            reinitialize: Some(false),
        }
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

        let port_id = set_config_value(tx, "QMSDB", "Port", &port.to_string(), "integer").await?;
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
            "boolean",
        )
        .await?;
        add_env_cfg_value(tx, env_id, automigrate_id).await?;
        self.automigrate = Some(automigrate == 0);

        let reinitialize_id = set_config_value(
            tx,
            "QMSDB",
            "Reinitialize",
            &format!("{}", reinitialize == 0),
            "boolean",
        )
        .await?;
        add_env_cfg_value(tx, env_id, reinitialize_id).await?;
        self.reinitialize = Some(reinitialize == 0);

        Ok(())
    }
}
