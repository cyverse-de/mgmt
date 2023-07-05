use crate::db::{add_env_cfg_value, set_config_value};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
pub struct Website {
    #[serde(rename = "URL")]
    url: Option<url::Url>,
}

impl Default for Website {
    fn default() -> Self {
        Website {
            url: Url::parse("https://cyverse.org").ok(),
        }
    }
}

impl Website {
    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let url = Input::<String>::with_theme(theme)
            .with_prompt("Dashboard Website URL")
            .default("https://cyverse.org".into())
            .interact()?;

        self.url = Url::parse(&url).ok();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DashboardAggregator {
    public_group: String,
    website: Option<Website>,
}

impl DashboardAggregator {
    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let public_group = Input::<String>::with_theme(theme)
            .with_prompt("Dashboard Public Group")
            .interact()?;

        self.public_group = public_group;
        let mut new_website = Website::default();
        new_website.ask_for_info(theme)?;
        self.website = Some(new_website);

        Ok(())
    }
}
