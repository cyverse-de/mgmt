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
        if let Some(url) = website.url {
            cfgs.push(db::Configuration {
                id: None,
                section: Some(website.section.clone()),
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

    public_group: String,
    website: Option<Website>,
}

impl Default for DashboardAggregator {
    fn default() -> Self {
        DashboardAggregator {
            section: "DashboardAggregator".to_string(),
            public_group: String::new(),
            website: Some(Website::default()),
        }
    }
}

impl LoadFromConfiguration for DashboardAggregator {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "URL" => {
                    let mut ws = Website::default();
                    ws.cfg_set_key(cfg)?;
                    self.website = Some(ws);
                }
                "PublicGroup" => self.public_group = value,
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
        cfgs.push(db::Configuration {
            id: None,
            section: Some(da.section.clone()),
            key: Some("PublicGroup".to_string()),
            value: Some(da.public_group),
            value_type: Some("string".to_string()),
        });
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
        let public_group = Input::<String>::with_theme(theme)
            .with_prompt("Dashboard Public Group")
            .interact()?;
        let public_group_id = set_config_value(
            tx,
            "DashboardAggregator",
            "PublicGroup",
            &public_group,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, 0, public_group_id).await?;
        self.public_group = public_group;

        let mut new_website = Website::default();
        new_website.ask_for_info(tx, theme, env_id).await?;
        self.website = Some(new_website);

        Ok(())
    }
}
