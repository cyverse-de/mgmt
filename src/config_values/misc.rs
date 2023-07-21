use crate::db::{add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
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

impl LoadFromConfiguration for Jobs {
    fn get_section(&self) -> String {
        "Jobs".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "DataTransferImage" => self.data_transfer_image = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl Jobs {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let data_transfer_image = Input::<String>::with_theme(theme)
            .with_prompt("Jobs Data Transfer Image")
            .default("harbor.cyverse.org/de/porklock".into())
            .interact()?;

        let data_transfer_image_id = set_config_value(
            tx,
            "Jobs",
            "DataTransferImage",
            &data_transfer_image,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, data_transfer_image_id).await?;
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
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let key_password = Password::with_theme(theme)
            .with_prompt("PGP Key Password")
            .interact()?;
        let key_password_id =
            set_config_value(tx, "PGP", "KeyPassword", &key_password, "string").await?;
        add_env_cfg_value(tx, env_id, key_password_id).await?;
        self.key_password = key_password;

        Ok(())
    }
}

impl LoadFromConfiguration for Pgp {
    fn get_section(&self) -> String {
        "PGP".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "KeyPassword" => self.key_password = value,
                _ => (),
            }
        }
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
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
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

        let base_uri_id =
            set_config_value(tx, "PermanentID", "DataCite.BaseURI", &base_uri, "url").await?;
        add_env_cfg_value(tx, env_id, base_uri_id).await?;
        self.base_uri = Url::parse(&base_uri).ok();

        let user_id = set_config_value(tx, "PermanentID", "DataCite.User", &user, "string").await?;
        add_env_cfg_value(tx, env_id, user_id).await?;
        self.user = user;

        let password_id =
            set_config_value(tx, "PermanentID", "DataCite.Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        let doi_prefix_id = set_config_value(
            tx,
            "PermanentID",
            "DataCite.DOIPrefix",
            &doi_prefix,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, doi_prefix_id).await?;
        self.doi_prefix = doi_prefix;

        Ok(())
    }
}

impl LoadFromConfiguration for PermanentIdDataCite {
    fn get_section(&self) -> String {
        "PermanentID".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "DataCite.BaseURI" => self.base_uri = Url::parse(&value).ok(),
                "DataCite.User" => self.user = value,
                "DataCite.Password" => self.password = value,
                "DataCite.DOIPrefix" => self.doi_prefix = value,
                _ => (),
            }
        }
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
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let curators_group = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID Curators Group")
            .default("data-curators".into())
            .interact()?;
        let curators_group_id = set_config_value(
            tx,
            "PermanentID",
            "CuratorsGroup",
            &curators_group,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, curators_group_id).await?;
        self.curators_group = curators_group;

        self.data_cite.ask_for_info(tx, theme, env_id).await?;

        Ok(())
    }
}

impl LoadFromConfiguration for PermanentId {
    fn get_section(&self) -> String {
        "PermanentID".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "CuratorsGroup" => self.curators_group = value,
                _ => (),
            }

            if key.starts_with("DataCite.") {
                self.data_cite.cfg_set_key(cfg)?;
            }
        }

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

impl LoadFromConfiguration for Unleash {
    fn get_section(&self) -> String {
        "Unleash".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "BaseURL" => self.base_url = Url::parse(&value).ok(),
                "APIPath" => self.api_path = Some(value),
                "APIToken" => self.api_token = value,
                "MaintenanceFlag" => self.maintenance_flag = Some(value),
                _ => (),
            }
        }
        Ok(())
    }
}

impl Unleash {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
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

        let base_url_id = set_config_value(tx, "Unleash", "BaseURL", &base_url, "string").await?;
        add_env_cfg_value(tx, env_id, base_url_id).await?;
        self.base_url = Url::parse(&base_url).ok();

        let api_path_id = set_config_value(tx, "Unleash", "APIPath", &api_path, "string").await?;
        add_env_cfg_value(tx, env_id, api_path_id).await?;
        self.api_path = Some(api_path);

        let maintenance_flag_id = set_config_value(
            tx,
            "Unleash",
            "MaintenanceFlag",
            &maintenance_flag,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, maintenance_flag_id).await?;
        self.maintenance_flag = Some(maintenance_flag);

        let api_token_id =
            set_config_value(tx, "Unleash", "APIToken", &api_token, "string").await?;
        add_env_cfg_value(tx, env_id, api_token_id).await?;
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
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("User Portal Base URI")
            .interact()?;

        let base_uri_id =
            set_config_value(tx, "UserPortal", "BaseURI", &base_uri, "string").await?;
        add_env_cfg_value(tx, env_id, base_uri_id).await?;
        self.base_uri = Some(base_uri);

        Ok(())
    }
}

impl LoadFromConfiguration for UserPortal {
    fn get_section(&self) -> String {
        "UserPortal".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "BaseURI" => self.base_uri = Some(value),
                _ => (),
            }
        }
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

impl LoadFromConfiguration for Admin {
    fn get_section(&self) -> String {
        "Admin".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Groups" => self.groups = Some(value),
                "Attribute" => self.attribute = Some(value),
                _ => (),
            }
        }
        Ok(())
    }
}

impl Admin {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let groups = Input::<String>::with_theme(theme)
            .with_prompt("Admin Groups")
            .default("de_admins".into())
            .interact()?;

        let attribute = Input::<String>::with_theme(theme)
            .with_prompt("Admin Attribute")
            .default("entitlement".into())
            .interact()?;

        let groups_id = set_config_value(tx, "Admin", "Groups", &groups, "string").await?;
        add_env_cfg_value(tx, env_id, groups_id).await?;
        self.groups = Some(groups);

        let attribute_id = set_config_value(tx, "Admin", "Attribute", &attribute, "string").await?;
        add_env_cfg_value(tx, env_id, attribute_id).await?;
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

impl LoadFromConfiguration for Analytics {
    fn get_section(&self) -> String {
        "Analytics".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Enabled" => self.enabled = Some(value.parse::<bool>().unwrap_or(false)),
                "Id" => self.id = Some(value),
                _ => (),
            }
        }
        Ok(())
    }
}

impl Analytics {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let enabled = Select::with_theme(theme)
            .with_prompt("Analytics Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        let id = Input::<String>::with_theme(theme)
            .with_prompt("Analytics ID")
            .default("g-id".into())
            .interact()?;

        let enabled_id = set_config_value(
            tx,
            "Analytics",
            "Enabled",
            &format!("{}", enabled == 0),
            "boolean",
        )
        .await?;
        add_env_cfg_value(tx, env_id, enabled_id).await?;
        self.enabled = Some(enabled == 0);

        let id_id = set_config_value(tx, "Analytics", "Id", &id, "string").await?;
        add_env_cfg_value(tx, env_id, id_id).await?;
        self.id = Some(id);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Harbor {
    #[serde(rename = "URL")]
    url: Option<String>, // called a URL, but it's actually a host name.

    #[serde(rename = "ProjectQAImagePullSecretName")]
    project_qa_image_pull_secret_name: String,

    #[serde(rename = "ProjectQARobotName")]
    project_qa_robot_name: String,

    #[serde(rename = "ProjectQARobotSecret")]
    project_qa_robot_secret: String,
}

impl Default for Harbor {
    fn default() -> Self {
        Harbor {
            url: Some(String::from("harbor.cyverse.org")),
            project_qa_image_pull_secret_name: String::new(),
            project_qa_robot_name: String::new(),
            project_qa_robot_secret: String::new(),
        }
    }
}

impl LoadFromConfiguration for Harbor {
    fn get_section(&self) -> String {
        "Harbor".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "URL" => self.url = Some(value),
                "ProjectQAImagePullSecretName" => self.project_qa_image_pull_secret_name = value,
                "ProjectQARobotName" => self.project_qa_robot_name = value,
                "ProjectQARobotSecret" => self.project_qa_robot_secret = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl Harbor {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
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

        let url_id = set_config_value(tx, "Harbor", "URL", &url, "string").await?;
        add_env_cfg_value(tx, env_id, url_id).await?;
        self.url = Some(url);

        let project_qa_robot_name_id = set_config_value(
            tx,
            "Harbor",
            "ProjectQARobotName",
            &project_qa_robot_name,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, project_qa_robot_name_id).await?;
        self.project_qa_robot_name = project_qa_robot_name;

        let project_qa_robot_secret_id = set_config_value(
            tx,
            "Harbor",
            "ProjectQARobotSecret",
            &project_qa_robot_secret,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, project_qa_robot_secret_id).await?;
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

impl LoadFromConfiguration for Qms {
    fn get_section(&self) -> String {
        "QMS".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Enabled" => self.enabled = Some(value.parse::<bool>().unwrap_or(false)),
                _ => (),
            }
        }
        Ok(())
    }
}

impl Qms {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let enabled = Select::with_theme(theme)
            .with_prompt("QMS Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;
        let enabled_id = set_config_value(
            tx,
            "QMS",
            "Enabled",
            &format!("{}", enabled == 0),
            "boolean",
        )
        .await?;
        add_env_cfg_value(tx, env_id, enabled_id).await?;
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

impl LoadFromConfiguration for Jaeger {
    fn get_section(&self) -> String {
        "Jaeger".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Endpoint" => self.endpoint = Url::parse(&value).ok(),
                "HttpEndpoint" => self.http_endpoint = Url::parse(&value).ok(),
                _ => (),
            }
        }
        Ok(())
    }
}

impl Jaeger {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let endpoint = Input::<String>::with_theme(theme)
            .with_prompt("Jaeger Endpoint")
            .default("http://jaeger-collector.jaeger.svc.cluster.local:14250".into())
            .interact()?;

        let http_endpoint = Input::<String>::with_theme(theme)
            .with_prompt("Jaeger HTTP Endpoint")
            .default("http://jaeger-collector.jaeger.svc.cluster.local:14268/api/traces".into())
            .interact()?;

        let endpoint_id = set_config_value(tx, "Jaeger", "Endpoint", &endpoint, "string").await?;
        add_env_cfg_value(tx, env_id, endpoint_id).await?;
        self.endpoint = Url::parse(&endpoint).ok();

        let http_endpoint_id =
            set_config_value(tx, "Jaeger", "HttpEndpoint", &http_endpoint, "string").await?;
        add_env_cfg_value(tx, env_id, http_endpoint_id).await?;
        self.http_endpoint = Url::parse(&http_endpoint).ok();

        Ok(())
    }
}
