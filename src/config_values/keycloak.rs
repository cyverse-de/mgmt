use crate::db::{add_env_cfg_value, set_config_value, LoadFromConfiguration};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct KeycloakVice {
    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,
}

impl LoadFromConfiguration for KeycloakVice {
    fn get_section(&self) -> String {
        "Keycloak".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
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

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Keycloak {
    #[serde(rename = "ServerURI")]
    server_uri: Option<Url>,
    realm: String,

    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,

    #[serde(rename = "VICE")]
    vice: KeycloakVice,
}

impl LoadFromConfiguration for Keycloak {
    fn get_section(&self) -> String {
        "Keycloak".to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
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
