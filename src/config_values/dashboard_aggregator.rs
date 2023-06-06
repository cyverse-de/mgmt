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
    fn merge(&self, right: &Website) -> anyhow::Result<Website> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

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
    pub fn merge(&self, right: &DashboardAggregator) -> anyhow::Result<DashboardAggregator> {
        let mut merged: DashboardAggregator = serde_merge::omerge(&self, &right)?;
        if let Some(website) = &self.website {
            if let Some(right_website) = &right.website {
                merged.website = Some(website.merge(right_website)?);
            }
        }
        Ok(merged)
    }

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
