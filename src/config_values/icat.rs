use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "ICAT")]
pub struct Icat {
    host: String,
    port: u16,
    user: String,
    password: String,
}

impl Icat {
    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
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

        self.host = host;
        self.port = port;
        self.user = user;
        self.password = password;

        Ok(())
    }
}
