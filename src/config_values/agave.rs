use dialoguer::{theme::ColorfulTheme, Input, Select};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Agave {
    key: String,
    secret: String,

    #[serde(rename = "RedirectURI")]
    redirect_uri: String,

    storage_system: String,

    #[serde(rename = "CallbackBaseURI")]
    callback_base_uri: String,

    read_timeout: Option<u32>,
    enabled: Option<bool>,
    jobs_enabled: Option<bool>,
}

impl Default for Agave {
    fn default() -> Self {
        Agave {
            key: String::new(),
            secret: String::new(),
            redirect_uri: String::new(),
            storage_system: String::new(),
            callback_base_uri: String::new(),
            read_timeout: Some(30000),
            enabled: Some(false),
            jobs_enabled: Some(false),
        }
    }
}

impl Agave {
    pub fn ask_for_info(
        &mut self,
        theme: &ColorfulTheme,
        base_url: &url::Url,
        irods_external: &str,
    ) -> anyhow::Result<()> {
        let df_base_url = base_url.clone().join("/de/agave-cb")?;
        let callback_base_uri = Input::<String>::with_theme(theme)
            .with_prompt("Agave Callback Base URI")
            .default(df_base_url.to_string())
            .interact()?;
        self.callback_base_uri = callback_base_uri;

        let rd_uri = base_url.clone().join("/oauth/callback/agave")?;
        let redirect_uri = Input::<String>::with_theme(theme)
            .with_prompt("Agave Redirect URI")
            .default(rd_uri.to_string())
            .interact()?;
        self.redirect_uri = redirect_uri;

        let agave_key = Input::<String>::with_theme(theme)
            .with_prompt("Agave Key")
            .interact()?;

        self.key = agave_key;

        let secret = Input::<String>::with_theme(theme)
            .with_prompt("Agave Secret")
            .interact()?;

        self.secret = secret;

        let storage_system = Input::<String>::with_theme(theme)
            .with_prompt("Agave Storage System")
            .default(irods_external.into())
            .interact()?;

        self.storage_system = storage_system;
        self.enabled = Some(true);

        let read_timeout = Input::<u32>::with_theme(theme)
            .with_prompt("Agave Read Timeout")
            .default(30000)
            .interact()?;
        self.read_timeout = Some(read_timeout);

        let jobs_enabled = Select::with_theme(theme)
            .with_prompt("Agave Jobs Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;
        self.jobs_enabled = Some(jobs_enabled == 0);

        Ok(())
    }
}
