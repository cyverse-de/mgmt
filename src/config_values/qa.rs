use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct QACeph {
    #[serde(skip)]
    section: String,
    password: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl Default for QACeph {
    fn default() -> Self {
        QACeph {
            section: "QA".to_string(),
            password: None,
            username: None,
            first_name: None,
            last_name: None,
        }
    }
}

impl LoadFromDatabase for QACeph {
    fn get_section(&self) -> String {
        self.section.clone()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        match cfg.key.as_str() {
            "Ceph.Password" => self.password = Some(cfg.value.clone()),
            "Ceph.Username" => self.username = Some(cfg.value.clone()),
            "Ceph.FirstName" => self.first_name = Some(cfg.value.clone()),
            "Ceph.LastName" => self.last_name = Some(cfg.value.clone()),
            _ => (),
        }
        Ok(())
    }
}

impl From<QACeph> for Vec<db::ConfigurationValue> {
    fn from(ceph: QACeph) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();

        let section: String;
        if ceph.section.is_empty() {
            section = "QA".to_string();
        } else {
            section = ceph.section.clone();
        }

        if let Some(password) = ceph.password {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Ceph.Password".to_string(),
                value: password,
                value_type: "string".to_string(),
            });
        }

        if let Some(username) = ceph.username {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Ceph.Username".to_string(),
                value: username,
                value_type: "string".to_string(),
            });
        }

        if let Some(first_name) = ceph.first_name {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Ceph.FirstName".to_string(),
                value: first_name,
                value_type: "string".to_string(),
            });
        }

        if let Some(last_name) = ceph.last_name {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Ceph.LastName".to_string(),
                value: last_name,
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

impl QACeph {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let password = Input::<String>::with_theme(theme)
            .with_prompt("Ceph Password")
            .allow_empty(true)
            .interact_text()?;

        let username = Input::<String>::with_theme(theme)
            .with_prompt("Ceph Username")
            .allow_empty(true)
            .interact_text()?;

        let first_name = Input::<String>::with_theme(theme)
            .with_prompt("Ceph First Name")
            .allow_empty(true)
            .interact_text()?;

        let last_name = Input::<String>::with_theme(theme)
            .with_prompt("Ceph Last Name")
            .allow_empty(true)
            .interact_text()?;

        let password_id = set_config_value(tx, "QA", "Ceph.Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = Some(password);

        let username_id = set_config_value(tx, "QA", "Ceph.Username", &username, "string").await?;
        add_env_cfg_value(tx, env_id, username_id).await?;
        self.username = Some(username);

        let first_name_id =
            set_config_value(tx, "QA", "Ceph.FirstName", &first_name, "string").await?;
        add_env_cfg_value(tx, env_id, first_name_id).await?;
        self.first_name = Some(first_name);

        let last_name_id =
            set_config_value(tx, "QA", "Ceph.LastName", &last_name, "string").await?;
        add_env_cfg_value(tx, env_id, last_name_id).await?;
        self.last_name = Some(last_name);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct QADE {
    #[serde(skip)]
    section: String,
    password: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    local_user: Option<String>,
    jwt_name: Option<String>,
    admin_password: Option<String>,
}

impl Default for QADE {
    fn default() -> Self {
        QADE {
            section: "QA".to_string(),
            password: None,
            username: None,
            first_name: None,
            last_name: None,
            local_user: None,
            jwt_name: None,
            admin_password: None,
        }
    }
}

impl LoadFromDatabase for QADE {
    fn get_section(&self) -> String {
        self.section.clone()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        match cfg.key.as_str() {
            "DE.Password" => self.password = Some(cfg.value.clone()),
            "DE.Username" => self.username = Some(cfg.value.clone()),
            "DE.FirstName" => self.first_name = Some(cfg.value.clone()),
            "DE.LastName" => self.last_name = Some(cfg.value.clone()),
            "DE.LocalUser" => self.local_user = Some(cfg.value.clone()),
            "DE.JwtName" => self.jwt_name = Some(cfg.value.clone()),
            "DE.AdminPassword" => self.admin_password = Some(cfg.value.clone()),
            _ => (),
        }
        Ok(())
    }
}

impl From<QADE> for Vec<db::ConfigurationValue> {
    fn from(de: QADE) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();

        let section: String;
        if de.section.is_empty() {
            section = "QA".to_string();
        } else {
            section = de.section.clone();
        }

        if let Some(password) = de.password {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DE.Password".to_string(),
                value: password,
                value_type: "string".to_string(),
            });
        }

        if let Some(username) = de.username {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DE.Username".to_string(),
                value: username,
                value_type: "string".to_string(),
            });
        }

        if let Some(first_name) = de.first_name {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DE.FirstName".to_string(),
                value: first_name,
                value_type: "string".to_string(),
            });
        }

        if let Some(last_name) = de.last_name {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DE.LastName".to_string(),
                value: last_name,
                value_type: "string".to_string(),
            });
        }

        if let Some(local_user) = de.local_user {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DE.LocalUser".to_string(),
                value: local_user,
                value_type: "string".to_string(),
            });
        }

        if let Some(jwt_name) = de.jwt_name {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DE.JwtName".to_string(),
                value: jwt_name,
                value_type: "string".to_string(),
            });
        }

        if let Some(admin_password) = de.admin_password {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DE.AdminPassword".to_string(),
                value: admin_password,
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

impl QADE {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let password = Input::<String>::with_theme(theme)
            .with_prompt("DE Password")
            .allow_empty(true)
            .interact_text()?;

        let username = Input::<String>::with_theme(theme)
            .with_prompt("DE Username")
            .allow_empty(true)
            .interact_text()?;

        let first_name = Input::<String>::with_theme(theme)
            .with_prompt("DE First Name")
            .allow_empty(true)
            .interact_text()?;

        let last_name = Input::<String>::with_theme(theme)
            .with_prompt("DE Last Name")
            .allow_empty(true)
            .interact_text()?;

        let local_user = Input::<String>::with_theme(theme)
            .with_prompt("DE Local User")
            .allow_empty(true)
            .interact_text()?;

        let jwt_name = Input::<String>::with_theme(theme)
            .with_prompt("DE JWT Name")
            .allow_empty(true)
            .interact_text()?;

        let admin_password = Input::<String>::with_theme(theme)
            .with_prompt("DE Admin Password")
            .allow_empty(true)
            .interact_text()?;

        let password_id = set_config_value(tx, "QA", "DE.Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = Some(password);

        let username_id = set_config_value(tx, "QA", "DE.Username", &username, "string").await?;
        add_env_cfg_value(tx, env_id, username_id).await?;
        self.username = Some(username);

        let first_name_id =
            set_config_value(tx, "QA", "DE.FirstName", &first_name, "string").await?;
        add_env_cfg_value(tx, env_id, first_name_id).await?;
        self.first_name = Some(first_name);

        let last_name_id = set_config_value(tx, "QA", "DE.LastName", &last_name, "string").await?;
        add_env_cfg_value(tx, env_id, last_name_id).await?;
        self.last_name = Some(last_name);

        let local_user_id =
            set_config_value(tx, "QA", "DE.LocalUser", &local_user, "string").await?;
        add_env_cfg_value(tx, env_id, local_user_id).await?;
        self.local_user = Some(local_user);

        let jwt_name_id = set_config_value(tx, "QA", "DE.JwtName", &jwt_name, "string").await?;
        add_env_cfg_value(tx, env_id, jwt_name_id).await?;
        self.jwt_name = Some(jwt_name);

        let admin_password_id =
            set_config_value(tx, "QA", "DE.AdminPassword", &admin_password, "string").await?;
        add_env_cfg_value(tx, env_id, admin_password_id).await?;
        self.admin_password = Some(admin_password);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct QALegacy {
    #[serde(skip)]
    section: String,

    password: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    local_user: Option<String>,
    jwt_name: Option<String>,
    admin_password: Option<String>,
    admin_password2: Option<String>,
    jwt_priv_pass: Option<String>,
}

impl Default for QALegacy {
    fn default() -> Self {
        QALegacy {
            section: "QA".to_string(),
            password: None,
            username: None,
            first_name: None,
            last_name: None,
            local_user: None,
            jwt_name: None,
            admin_password: None,
            admin_password2: None,
            jwt_priv_pass: None,
        }
    }
}

impl LoadFromDatabase for QALegacy {
    fn get_section(&self) -> String {
        self.section.clone()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        match cfg.key.as_str() {
            "Legacy.Password" => self.password = Some(cfg.value.clone()),
            "Legacy.Username" => self.username = Some(cfg.value.clone()),
            "Legacy.FirstName" => self.first_name = Some(cfg.value.clone()),
            "Legacy.LastName" => self.last_name = Some(cfg.value.clone()),
            "Legacy.LocalUser" => self.local_user = Some(cfg.value.clone()),
            "Legacy.JwtName" => self.jwt_name = Some(cfg.value.clone()),
            "Legacy.AdminPassword" => self.admin_password = Some(cfg.value.clone()),
            "Legacy.AdminPassword2" => self.admin_password2 = Some(cfg.value.clone()),
            "Legacy.JwtPrivPass" => self.jwt_priv_pass = Some(cfg.value.clone()),
            _ => (),
        }
        Ok(())
    }
}

impl From<QALegacy> for Vec<db::ConfigurationValue> {
    fn from(legacy: QALegacy) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();

        let section: String;
        if legacy.section.is_empty() {
            section = "QA".to_string();
        } else {
            section = legacy.section.clone();
        }

        if let Some(password) = legacy.password {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Legacy.Password".to_string(),
                value: password,
                value_type: "string".to_string(),
            });
        }

        if let Some(username) = legacy.username {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Legacy.Username".to_string(),
                value: username,
                value_type: "string".to_string(),
            });
        }

        if let Some(first_name) = legacy.first_name {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Legacy.FirstName".to_string(),
                value: first_name,
                value_type: "string".to_string(),
            });
        }

        if let Some(last_name) = legacy.last_name {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Legacy.LastName".to_string(),
                value: last_name,
                value_type: "string".to_string(),
            });
        }

        if let Some(local_user) = legacy.local_user {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Legacy.LocalUser".to_string(),
                value: local_user,
                value_type: "string".to_string(),
            });
        }

        if let Some(jwt_name) = legacy.jwt_name {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Legacy.JwtName".to_string(),
                value: jwt_name,
                value_type: "string".to_string(),
            });
        }

        if let Some(admin_password) = legacy.admin_password {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Legacy.AdminPassword".to_string(),
                value: admin_password,
                value_type: "string".to_string(),
            });
        }

        if let Some(admin_password2) = legacy.admin_password2 {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Legacy.AdminPassword2".to_string(),
                value: admin_password2,
                value_type: "string".to_string(),
            });
        }

        if let Some(jwt_priv_pass) = legacy.jwt_priv_pass {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Legacy.JwtPrivPass".to_string(),
                value: jwt_priv_pass,
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

impl QALegacy {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        let password = Input::<String>::with_theme(theme)
            .with_prompt("Legacy Password")
            .allow_empty(true)
            .interact_text()?;

        let username = Input::<String>::with_theme(theme)
            .with_prompt("Legacy Username")
            .allow_empty(true)
            .interact_text()?;

        let first_name = Input::<String>::with_theme(theme)
            .with_prompt("Legacy First Name")
            .allow_empty(true)
            .interact_text()?;

        let last_name = Input::<String>::with_theme(theme)
            .with_prompt("Legacy Last Name")
            .allow_empty(true)
            .interact_text()?;

        let local_user = Input::<String>::with_theme(theme)
            .with_prompt("Legacy Local User")
            .allow_empty(true)
            .interact_text()?;

        let jwt_name = Input::<String>::with_theme(theme)
            .with_prompt("Legacy JWT Name")
            .allow_empty(true)
            .interact_text()?;

        let admin_password = Input::<String>::with_theme(theme)
            .with_prompt("Legacy Admin Password")
            .allow_empty(true)
            .interact_text()?;

        let admin_password2 = Input::<String>::with_theme(theme)
            .with_prompt("Legacy Admin Password 2")
            .allow_empty(true)
            .interact_text()?;

        let jwt_priv_pass = Input::<String>::with_theme(theme)
            .with_prompt("Legacy JWT Priv Pass")
            .allow_empty(true)
            .interact_text()?;

        let password_id =
            set_config_value(tx, "QA", "Legacy.Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = Some(password);

        let username_id =
            set_config_value(tx, "QA", "Legacy.Username", &username, "string").await?;
        add_env_cfg_value(tx, env_id, username_id).await?;
        self.username = Some(username);

        let first_name_id =
            set_config_value(tx, "QA", "Legacy.FirstName", &first_name, "string").await?;
        add_env_cfg_value(tx, env_id, first_name_id).await?;
        self.first_name = Some(first_name);

        let last_name_id =
            set_config_value(tx, "QA", "Legacy.LastName", &last_name, "string").await?;
        add_env_cfg_value(tx, env_id, last_name_id).await?;
        self.last_name = Some(last_name);

        let local_user_id =
            set_config_value(tx, "QA", "Legacy.LocalUser", &local_user, "string").await?;
        add_env_cfg_value(tx, env_id, local_user_id).await?;
        self.local_user = Some(local_user);

        let jwt_name_id = set_config_value(tx, "QA", "Legacy.JwtName", &jwt_name, "string").await?;
        add_env_cfg_value(tx, env_id, jwt_name_id).await?;
        self.jwt_name = Some(jwt_name);

        let admin_password_id =
            set_config_value(tx, "QA", "Legacy.AdminPassword", &admin_password, "string").await?;
        add_env_cfg_value(tx, env_id, admin_password_id).await?;
        self.admin_password = Some(admin_password);

        let admin_password2_id = set_config_value(
            tx,
            "QA",
            "Legacy.AdminPassword2",
            &admin_password2,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, admin_password2_id).await?;
        self.admin_password2 = Some(admin_password2);

        let jwt_priv_pass_id =
            set_config_value(tx, "QA", "Legacy.JwtPrivPass", &jwt_priv_pass, "string").await?;
        add_env_cfg_value(tx, env_id, jwt_priv_pass_id).await?;
        self.jwt_priv_pass = Some(jwt_priv_pass);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct QA {
    #[serde(skip)]
    section: String,

    ceph: QACeph,

    #[serde(rename = "DE")]
    de: QADE,

    legacy: QALegacy,
}

impl Default for QA {
    fn default() -> Self {
        QA {
            section: "QA".to_string(),
            ceph: QACeph::default(),
            de: QADE::default(),
            legacy: QALegacy::default(),
        }
    }
}

impl LoadFromDatabase for QA {
    fn get_section(&self) -> String {
        self.section.clone()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let k = cfg.key.clone();

        if k.starts_with("Ceph.") {
            self.ceph.cfg_set_key(cfg)?;
        } else if k.starts_with("DE.") {
            self.de.cfg_set_key(cfg)?;
        } else if k.starts_with("Legacy.") {
            self.legacy.cfg_set_key(cfg)?;
        }

        Ok(())
    }
}

impl From<QA> for Vec<db::ConfigurationValue> {
    fn from(qa: QA) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();

        let ceph_vec: Vec<db::ConfigurationValue> = qa.ceph.into();
        vec.extend::<Vec<db::ConfigurationValue>>(ceph_vec);

        let de_vec: Vec<db::ConfigurationValue> = qa.de.into();
        vec.extend::<Vec<db::ConfigurationValue>>(de_vec);

        let legacy_vec: Vec<db::ConfigurationValue> = qa.legacy.into();
        vec.extend::<Vec<db::ConfigurationValue>>(legacy_vec);

        vec
    }
}

impl QA {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        self.ceph.ask_for_info(tx, theme, env_id).await?;
        self.de.ask_for_info(tx, theme, env_id).await?;
        self.legacy.ask_for_info(tx, theme, env_id).await?;

        Ok(())
    }
}
