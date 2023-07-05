use crate::db::{add_env_cfg_value, set_config_value};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "AMQP")]
pub struct Amqp {
    user: String,
    password: String,
    host: String,
    port: u16,
    vhost: String,
}

impl Amqp {
    pub fn ask_for_info(&mut self, theme: &ColorfulTheme, prefix: &str) -> anyhow::Result<()> {
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

        self.user = user;
        self.password = password;
        self.host = host;
        self.port = port;
        self.vhost = vhost;

        Ok(())
    }
}
