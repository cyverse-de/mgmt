use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct KeycloakVice {
    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,
}

impl KeycloakVice {
    fn merge(&self, right: &KeycloakVice) -> anyhow::Result<KeycloakVice> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let client_id = Input::<String>::with_theme(theme)
            .with_prompt("Keycloak VICE Client ID")
            .default("de-vice".into())
            .interact()?;

        let client_secret = Password::with_theme(theme)
            .with_prompt("Keycloak VICE Client Secret")
            .interact()?;

        self.client_id = client_id;
        self.client_secret = client_secret;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Keycloak {
    server_uri: Option<Url>,
    realm: String,

    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,

    #[serde(rename = "VICE")]
    vice: KeycloakVice,
}

impl Keycloak {
    pub fn merge(&self, right: &Keycloak) -> anyhow::Result<Keycloak> {
        let mut merged: Keycloak = serde_merge::omerge(&self, &right)?;
        merged.vice = self.vice.merge(&right.vice)?;
        Ok(merged)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
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

        self.server_uri = Url::parse(&server_uri).ok();
        self.realm = realm;
        self.client_id = client_id;
        self.client_secret = client_secret;

        self.vice.ask_for_info(theme)?;

        Ok(())
    }
}