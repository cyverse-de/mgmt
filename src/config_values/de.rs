use crate::config_values::amqp::Amqp;
use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DESubscriptions {
    #[serde(skip)]
    section: String,

    #[serde(rename = "CheckoutURL")]
    checkout_url: Option<Url>,

    enforce: bool,
}

impl LoadFromDatabase for DESubscriptions {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Subscriptions.CheckoutURL" => self.checkout_url = Url::parse(&value).ok(),
                "Subscriptions.Enforce" => self.enforce = value.parse::<bool>()?,
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<DESubscriptions> for Vec<db::ConfigurationValue> {
    fn from(subs: DESubscriptions) -> Self {
        let mut cfgs = Vec::new();
        let section: String;

        if subs.section.is_empty() {
            section = "DE".to_string();
        } else {
            section = subs.section.clone();
        }

        if let Some(url) = subs.checkout_url {
            cfgs.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Subscriptions.CheckoutURL".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }
        cfgs.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Subscriptions.Enforce".to_string()),
            value: Some(subs.enforce.to_string()),
            value_type: Some("bool".to_string()),
        });
        cfgs
    }
}

impl DESubscriptions {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let checkout_url = Input::<String>::with_theme(theme)
            .with_prompt("Subscriptions Checkout URL")
            .default("https://cyverse-subscription.phoenixbioinformatics.org".into())
            .interact()?;

        let checkout_url_id = set_config_value(
            tx,
            "DE",
            "Subscriptions.CheckoutURL",
            &checkout_url,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, checkout_url_id).await?;
        self.checkout_url = Url::parse(&checkout_url).ok();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DECoge {
    #[serde(skip)]
    section: String,

    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,
}

impl DECoge {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("CoGe Base URI")
            .default("https://genomevolution.org/coge/api/v1".into())
            .interact()?;

        let base_uri_id = set_config_value(tx, "DE", "Coge.BaseURI", &base_uri, "string").await?;
        add_env_cfg_value(tx, env_id, base_uri_id).await?;
        self.base_uri = Url::parse(&base_uri).ok();

        Ok(())
    }
}

impl LoadFromDatabase for DECoge {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Coge.BaseURI" => self.base_uri = Url::parse(&value).ok(),
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<DECoge> for Vec<db::ConfigurationValue> {
    fn from(coge: DECoge) -> Self {
        let mut cfgs = Vec::new();
        let section: String;

        if coge.section.is_empty() {
            section = "DE".to_string();
        } else {
            section = coge.section.clone();
        }

        if let Some(url) = coge.base_uri {
            cfgs.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Coge.BaseURI".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }
        cfgs
    }
}

impl Default for DECoge {
    fn default() -> Self {
        DECoge {
            section: "DE".to_string(),
            base_uri: Url::parse("https://genomevolution.org/coge/api/v1").ok(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DETools {
    #[serde(skip)]
    section: String,

    admin: DEToolsAdmin,
}

impl LoadFromDatabase for DETools {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Tools.Admin.MaxCpuLimit" => self.admin.max_cpu_limit = Some(value.parse()?),
                "Tools.Admin.MaxMemoryLimit" => self.admin.max_memory_limit = Some(value.parse()?),
                "Tools.Admin.MaxDiskLimit" => self.admin.max_disk_limit = Some(value.parse()?),
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<DETools> for Vec<db::ConfigurationValue> {
    fn from(tools: DETools) -> Self {
        let mut cfgs = Vec::new();
        let section: String;

        if tools.section.is_empty() {
            section = "DE".to_string();
        } else {
            section = tools.section.clone();
        }

        if let Some(max_cpu_limit) = tools.admin.max_cpu_limit {
            cfgs.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Tools.Admin.MaxCpuLimit".to_string()),
                value: Some(max_cpu_limit.to_string()),
                value_type: Some("int".to_string()),
            });
        }
        if let Some(max_memory_limit) = tools.admin.max_memory_limit {
            cfgs.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Tools.Admin.MaxMemoryLimit".to_string()),
                value: Some(max_memory_limit.to_string()),
                value_type: Some("int".to_string()),
            });
        }
        if let Some(max_disk_limit) = tools.admin.max_disk_limit {
            cfgs.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Tools.Admin.MaxDiskLimit".to_string()),
                value: Some(max_disk_limit.to_string()),
                value_type: Some("int".to_string()),
            });
        }
        cfgs
    }
}

impl DETools {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let max_cpu_limit = Input::<u32>::with_theme(theme)
            .with_prompt("Max CPU Limit")
            .default(24)
            .interact()?;

        let max_cpu_limit_id = set_config_value(
            tx,
            "DE",
            "Tools.Admin.MaxCpuLimit",
            &max_cpu_limit.to_string(),
            "int",
        )
        .await?;
        add_env_cfg_value(tx, env_id, max_cpu_limit_id).await?;
        self.admin.max_cpu_limit = Some(max_cpu_limit);

        let max_memory_limit = Input::<u64>::with_theme(theme)
            .with_prompt("Max Memory Limit")
            .default(75161927680)
            .interact()?;

        let max_memory_limit_id = set_config_value(
            tx,
            "DE",
            "Tools.Admin.MaxMemoryLimit",
            &max_memory_limit.to_string(),
            "int",
        )
        .await?;
        add_env_cfg_value(tx, env_id, max_memory_limit_id).await?;
        self.admin.max_memory_limit = Some(max_memory_limit);

        let max_disk_limit = Input::<u64>::with_theme(theme)
            .with_prompt("Max Disk Limit")
            .default(1099511627776)
            .interact()?;

        let max_disk_limit_id = set_config_value(
            tx,
            "DE",
            "Tools.Admin.MaxDiskLimit",
            &max_disk_limit.to_string(),
            "int",
        )
        .await?;
        add_env_cfg_value(tx, env_id, max_disk_limit_id).await?;
        self.admin.max_disk_limit = Some(max_disk_limit);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DEToolsAdmin {
    max_cpu_limit: Option<u32>,
    max_memory_limit: Option<u64>,
    max_disk_limit: Option<u64>,
}

impl Default for DEToolsAdmin {
    fn default() -> Self {
        DEToolsAdmin {
            max_cpu_limit: Some(24),
            max_memory_limit: Some(75161927680),
            max_disk_limit: Some(1099511627776),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    #[serde(skip)]
    section: String,

    #[serde(rename = "FAQ")]
    faq: String,

    description: String,
}

impl Default for Info {
    fn default() -> Self {
        Info {
            section: "DE".to_string(),
            faq: "https://wiki.cyverse.org/wiki/display/DEmanual/FAQ".to_string(),
            description: "CyVerse Discovery Environment".to_string(),
        }
    }
}

impl LoadFromDatabase for Info {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Info.FAQ" => self.faq = value,
                "Info.Description" => self.description = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<Info> for Vec<db::ConfigurationValue> {
    fn from(info: Info) -> Self {
        let mut cfgs = Vec::new();
        let section: String;

        if info.section.is_empty() {
            section = "DE".to_string();
        } else {
            section = info.section.clone();
        }

        cfgs.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Info.FAQ".to_string()),
            value: Some(info.faq),
            value_type: Some("string".to_string()),
        });
        cfgs.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Info.Description".to_string()),
            value: Some(info.description),
            value_type: Some("string".to_string()),
        });
        cfgs
    }
}

impl Info {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let faq = Input::<String>::with_theme(theme)
            .with_prompt("Info.FAQ")
            .default("https://wiki.cyverse.org/wiki/display/DEmanual/FAQ".into())
            .interact()?;

        let faq_id = set_config_value(tx, "DE", "Info.FAQ", &faq, "string").await?;
        add_env_cfg_value(tx, env_id, faq_id).await?;
        self.faq = faq;

        let description = Input::<String>::with_theme(theme)
            .with_prompt("Info.Description")
            .default("CyVerse Discovery Environment".into())
            .interact()?;

        let description_id =
            set_config_value(tx, "DE", "Info.Description", &description, "string").await?;
        add_env_cfg_value(tx, env_id, description_id).await?;
        self.description = description;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DE {
    #[serde(skip)]
    section: String,

    #[serde(rename = "AMQP")]
    amqp: Amqp,

    #[serde(rename = "BaseURI")]
    pub base_uri: Option<Url>, //Required before deployment.

    subscriptions: Option<DESubscriptions>,
    default_output_folder: Option<String>,
    coge: Option<DECoge>,
    tools: Option<DETools>,
    info: Option<Info>,
}

impl LoadFromDatabase for DE {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "BaseURI" => self.base_uri = Url::parse(&value).ok(),
                "DefaultOutputFolder" => self.default_output_folder = Some(value),
                _ => (),
            }
            if key.starts_with("Subscriptions") {
                if let Some(subs) = &mut self.subscriptions {
                    subs.cfg_set_key(cfg)?;
                }
            }

            if key.starts_with("Coge") {
                if let Some(coge) = &mut self.coge {
                    coge.cfg_set_key(cfg)?;
                }
            }

            if key.starts_with("Tools") {
                if let Some(tools) = &mut self.tools {
                    tools.cfg_set_key(cfg)?;
                }
            }

            if key.starts_with("AMQP") {
                self.amqp.set_section("DE")?;
                self.amqp.cfg_set_key(cfg)?;
            }

            if key.starts_with("Info") {
                if let Some(info) = &mut self.info {
                    info.cfg_set_key(cfg)?;
                }
            }
        }
        Ok(())
    }
}

impl Default for DE {
    fn default() -> Self {
        DE {
            section: "DE".to_string(),
            amqp: Amqp::default(),
            base_uri: Url::parse("https://de.cyverse.org").ok(),
            subscriptions: Some(DESubscriptions::default()),
            default_output_folder: Some(String::from("analyses")),
            coge: Some(DECoge::default()),
            tools: Some(DETools::default()),
            info: Some(Info::default()),
        }
    }
}

impl From<DE> for Vec<db::ConfigurationValue> {
    fn from(de: DE) -> Self {
        let mut cfgs = Vec::new();
        let section: String;

        if de.section.is_empty() {
            section = "DE".to_string();
        } else {
            section = de.section.clone();
        }

        if let Some(url) = de.base_uri {
            cfgs.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("BaseURI".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }
        cfgs.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("DefaultOutputFolder".to_string()),
            value: de.default_output_folder,
            value_type: Some("string".to_string()),
        });
        if let Some(subs) = de.subscriptions {
            cfgs.extend::<Vec<db::ConfigurationValue>>(subs.into());
        }
        if let Some(coge) = de.coge {
            cfgs.extend::<Vec<db::ConfigurationValue>>(coge.into());
        }
        if let Some(tools) = de.tools {
            cfgs.extend::<Vec<db::ConfigurationValue>>(tools.into());
        }

        let mut amqp_cfgs: Vec<db::ConfigurationValue> = de.amqp.into();
        amqp_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some(section.clone());
        });

        cfgs.extend::<Vec<db::ConfigurationValue>>(amqp_cfgs);

        if let Some(info) = de.info {
            cfgs.extend::<Vec<db::ConfigurationValue>>(info.into());
        }

        cfgs
    }
}

impl DE {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        self.amqp.set_section("DE")?;
        self.amqp.ask_for_info(tx, theme, env_id, "DE").await?;

        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("DE Base URI")
            .interact()?;

        let base_uri_id = set_config_value(tx, "DE", "BaseURI", &base_uri, "string").await?;
        add_env_cfg_value(tx, env_id, base_uri_id).await?;
        self.base_uri = Url::parse(&base_uri).ok();

        let mut new_subs = DESubscriptions::default();
        new_subs.ask_for_info(tx, theme, env_id).await?;
        self.subscriptions = Some(new_subs);

        let default_output_folder = Input::<String>::with_theme(theme)
            .with_prompt("DE Default Output Folder")
            .default("analyses".into())
            .interact()?;
        let default_output_folder_id = set_config_value(
            tx,
            "DE",
            "DefaultOutputFolder",
            &default_output_folder,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, default_output_folder_id).await?;
        self.default_output_folder = Some(default_output_folder);

        let mut new_coge = DECoge::default();
        new_coge.ask_for_info(tx, theme, env_id).await?;
        self.coge = Some(new_coge);

        let mut new_tools = DETools::default();
        new_tools.ask_for_info(tx, theme, env_id).await?;
        self.tools = Some(new_tools);

        let mut new_info = Info::default();
        new_info.ask_for_info(tx, theme, env_id).await?;
        self.info = Some(new_info);

        Ok(())
    }
}
