use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KeycloakVice {
    #[serde(skip)]
    section: String,

    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,
}

impl Default for KeycloakVice {
    fn default() -> Self {
        KeycloakVice {
            section: "Keycloak".to_string(),
            client_id: String::new(),
            client_secret: String::new(),
        }
    }
}

impl LoadFromDatabase for KeycloakVice {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "VICE.ClientID" => self.client_id = value,
                "VICE.ClientSecret" => self.client_secret = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<KeycloakVice> for Vec<db::ConfigurationValue> {
    fn from(kv: KeycloakVice) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if kv.section.is_empty() {
            section = "Keycloak".to_string();
        } else {
            section = kv.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("VICE.ClientID".to_string()),
            value: Some(kv.client_id),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("VICE.ClientSecret".to_string()),
            value: Some(kv.client_secret),
            value_type: Some("string".to_string()),
        });

        vec
    }
}

impl KeycloakVice {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let client_id = Input::<String>::with_theme(theme)
            .with_prompt("Keycloak VICE Client ID")
            .default("de-vice".into())
            .interact()?;

        let client_secret = Password::with_theme(theme)
            .with_prompt("Keycloak VICE Client Secret")
            .interact()?;

        let client_id_id =
            set_config_value(tx, "Keycloak", "VICE.ClientID", &client_id, "string").await?;
        add_env_cfg_value(tx, env_id, client_id_id).await?;
        self.client_id = client_id;

        let client_secret_id = set_config_value(
            tx,
            "Keycloak",
            "VICE.ClientSecret",
            &client_secret,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, client_secret_id).await?;
        self.client_secret = client_secret;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Keycloak {
    #[serde(skip)]
    section: String,

    #[serde(rename = "ServerURI")]
    server_uri: Option<Url>,
    realm: String,

    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,

    #[serde(rename = "VICE")]
    vice: KeycloakVice,
}

impl Default for Keycloak {
    fn default() -> Self {
        Keycloak {
            section: "Keycloak".to_string(),
            server_uri: None,
            realm: String::new(),
            client_id: String::new(),
            client_secret: String::new(),
            vice: KeycloakVice::default(),
        }
    }
}

impl LoadFromDatabase for Keycloak {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "ServerURI" => self.server_uri = Url::parse(&value).ok(),
                "Realm" => self.realm = value,
                "ClientID" => self.client_id = value,
                "ClientSecret" => self.client_secret = value,
                _ => (),
            }

            if key.starts_with("VICE.") {
                self.vice.cfg_set_key(cfg)?;
            }
        }
        Ok(())
    }
}

impl From<Keycloak> for Vec<db::ConfigurationValue> {
    fn from(k: Keycloak) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if k.section.is_empty() {
            section = "Keycloak".to_string();
        } else {
            section = k.section.clone();
        }

        if let Some(server_uri) = k.server_uri {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("ServerURI".to_string()),
                value: Some(server_uri.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Realm".to_string()),
            value: Some(k.realm),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("ClientID".to_string()),
            value: Some(k.client_id),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("ClientSecret".to_string()),
            value: Some(k.client_secret),
            value_type: Some("string".to_string()),
        });

        vec.extend::<Vec<db::ConfigurationValue>>(k.vice.into());

        vec
    }
}

impl Keycloak {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let server_uri = Input::<String>::with_theme(theme)
            .with_prompt("Keycloak Server URI")
            .interact()?;

        let realm = Input::<String>::with_theme(theme)
            .with_prompt("Keycloak Realm")
            .default("CyVerse".into())
            .interact()?;

        let client_id = Input::<String>::with_theme(theme)
            .with_prompt("Keycloak Client ID")
            .default("de".into())
            .interact()?;

        let client_secret = Password::with_theme(theme)
            .with_prompt("Keycloak Client Secret")
            .interact()?;

        let server_uri_id =
            set_config_value(tx, "Keycloak", "ServerURI", &server_uri, "string").await?;
        add_env_cfg_value(tx, env_id, server_uri_id).await?;
        self.server_uri = Url::parse(&server_uri).ok();

        let realm_id = set_config_value(tx, "Keycloak", "Realm", &realm, "string").await?;
        add_env_cfg_value(tx, env_id, realm_id).await?;
        self.realm = realm;

        let client_id_id =
            set_config_value(tx, "Keycloak", "ClientID", &client_id, "string").await?;
        add_env_cfg_value(tx, env_id, client_id_id).await?;
        self.client_id = client_id;

        let client_secret_id =
            set_config_value(tx, "Keycloak", "ClientSecret", &client_secret, "string").await?;
        add_env_cfg_value(tx, env_id, client_secret_id).await?;
        self.client_secret = client_secret;

        self.vice.ask_for_info(tx, theme, env_id).await?;

        Ok(())
    }
}
