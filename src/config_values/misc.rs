use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Jobs {
    #[serde(skip)]
    section: String,

    data_transfer_image: Option<String>,
    data_transfer_tag: Option<String>,
}

impl Default for Jobs {
    fn default() -> Self {
        Jobs {
            section: "Jobs".to_string(),
            data_transfer_tag: None,
            data_transfer_image: None,
        }
    }
}

impl LoadFromDatabase for Jobs {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();
        match key.as_str() {
            "DataTransferTag" => self.data_transfer_tag = Some(value),
            "DataTransferImage" => self.data_transfer_image = Some(value),
            _ => (),
        }

        Ok(())
    }
}

impl From<Jobs> for Vec<db::ConfigurationValue> {
    fn from(job: Jobs) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if job.section.is_empty() {
            section = "Jobs".to_string();
        } else {
            section = job.section.clone();
        }

        if let Some(image) = job.data_transfer_image {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DataTransferImage".to_string(),
                value: image,
                value_type: "string".to_string(),
            });
        }

        if let Some(tag) = job.data_transfer_tag {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DataTransferTag".to_string(),
                value: tag,
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

impl Jobs {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
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
        self.data_transfer_image = Some(data_transfer_image);

        let data_transfer_tag = Input::<String>::with_theme(theme)
            .with_prompt("Jobs Data Transfer Tag")
            .default("latest".into())
            .interact()?;
        let data_transfer_tag_id =
            set_config_value(tx, "Jobs", "DataTransferTag", &data_transfer_tag, "string").await?;
        add_env_cfg_value(tx, env_id, data_transfer_tag_id).await?;
        self.data_transfer_tag = Some(data_transfer_tag);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "PGP")]
pub struct Pgp {
    #[serde(skip)]
    section: String,

    key_password: String,
}

impl Default for Pgp {
    fn default() -> Self {
        Pgp {
            section: "PGP".to_string(),
            key_password: String::new(),
        }
    }
}

impl Pgp {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let key_password = Input::<String>::with_theme(theme)
            .with_prompt("PGP Key Password")
            .interact()?;
        let key_password_id =
            set_config_value(tx, "PGP", "KeyPassword", &key_password, "string").await?;
        add_env_cfg_value(tx, env_id, key_password_id).await?;
        self.key_password = key_password;

        Ok(())
    }
}

impl LoadFromDatabase for Pgp {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();
        match key.as_str() {
            "KeyPassword" => self.key_password = value,
            _ => (),
        }
        Ok(())
    }
}

impl From<Pgp> for Vec<db::ConfigurationValue> {
    fn from(p: Pgp) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if p.section.is_empty() {
            section = "PGP".to_string();
        } else {
            section = p.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "KeyPassword".to_string(),
            value: p.key_password,
            value_type: "string".to_string(),
        });
        vec
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PermanentIdDataCite {
    #[serde(skip)]
    section: String,

    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,

    user: String,
    password: String,

    #[serde(rename = "DOIPrefix")]
    doi_prefix: String,
}

impl Default for PermanentIdDataCite {
    fn default() -> Self {
        PermanentIdDataCite {
            section: "PermanentID".to_string(),
            base_uri: None,
            user: String::new(),
            password: String::new(),
            doi_prefix: String::new(),
        }
    }
}

impl PermanentIdDataCite {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID DataCite Base URI")
            .default("https://api.datacite.org/".into())
            .interact()?;

        let user = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID DataCite User")
            .interact()?;

        let password = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID DataCite Password")
            .interact()?;

        let doi_prefix = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID DataCite DOI Prefix")
            .interact()?;

        let base_uri_id =
            set_config_value(tx, "PermanentID", "DataCite.BaseURI", &base_uri, "string").await?;
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

impl LoadFromDatabase for PermanentIdDataCite {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "DataCite.BaseURI" => self.base_uri = Url::parse(&value).ok(),
            "DataCite.User" => self.user = value,
            "DataCite.Password" => self.password = value,
            "DataCite.DOIPrefix" => self.doi_prefix = value,
            _ => (),
        }

        Ok(())
    }
}

impl From<PermanentIdDataCite> for Vec<db::ConfigurationValue> {
    fn from(p: PermanentIdDataCite) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if p.section.is_empty() {
            section = "PermanentID".to_string();
        } else {
            section = p.section.clone();
        }

        if let Some(base_uri) = p.base_uri {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DataCite.BaseURI".to_string(),
                value: base_uri.to_string(),
                value_type: "string".to_string(),
            });
        }

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "DataCite.User".to_string(),
            value: p.user,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "DataCite.Password".to_string(),
            value: p.password,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "DataCite.DOIPrefix".to_string(),
            value: p.doi_prefix,
            value_type: "string".to_string(),
        });

        vec
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PermanentId {
    #[serde(skip)]
    section: String,

    curators_group: String,
    data_cite: PermanentIdDataCite,
}

impl Default for PermanentId {
    fn default() -> Self {
        PermanentId {
            section: "PermanentID".to_string(),
            curators_group: String::new(),
            data_cite: PermanentIdDataCite::default(),
        }
    }
}

impl PermanentId {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
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

impl LoadFromDatabase for PermanentId {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "CuratorsGroup" => self.curators_group = value,
            _ => (),
        }

        if key.starts_with("DataCite.") {
            self.data_cite.cfg_set_key(cfg)?;
        }

        Ok(())
    }
}

impl From<PermanentId> for Vec<db::ConfigurationValue> {
    fn from(p: PermanentId) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if p.section.is_empty() {
            section = "PermanentID".to_string();
        } else {
            section = p.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "CuratorsGroup".to_string(),
            value: p.curators_group,
            value_type: "string".to_string(),
        });

        vec.extend::<Vec<db::ConfigurationValue>>(p.data_cite.into());

        vec
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Unleash {
    #[serde(skip)]
    section: String,

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
            section: "Unleash".to_string(),
            base_url: None,
            api_path: None,
            maintenance_flag: None,
            api_token: String::new(),
        }
    }
}

impl LoadFromDatabase for Unleash {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "BaseURL" => self.base_url = Url::parse(&value).ok(),
            "APIPath" => self.api_path = Some(value),
            "APIToken" => self.api_token = value,
            "MaintenanceFlag" => self.maintenance_flag = Some(value),
            _ => (),
        }

        Ok(())
    }
}

impl From<Unleash> for Vec<db::ConfigurationValue> {
    fn from(u: Unleash) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if u.section.is_empty() {
            section = "Unleash".to_string();
        } else {
            section = u.section.clone();
        }

        if let Some(base_url) = u.base_url {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "BaseURL".to_string(),
                value: base_url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(api_path) = u.api_path {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "APIPath".to_string(),
                value: api_path,
                value_type: "string".to_string(),
            });
        }

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "APIToken".to_string(),
            value: u.api_token,
            value_type: "string".to_string(),
        });

        if let Some(maintenance_flag) = u.maintenance_flag {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "MaintenanceFlag".to_string(),
                value: maintenance_flag,
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

impl Unleash {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
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

        let api_token = Input::<String>::with_theme(theme)
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserPortal {
    #[serde(skip)]
    section: String,

    #[serde(rename = "BaseURI")]
    base_uri: Option<String>,
}

impl Default for UserPortal {
    fn default() -> Self {
        UserPortal {
            section: "UserPortal".to_string(),
            base_uri: Some(String::new()),
        }
    }
}

impl UserPortal {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
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

impl LoadFromDatabase for UserPortal {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "BaseURI" => self.base_uri = Some(value),
            _ => (),
        }

        Ok(())
    }
}

impl From<UserPortal> for Vec<db::ConfigurationValue> {
    fn from(u: UserPortal) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if u.section.is_empty() {
            section = "UserPortal".to_string();
        } else {
            section = u.section.clone();
        }

        if let Some(base_uri) = u.base_uri {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "BaseURI".to_string(),
                value: base_uri,
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Admin {
    #[serde(skip)]
    section: String,

    groups: Option<String>,
    attribute: Option<String>,
}

impl Default for Admin {
    fn default() -> Self {
        Admin {
            section: "Admin".to_string(),
            groups: None,
            attribute: None,
        }
    }
}

impl LoadFromDatabase for Admin {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();
        match key.as_str() {
            "Groups" => self.groups = Some(value),
            "Attribute" => self.attribute = Some(value),
            _ => (),
        }

        Ok(())
    }
}

impl From<Admin> for Vec<db::ConfigurationValue> {
    fn from(a: Admin) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if a.section.is_empty() {
            section = "Admin".to_string();
        } else {
            section = a.section.clone();
        }

        if let Some(groups) = a.groups {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Groups".to_string(),
                value: groups,
                value_type: "string".to_string(),
            });
        }

        if let Some(attribute) = a.attribute {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Attribute".to_string(),
                value: attribute,
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

impl Admin {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Analytics {
    #[serde(skip)]
    section: String,

    enabled: Option<bool>,
    id: Option<String>,
}

impl Default for Analytics {
    fn default() -> Self {
        Analytics {
            section: "Analytics".to_string(),
            enabled: None,
            id: None,
        }
    }
}

impl LoadFromDatabase for Analytics {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "Enabled" => self.enabled = Some(value.parse::<bool>().unwrap_or(false)),
            "Id" => self.id = Some(value),
            _ => (),
        }

        Ok(())
    }
}

impl From<Analytics> for Vec<db::ConfigurationValue> {
    fn from(a: Analytics) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if a.section.is_empty() {
            section = "Analytics".to_string();
        } else {
            section = a.section.clone();
        }

        if let Some(enabled) = a.enabled {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Enabled".to_string(),
                value: format!("{}", enabled),
                value_type: "bool".to_string(),
            });
        }

        if let Some(id) = a.id {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Id".to_string(),
                value: id,
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

impl Analytics {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
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
            "bool",
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Harbor {
    #[serde(skip)]
    section: String,

    #[serde(rename = "URL")]
    url: Option<String>, // called a URL, but it's actually a host name.

    #[serde(rename = "ProjectQAImagePullSecretName")]
    project_qa_image_pull_secret_name: Option<String>,

    #[serde(rename = "ProjectQARobotName")]
    project_qa_robot_name: Option<String>,

    #[serde(rename = "ProjectQARobotSecret")]
    project_qa_robot_secret: Option<String>,
}

impl Default for Harbor {
    fn default() -> Self {
        Harbor {
            section: "Harbor".to_string(),
            url: None,
            project_qa_image_pull_secret_name: Some(String::new()),
            project_qa_robot_name: Some(String::new()),
            project_qa_robot_secret: Some(String::new()),
        }
    }
}

impl LoadFromDatabase for Harbor {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "URL" => self.url = Some(value),
            "ProjectQAImagePullSecretName" => self.project_qa_image_pull_secret_name = Some(value),
            "ProjectQARobotName" => self.project_qa_robot_name = Some(value),
            "ProjectQARobotSecret" => self.project_qa_robot_secret = Some(value),
            _ => (),
        }

        Ok(())
    }
}

impl From<Harbor> for Vec<db::ConfigurationValue> {
    fn from(h: Harbor) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if h.section.is_empty() {
            section = "Harbor".to_string();
        } else {
            section = h.section.clone();
        }

        if let Some(url) = h.url {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "URL".to_string(),
                value: url,
                value_type: "string".to_string(),
            });
        }

        if let Some(project_qa_image_pull_secret_name) = h.project_qa_image_pull_secret_name {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "ProjectQAImagePullSecretName".to_string(),
                value: project_qa_image_pull_secret_name,
                value_type: "string".to_string(),
            });
        }

        if let Some(project_qa_robot_name) = h.project_qa_robot_name {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "ProjectQARobotName".to_string(),
                value: project_qa_robot_name,
                value_type: "string".to_string(),
            });
        }

        if let Some(project_qa_robot_secret) = h.project_qa_robot_secret {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "ProjectQARobotSecret".to_string(),
                value: project_qa_robot_secret,
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

impl Harbor {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let url = Input::<String>::with_theme(theme)
            .with_prompt("Harbor URL")
            .default("harbor.cyverse.org".into())
            .interact()?;

        let project_qa_robot_name = Input::<String>::with_theme(theme)
            .with_prompt("Harbor Project QA Robot Name")
            .interact()?;

        let project_qa_robot_secret = Input::<String>::with_theme(theme)
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
        self.project_qa_robot_name = Some(project_qa_robot_name);

        let project_qa_robot_secret_id = set_config_value(
            tx,
            "Harbor",
            "ProjectQARobotSecret",
            &project_qa_robot_secret,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, project_qa_robot_secret_id).await?;
        self.project_qa_robot_secret = Some(project_qa_robot_secret);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Qms {
    #[serde(skip)]
    section: String,

    enabled: Option<bool>,
}

impl Default for Qms {
    fn default() -> Self {
        Qms {
            section: String::from("QMS"),
            enabled: None,
        }
    }
}

impl LoadFromDatabase for Qms {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "Enabled" => self.enabled = Some(value.parse::<bool>().unwrap_or(false)),
            _ => (),
        }

        Ok(())
    }
}

impl From<Qms> for Vec<db::ConfigurationValue> {
    fn from(q: Qms) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if q.section.is_empty() {
            section = "QMS".to_string();
        } else {
            section = q.section.clone();
        }

        if let Some(enabled) = q.enabled {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Enabled".to_string(),
                value: format!("{}", enabled),
                value_type: "bool".to_string(),
            });
        }

        vec
    }
}

impl Qms {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let enabled = Select::with_theme(theme)
            .with_prompt("QMS Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;
        let enabled_id =
            set_config_value(tx, "QMS", "Enabled", &format!("{}", enabled == 0), "bool").await?;
        add_env_cfg_value(tx, env_id, enabled_id).await?;
        self.enabled = Some(enabled == 0);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Jaeger {
    #[serde(skip)]
    section: String,

    endpoint: Option<Url>,
    http_endpoint: Option<Url>,
}

impl Default for Jaeger {
    fn default() -> Self {
        Jaeger {
            section: "Jaeger".to_string(),
            endpoint: None,
            http_endpoint: None,
        }
    }
}

impl LoadFromDatabase for Jaeger {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "Endpoint" => self.endpoint = Url::parse(&value).ok(),
            "HttpEndpoint" => self.http_endpoint = Url::parse(&value).ok(),
            _ => (),
        }

        Ok(())
    }
}

impl From<Jaeger> for Vec<db::ConfigurationValue> {
    fn from(j: Jaeger) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if j.section.is_empty() {
            section = "Jaeger".to_string();
        } else {
            section = j.section.clone();
        }

        if let Some(endpoint) = j.endpoint {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Endpoint".to_string(),
                value: endpoint.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(http_endpoint) = j.http_endpoint {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "HttpEndpoint".to_string(),
                value: http_endpoint.to_string(),
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

impl Jaeger {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
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
