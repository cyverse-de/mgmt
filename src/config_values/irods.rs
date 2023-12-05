use crate::config_values::amqp::Amqp;
use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct IrodsWebDav {
    #[serde(skip)]
    section: String,

    #[serde(rename = "AnonURI")]
    anon_uri: Option<Url>,
}

impl LoadFromDatabase for IrodsWebDav {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "WebDAV.AnonURI" => self.anon_uri = Url::parse(&value).ok(),
            _ => (),
        }
        Ok(())
    }
}

impl Default for IrodsWebDav {
    fn default() -> Self {
        IrodsWebDav {
            section: "IRODS".to_string(),
            anon_uri: None,
        }
    }
}

impl From<IrodsWebDav> for Vec<db::ConfigurationValue> {
    fn from(iwd: IrodsWebDav) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if iwd.section.is_empty() {
            section = "IRODS".to_string();
        } else {
            section = iwd.section.clone();
        }

        if let Some(anon_uri) = iwd.anon_uri {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "WebDAV.AnonURI".to_string(),
                value: anon_uri.to_string(),
                value_type: "string".to_string(),
            });
        }

        vec
    }
}

impl IrodsWebDav {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
        external: &str,
    ) -> anyhow::Result<()> {
        let anon_uri = Input::<String>::with_theme(theme)
            .with_prompt("Irods WebDav Anon URI")
            .default(format!("https://{}/dav-anon", external))
            .interact()?;

        let anon_uri_id =
            set_config_value(tx, "IRODS", "WebDAV.AnonURI", &anon_uri, "string").await?;
        add_env_cfg_value(tx, env_id, anon_uri_id).await?;
        self.anon_uri = Url::parse(&anon_uri).ok();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", rename = "IRODS")]
pub struct Irods {
    #[serde(skip)]
    section: String,

    #[serde(rename = "AMQP")]
    amqp: Amqp,

    host: String,
    user: String,
    zone: String,
    password: String,
    admin_users: Option<Vec<String>>,
    perms_filter: Option<Vec<String>>,
    pub external_host: Option<String>,

    #[serde(rename = "WebDAV")]
    web_dav: Option<IrodsWebDav>,

    quota_root_resources: Option<String>,
}

impl From<Irods> for Vec<db::ConfigurationValue> {
    fn from(i: Irods) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if i.section.is_empty() {
            section = "IRODS".to_string();
        } else {
            section = i.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Host".to_string(),
            value: i.host,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "User".to_string(),
            value: i.user,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Zone".to_string(),
            value: i.zone,
            value_type: "string".to_string(),
        });

        vec.push(db::ConfigurationValue {
            id: 0,
            section: section.clone(),
            key: "Password".to_string(),
            value: i.password,
            value_type: "string".to_string(),
        });

        if let Some(admin_users) = i.admin_users {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "AdminUsers".to_string(),
                value: admin_users.join(","),
                value_type: "string".to_string(),
            });
        }

        if let Some(perms_filter) = i.perms_filter {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "PermsFilter".to_string(),
                value: perms_filter.join(","),
                value_type: "string".to_string(),
            });
        }

        if let Some(external_host) = i.external_host {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "ExternalHost".to_string(),
                value: external_host,
                value_type: "string".to_string(),
            });
        }

        if let Some(quota_root_resources) = i.quota_root_resources {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "QuotaRootResources".to_string(),
                value: quota_root_resources,
                value_type: "string".to_string(),
            });
        }

        if let Some(web_dav) = i.web_dav {
            vec.extend::<Vec<db::ConfigurationValue>>(web_dav.into());
        }

        let mut amqp_vec: Vec<db::ConfigurationValue> = i.amqp.into();
        amqp_vec.iter_mut().for_each(|c| {
            c.section = section.clone();
        });

        vec.extend::<Vec<db::ConfigurationValue>>(amqp_vec);

        vec
    }
}

impl LoadFromDatabase for Irods {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        let key = cfg.key.clone();
        let value = cfg.value.clone();

        match key.as_str() {
            "Host" => self.host = value,
            "User" => self.user = value,
            "Zone" => self.zone = value,
            "Password" => self.password = value,
            "AdminUsers" => {
                self.admin_users = Some(value.split(',').map(|s| s.to_string()).collect())
            }
            "PermsFilter" => {
                self.perms_filter = Some(value.split(',').map(|s| s.to_string()).collect())
            }
            "ExternalHost" => self.external_host = Some(value),
            "QuotaRootResources" => self.quota_root_resources = Some(value),
            _ => (),
        }

        if key.starts_with("WebDAV") {
            if self.web_dav.is_none() {
                self.web_dav = Some(IrodsWebDav::default());
            }

            if let Some(web_dav) = self.web_dav.as_mut() {
                web_dav.cfg_set_key(cfg)?;
            }
        }

        if key.starts_with("AMQP") {
            self.amqp.set_section("IRODS")?;
            self.amqp.cfg_set_key(cfg)?;
        }

        Ok(())
    }
}

impl Default for Irods {
    fn default() -> Self {
        Irods {
            section: "IRODS".to_string(),
            amqp: Amqp::default(),
            host: String::new(),
            user: String::new(),
            zone: String::new(),
            password: String::new(),
            admin_users: Some(Vec::new()),
            perms_filter: Some(Vec::new()),
            web_dav: Some(IrodsWebDav::default()),
            external_host: None,
            quota_root_resources: None,
        }
    }
}

impl Irods {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, Postgres>,
        theme: &ColorfulTheme,
        env_id: i32,
    ) -> anyhow::Result<()> {
        self.amqp.set_section("IRODS")?;
        self.amqp.ask_for_info(tx, theme, env_id, "iRODS").await?;

        let host = Input::<String>::with_theme(theme)
            .with_prompt("iRODS Host")
            .interact()?;

        let external_host = Input::<String>::with_theme(theme)
            .with_prompt("iRODS External Host")
            .default(host.clone())
            .interact()?;

        let user = Input::<String>::with_theme(theme)
            .with_prompt("iRODS User")
            .interact()?;

        let zone = Input::<String>::with_theme(theme)
            .with_prompt("iRODS Zone")
            .interact()?;

        let password = Input::<String>::with_theme(theme)
            .with_prompt("iRODS Password")
            .interact()?;

        let admin_users = Input::<String>::with_theme(theme)
            .with_prompt("iRODS Admin Users")
            .default("rodsadmin".to_string())
            .interact()?;

        let perms_filter = Input::<String>::with_theme(theme)
            .with_prompt("iRODS Perms Filter")
            .default("rodsadmin".to_string())
            .interact()?;

        let host_id = set_config_value(tx, "IRODS", "Host", &host, "string").await?;
        add_env_cfg_value(tx, env_id, host_id).await?;
        self.host = host;

        let external_host_id =
            set_config_value(tx, "IRODS", "ExternalHost", &external_host, "string").await?;
        add_env_cfg_value(tx, env_id, external_host_id).await?;
        self.external_host = Some(external_host);

        let user_id = set_config_value(tx, "IRODS", "User", &user, "string").await?;
        self.user = user;
        add_env_cfg_value(tx, env_id, user_id).await?;

        let zone_id = set_config_value(tx, "IRODS", "Zone", &zone, "string").await?;
        add_env_cfg_value(tx, env_id, zone_id).await?;
        self.zone = zone;

        let password_id = set_config_value(tx, "IRODS", "Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        let admin_users_id =
            set_config_value(tx, "IRODS", "AdminUsers", &admin_users, "string").await?;
        add_env_cfg_value(tx, env_id, admin_users_id).await?;
        self.admin_users = Some(admin_users.split(',').map(|s| s.to_string()).collect());

        let perms_filter_id =
            set_config_value(tx, "IRODS", "PermsFilter", &perms_filter, "string").await?;
        add_env_cfg_value(tx, env_id, perms_filter_id).await?;
        self.perms_filter = Some(perms_filter.split(',').map(|s| s.to_string()).collect());

        let mut new_web_dav = IrodsWebDav::default();

        // We're okay with unwrap here since it's user input and panicking is fine.
        new_web_dav
            .ask_for_info(tx, theme, env_id, self.external_host.as_ref().unwrap())
            .await?;

        self.web_dav = Some(new_web_dav);

        Ok(())
    }
}
