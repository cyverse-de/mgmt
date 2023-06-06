use crate::config_values::amqp::Amqp;
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DESubscriptions {
    #[serde(rename = "CheckoutURL")]
    checkout_url: Option<Url>,

    enforce: bool,
}

impl DESubscriptions {
    fn merge(&self, right: &DESubscriptions) -> anyhow::Result<DESubscriptions> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let checkout_url = Input::<String>::with_theme(theme)
            .with_prompt("Subscriptions Checkout URL")
            .default("https://cyverse-subscription.phoenixbioinformatics.org".into())
            .interact()?;

        self.checkout_url = Url::parse(&checkout_url).ok();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DECoge {
    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,
}

impl DECoge {
    fn merge(&self, right: &DECoge) -> anyhow::Result<DECoge> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("CoGe Base URI")
            .default("https://genomevolution.org/coge/api/v1".into())
            .interact()?;

        self.base_uri = Url::parse(&base_uri).ok();

        Ok(())
    }
}

impl Default for DECoge {
    fn default() -> Self {
        DECoge {
            base_uri: Url::parse("https://genomevolution.org/coge/api/v1").ok(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DETools {
    admin: DEToolsAdmin,
}

impl DETools {
    fn merge(&self, right: &DETools) -> anyhow::Result<DETools> {
        let mut merged: DETools = serde_merge::omerge(&self, &right)?;
        merged.admin = self.admin.merge(&right.admin)?;
        Ok(merged)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let max_cpu_limit = Input::<u32>::with_theme(theme)
            .with_prompt("Max CPU Limit")
            .default(24)
            .interact()?;

        self.admin.max_cpu_limit = Some(max_cpu_limit);

        let max_memory_limit = Input::<u64>::with_theme(theme)
            .with_prompt("Max Memory Limit")
            .default(75161927680)
            .interact()?;

        self.admin.max_memory_limit = Some(max_memory_limit);

        let max_disk_limit = Input::<u64>::with_theme(theme)
            .with_prompt("Max Disk Limit")
            .default(1099511627776)
            .interact()?;

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

impl DEToolsAdmin {
    fn merge(&self, right: &DEToolsAdmin) -> anyhow::Result<DEToolsAdmin> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DE {
    #[serde(rename = "AMQP")]
    amqp: Amqp,

    #[serde(rename = "BaseURI")]
    pub base_uri: Option<Url>, //Required before deployment.

    subscriptions: Option<DESubscriptions>,
    default_output_folder: String,
    coge: Option<DECoge>,
    tools: Option<DETools>,
}

impl Default for DE {
    fn default() -> Self {
        DE {
            amqp: Amqp::default(),
            base_uri: Url::parse("https://de.cyverse.org").ok(),
            subscriptions: Some(DESubscriptions::default()),
            default_output_folder: String::from("analyses"),
            coge: Some(DECoge::default()),
            tools: Some(DETools::default()),
        }
    }
}

impl DE {
    pub fn merge(&self, right: &DE) -> anyhow::Result<DE> {
        let mut merged: DE = serde_merge::omerge(&self, &right)?;
        if let Some(subscriptions) = &self.subscriptions {
            if let Some(right_subscriptions) = &right.subscriptions {
                merged.subscriptions = Some(subscriptions.merge(right_subscriptions)?);
            }
        }
        if let Some(coge) = &self.coge {
            if let Some(right_coge) = &right.coge {
                merged.coge = Some(coge.merge(&right_coge)?);
            }
        }
        if let Some(tools) = &self.tools {
            if let Some(right_tools) = &right.tools {
                merged.tools = Some(tools.merge(&right_tools)?);
            }
        }
        Ok(merged)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        self.amqp.ask_for_info(theme, "DE")?;

        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("DE Base URI")
            .interact()?;

        self.base_uri = Url::parse(&base_uri).ok();

        let mut new_subs = DESubscriptions::default();
        new_subs.ask_for_info(theme)?;
        self.subscriptions = Some(new_subs);

        let default_output_folder = Input::<String>::with_theme(theme)
            .with_prompt("DE Default Output Folder")
            .default("analyses".into())
            .interact()?;
        self.default_output_folder = default_output_folder;

        let mut new_coge = DECoge::default();
        new_coge.ask_for_info(theme)?;
        self.coge = Some(new_coge);

        let mut new_tools = DETools::default();
        new_tools.ask_for_info(theme)?;
        self.tools = Some(new_tools);

        Ok(())
    }
}
