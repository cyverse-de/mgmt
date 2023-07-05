use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};

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
    pub fn ask_for_info(
        &mut self,
        theme: &ColorfulTheme,
        prefix: &str,
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

        self.user = user;
        self.password = password;
        self.host = host;
        self.port = port;
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
    pub fn ask_for_info(
        &mut self,
        theme: &ColorfulTheme,
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

        self.user = user;
        self.password = password;
        self.host = host;
        self.port = Some(port);
        self.name = Some(name);
        self.automigrate = Some(automigrate == 0);
        self.reinitialize = Some(reinitialize == 0);

        Ok(())
    }
}
