use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Intercom {
    enabled: bool,

    #[serde(rename = "AppID")]
    app_id: String,

    #[serde(rename = "CompanyID")]
    company_id: String,

    company_name: String,
}

impl Intercom {
    pub fn merge(&self, right: &Intercom) -> anyhow::Result<Intercom> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let enabled = Select::with_theme(theme)
            .with_prompt("Intercom Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        let app_id = Input::<String>::with_theme(theme)
            .with_prompt("Intercom App ID")
            .interact()?;

        let company_id = Input::<String>::with_theme(theme)
            .with_prompt("Intercom Company ID")
            .interact()?;

        let company_name = Input::<String>::with_theme(theme)
            .with_prompt("Intercom Company Name")
            .interact()?;

        self.enabled = enabled == 0;
        self.app_id = app_id;
        self.company_id = company_id;
        self.company_name = company_name;

        Ok(())
    }
}