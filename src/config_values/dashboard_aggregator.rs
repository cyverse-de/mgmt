use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
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

impl LoadFromConfiguration for Website {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "URL" => self.url = Url::parse(&value).ok(),
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<Website> for Vec<db::Configuration> {
    fn from(website: Website) -> Self {
        let mut cfgs = Vec::new();
        let section: String;
        if website.section.is_empty() {
            section = "DashboardAggregator".to_string();
        } else {
            section = website.section.clone();
        }

        if let Some(url) = website.url {
            cfgs.push(db::Configuration {
                id: None,
                section: Some(section.clone()),
                key: Some("URL".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }
        cfgs
    }
}

impl Website {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let url = Input::<String>::with_theme(theme)
            .with_prompt("Dashboard Website URL")
            .default("https://cyverse.org".into())
            .interact()?;

        let url_id = set_config_value(tx, "DashboardAggregator", "URL", &url, "string").await?;
        add_env_cfg_value(tx, env_id, url_id).await?;
        self.url = Url::parse(&url).ok();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
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

impl LoadFromConfiguration for DashboardAggregator {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let Some(key) = cfg.key.clone() {
            match key.as_str() {
                "URL" => {
                    let mut ws = Website::default();
                    ws.cfg_set_key(cfg)?;
                    self.website = Some(ws);
                }
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<DashboardAggregator> for Vec<db::Configuration> {
    fn from(da: DashboardAggregator) -> Self {
        let mut cfgs = Vec::new();

        if let Some(website) = da.website {
            cfgs.extend::<Vec<db::Configuration>>(website.into());
        }
        cfgs
    }
}

impl DashboardAggregator {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let mut new_website = Website::default();
        new_website.ask_for_info(tx, theme, env_id).await?;
        self.website = Some(new_website);

        Ok(())
    }
}
