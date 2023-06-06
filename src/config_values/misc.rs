use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Jobs {
    data_transfer_image: String,
}

impl Default for Jobs {
    fn default() -> Self {
        Jobs {
            data_transfer_image: String::from("harbor.cyverse.org/de/porklock"),
        }
    }
}

impl Jobs {
    pub fn merge(&self, right: &Jobs) -> anyhow::Result<Jobs> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let data_transfer_image = Input::<String>::with_theme(theme)
            .with_prompt("Jobs Data Transfer Image")
            .default("harbor.cyverse.org/de/porklock".into())
            .interact()?;

        self.data_transfer_image = data_transfer_image;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "PGP")]
pub struct Pgp {
    key_password: String,
}

impl Pgp {
    pub fn merge(&self, right: &Pgp) -> anyhow::Result<Pgp> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let key_password = Password::with_theme(theme)
            .with_prompt("PGP Key Password")
            .interact()?;

        self.key_password = key_password;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PermanentIdDataCite {
    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,

    user: String,
    password: String,

    #[serde(rename = "DOIPrefix")]
    doi_prefix: String,
}

impl PermanentIdDataCite {
    fn merge(&self, right: &PermanentIdDataCite) -> anyhow::Result<PermanentIdDataCite> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID DataCite Base URI")
            .default("https://api.datacite.org/".into())
            .interact()?;

        let user = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID DataCite User")
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("Permanent ID DataCite Password")
            .interact()?;

        let doi_prefix = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID DataCite DOI Prefix")
            .interact()?;

        self.base_uri = Url::parse(&base_uri).ok();
        self.user = user;
        self.password = password;
        self.doi_prefix = doi_prefix;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PermanentId {
    curators_group: String,
    data_cite: PermanentIdDataCite,
}

impl PermanentId {
    pub fn merge(&self, right: &PermanentId) -> anyhow::Result<PermanentId> {
        let mut merged: PermanentId = serde_merge::omerge(&self, &right)?;
        merged.data_cite = self.data_cite.merge(&right.data_cite)?;
        Ok(merged)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let curators_group = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID Curators Group")
            .default("data-curators".into())
            .interact()?;

        self.curators_group = curators_group;

        self.data_cite.ask_for_info(theme)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Unleash {
    #[serde(rename = "BaseURL")]
    base_url: Option<Url>,

    #[serde(rename = "APIPath")]
    api_path: Option<String>,

    #[serde(rename = "APIToken")]
    api_token: String,

    maintenance_flag: Option<String>,
}

impl Default for Unleash {
    fn default() -> Self {
        Unleash {
            base_url: Url::parse("http://unleash:4242").ok(),
            api_path: Some(String::from("/api")),
            maintenance_flag: Some(String::from("DE-Maintenance")),
            api_token: String::new(),
        }
    }
}

impl Unleash {
    pub fn merge(&self, right: &Unleash) -> anyhow::Result<Unleash> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let base_url = Input::<String>::with_theme(theme)
            .with_prompt("Unleash Base URL")
            .default("http://unleash:4242".into())
            .interact()?;

        let api_path = Input::<String>::with_theme(theme)
            .with_prompt("Unleash API Path")
            .default("/api".into())
            .interact()?;

        let maintenance_flag = Input::<String>::with_theme(theme)
            .with_prompt("Unleash Maintenance Flag")
            .default("DE-Maintenance".into())
            .interact()?;

        let api_token = Password::with_theme(theme)
            .with_prompt("Unleash API Token")
            .interact()?;

        self.base_url = Url::parse(&base_url).ok();
        self.api_path = Some(api_path);
        self.maintenance_flag = Some(maintenance_flag);
        self.api_token = api_token;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UserPortal {
    #[serde(rename = "BaseURI")]
    base_uri: Option<String>,
}

impl UserPortal {
    pub fn merge(&self, right: &UserPortal) -> anyhow::Result<UserPortal> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("User Portal Base URI")
            .interact()?;

        self.base_uri = Some(base_uri);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Admin {
    groups: Option<String>,
    attribute: Option<String>,
}

impl Default for Admin {
    fn default() -> Self {
        Admin {
            groups: Some(String::from("de_admins")),
            attribute: Some(String::from("entitlement")),
        }
    }
}

impl Admin {
    pub fn merge(&self, right: &Admin) -> anyhow::Result<Admin> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let groups = Input::<String>::with_theme(theme)
            .with_prompt("Admin Groups")
            .default("de_admins".into())
            .interact()?;

        let attribute = Input::<String>::with_theme(theme)
            .with_prompt("Admin Attribute")
            .default("entitlement".into())
            .interact()?;

        self.groups = Some(groups);
        self.attribute = Some(attribute);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Analytics {
    enabled: Option<bool>,
    id: Option<String>,
}

impl Default for Analytics {
    fn default() -> Self {
        Analytics {
            enabled: Some(false),
            id: Some(String::from("g-id")),
        }
    }
}

impl Analytics {
    pub fn merge(&self, right: &Analytics) -> anyhow::Result<Analytics> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let enabled = Select::with_theme(theme)
            .with_prompt("Analytics Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        let id = Input::<String>::with_theme(theme)
            .with_prompt("Analytics ID")
            .default("g-id".into())
            .interact()?;

        self.enabled = Some(enabled == 0);
        self.id = Some(id);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Harbor {
    #[serde(rename = "URL")]
    url: Option<String>, // called a URL, but it's actually a host name.

    #[serde(rename = "ProjectQARobotName")]
    project_qa_robot_name: String,

    #[serde(rename = "ProjectQARobotSecret")]
    project_qa_robot_secret: String,
}

impl Default for Harbor {
    fn default() -> Self {
        Harbor {
            url: Some(String::from("harbor.cyverse.org")),
            project_qa_robot_name: String::new(),
            project_qa_robot_secret: String::new(),
        }
    }
}

impl Harbor {
    pub fn merge(&self, right: &Harbor) -> anyhow::Result<Harbor> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let url = Input::<String>::with_theme(theme)
            .with_prompt("Harbor URL")
            .default("harbor.cyverse.org".into())
            .interact()?;

        let project_qa_robot_name = Input::<String>::with_theme(theme)
            .with_prompt("Harbor Project QA Robot Name")
            .interact()?;

        let project_qa_robot_secret = Password::with_theme(theme)
            .with_prompt("Harbor Project QA Robot Secret")
            .interact()?;

        self.url = Some(url);
        self.project_qa_robot_name = project_qa_robot_name;
        self.project_qa_robot_secret = project_qa_robot_secret;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Qms {
    enabled: Option<bool>,
}

impl Default for Qms {
    fn default() -> Self {
        Qms {
            enabled: Some(true),
        }
    }
}

impl Qms {
    pub fn merge(&self, right: &Qms) -> anyhow::Result<Qms> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let enabled = Select::with_theme(theme)
            .with_prompt("QMS Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        self.enabled = Some(enabled == 0);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Jaeger {
    endpoint: Option<Url>,
    http_endpoint: Option<Url>,
}

impl Default for Jaeger {
    fn default() -> Self {
        Jaeger {
            endpoint: Url::parse("http://jaeger-collector.jaeger.svc.cluster.local:14250").ok(),
            http_endpoint: Url::parse(
                "http://jaeger-collector.jaeger.svc.cluster.local:14268/api/traces",
            )
            .ok(),
        }
    }
}

impl Jaeger {
    pub fn merge(&self, right: &Jaeger) -> anyhow::Result<Jaeger> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let endpoint = Input::<String>::with_theme(theme)
            .with_prompt("Jaeger Endpoint")
            .default("http://jaeger-collector.jaeger.svc.cluster.local:14250".into())
            .interact()?;

        let http_endpoint = Input::<String>::with_theme(theme)
            .with_prompt("Jaeger HTTP Endpoint")
            .default("http://jaeger-collector.jaeger.svc.cluster.local:14268/api/traces".into())
            .interact()?;

        self.endpoint = Url::parse(&endpoint).ok();
        self.http_endpoint = Url::parse(&http_endpoint).ok();

        Ok(())
    }
}
