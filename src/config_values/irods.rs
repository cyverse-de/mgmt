use crate::config_values::amqp::Amqp;
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IrodsWebDav {
    #[serde(rename = "AnonURI")]
    anon_uri: Option<Url>,
}

impl Default for IrodsWebDav {
    fn default() -> Self {
        IrodsWebDav {
            anon_uri: Url::parse("https://data.cyverse.rocks/dav-anon").ok(),
        }
    }
}

impl IrodsWebDav {
    fn ask_for_info(&mut self, theme: &ColorfulTheme, external: &str) -> anyhow::Result<()> {
        let anon_uri = Input::<String>::with_theme(theme)
            .with_prompt("Irods WebDav Anon URI")
            .default(format!("https://{}/dav-anon", external))
            .interact()?;

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
    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        self.amqp.ask_for_info(theme, "iRODS")?;

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

        self.host = host;
        self.external_host = Some(external_host);
        self.user = user;
        self.zone = zone;
        self.password = password;
        self.admin_users = admin_users.split(',').map(|s| s.to_string()).collect();
        self.perms_filter = perms_filter.split(',').map(|s| s.to_string()).collect();

        let mut new_web_dav = IrodsWebDav::default();

        // We're okay with unwrap here since it's user input and panicking is fine.
        new_web_dav.ask_for_info(theme, self.external_host.as_ref().unwrap())?;

        self.web_dav = Some(new_web_dav);

        Ok(())
    }
}
