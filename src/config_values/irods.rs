use crate::config_values::amqp::Amqp;
use crate::db::{add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IrodsWebDav {
    #[serde(rename = "AnonURI")]
    anon_uri: Option<Url>,
}

impl LoadFromConfiguration for IrodsWebDav {
    fn get_section(&self) -> String {
        "IRODS".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "WebDAV.AnonURI" => self.anon_uri = Url::parse(&value).ok(),
                _ => (),
            }
        }
        Ok(())
    }
}

impl Default for IrodsWebDav {
    fn default() -> Self {
        IrodsWebDav {
            anon_uri: Url::parse("https://data.cyverse.rocks/dav-anon").ok(),
        }
    }
}

impl IrodsWebDav {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Irods {
    #[serde(rename = "AMQP")]
    amqp: Amqp,

    host: String,
    user: String,
    zone: String,
    password: String,
    admin_users: Vec<String>,
    perms_filter: Vec<String>,
    pub external_host: Option<String>,
    web_dav: Option<IrodsWebDav>,
    quota_root_resources: Option<String>,
}

impl LoadFromConfiguration for Irods {
    fn get_section(&self) -> String {
        "IRODS".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Host" => self.host = value,
                "User" => self.user = value,
                "Zone" => self.zone = value,
                "Password" => self.password = value,
                "AdminUsers" => {
                    self.admin_users = value.split(',').map(|s| s.to_string()).collect()
                }
                "PermsFilter" => {
                    self.perms_filter = value.split(',').map(|s| s.to_string()).collect()
                }
                "ExternalHost" => self.external_host = Some(value),
                "QuotaRootResources" => self.quota_root_resources = Some(value),
                _ => (),
            }

            if key.starts_with("WebDAV") {
                if let Some(web_dav) = self.web_dav.as_mut() {
                    web_dav.cfg_set_key(cfg)?;
                }
            }

            if key.starts_with("AMQP") {
                self.amqp.cfg_set_key(cfg)?;
            }
        }
        Ok(())
    }
}

impl Default for Irods {
    fn default() -> Self {
        Irods {
            amqp: Amqp::default(),
            host: String::new(),
            user: String::new(),
            zone: String::new(),
            password: String::new(),
            admin_users: Vec::new(),
            perms_filter: Vec::new(),
            web_dav: Some(IrodsWebDav::default()),
            external_host: Some(String::from("data.cyverse.rocks")),
            quota_root_resources: Some(String::from("mainIngestRes,mainReplRes")),
        }
    }
}

impl Irods {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
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

        let password = Password::with_theme(theme)
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
        self.admin_users = admin_users.split(',').map(|s| s.to_string()).collect();

        let perms_filter_id =
            set_config_value(tx, "IRODS", "PermsFilter", &perms_filter, "string").await?;
        add_env_cfg_value(tx, env_id, perms_filter_id).await?;
        self.perms_filter = perms_filter.split(',').map(|s| s.to_string()).collect();

        let mut new_web_dav = IrodsWebDav::default();

        // We're okay with unwrap here since it's user input and panicking is fine.
        new_web_dav
            .ask_for_info(tx, theme, env_id, self.external_host.as_ref().unwrap())
            .await?;

        self.web_dav = Some(new_web_dav);

        Ok(())
    }
}
