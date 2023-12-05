use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Website {
    #[serde(skip)]
    section: String,

    #[serde(rename = "URL")]
    url: Option<url::Url>,
}

impl Default for Website {
    fn default() -> Self {
        Website {
            section: "DashboardAggregator".to_string(),
            url: Url::parse("https://cyverse.org").ok(),
        }
    }
}

impl LoadFromDatabase for Website {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();
        match key.as_str() {
            "URL" => self.url = Url::parse(&value).ok(),
            _ => (),
        }
        Ok(())
    }
}

impl From<Website> for Vec<db::ConfigurationValue> {
    fn from(website: Website) -> Self {
        let mut cfgs = Vec::new();
        let section: String;
        if website.section.is_empty() {
            section = "DashboardAggregator".to_string();
        } else {
            section = website.section.clone();
        }

        if let Some(url) = website.url {
            cfgs.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "URL".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }
        cfgs
    }
}

impl Website {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let url = Input::<String>::with_theme(theme)
            .with_prompt("Dashboard Website URL")
            .default("https://cyverse.org".into())
            .interact()?;

        let url_id =
            set_config_value(tx, "DashboardAggregator", "Website.URL", &url, "string").await?;
        add_env_cfg_value(tx, env_id, url_id).await?;
        self.url = Url::parse(&url).ok();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DashboardAggregator {
    #[serde(skip)]
    section: String,

    website: Option<Website>,
}

impl Default for DashboardAggregator {
    fn default() -> Self {
        DashboardAggregator {
            section: "DashboardAggregator".to_string(),
            website: Some(Website::default()),
        }
    }
}

impl LoadFromDatabase for DashboardAggregator {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        match key.as_str() {
            "Website.URL" => {
                let mut ws = Website::default();
                ws.cfg_set_key(cfg)?;
                self.website = Some(ws);
            }
            _ => (),
        }
        Ok(())
    }
}

impl From<DashboardAggregator> for Vec<db::ConfigurationValue> {
    fn from(da: DashboardAggregator) -> Self {
        let mut cfgs = Vec::new();

        if let Some(website) = da.website {
            cfgs.extend::<Vec<db::ConfigurationValue>>(website.into());
        }
        cfgs
    }
}

impl DashboardAggregator {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let mut new_website = Website::default();
        new_website.ask_for_info(tx, theme, env_id).await?;
        self.website = Some(new_website);

        Ok(())
    }
}
