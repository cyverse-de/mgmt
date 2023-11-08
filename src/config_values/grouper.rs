use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GrouperLoader {
    #[serde(skip)]
    section: String,

    #[serde(rename = "URI")]
    uri: Option<Url>,

    user: String,
    password: String,
}

impl Default for GrouperLoader {
    fn default() -> Self {
        GrouperLoader {
            section: "Grouper".to_string(),
            uri: None,
            user: String::new(),
            password: String::new(),
        }
    }
}

impl LoadFromDatabase for GrouperLoader {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Loader.URI" => self.uri = Url::parse(&value).ok(),
                "Loader.User" => self.user = value,
                "Loader.Password" => self.password = value,
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<GrouperLoader> for Vec<db::ConfigurationValue> {
    fn from(gl: GrouperLoader) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if gl.section.is_empty() {
            section = "Grouper".to_string();
        } else {
            section = gl.section.clone();
        }

        if let Some(uri) = gl.uri {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Loader.URI".to_string()),
                value: Some(uri.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Loader.User".to_string()),
            value: Some(gl.user),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Loader.Password".to_string()),
            value: Some(gl.password),
            value_type: Some("string".to_string()),
        });

        vec
    }
}

impl GrouperLoader {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let uri = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Loader URI")
            .interact()?;

        let user = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Loader User")
            .interact()?;

        let password = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Loader Password")
            .interact()?;

        let uri_id = set_config_value(tx, "Grouper", "Loader.URI", &uri, "string").await?;
        add_env_cfg_value(tx, env_id, uri_id).await?;
        self.uri = Url::parse(&uri).ok();

        let user_id = set_config_value(tx, "Grouper", "Loader.User", &user, "string").await?;
        add_env_cfg_value(tx, env_id, user_id).await?;
        self.user = user;

        let password_id =
            set_config_value(tx, "Grouper", "Loader.Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Grouper {
    #[serde(skip)]
    section: String,
    morph_string: String,
    password: String,
    folder_name_prefix: String,
    loader: GrouperLoader,
}

impl Default for Grouper {
    fn default() -> Self {
        Grouper {
            section: "Grouper".to_string(),
            morph_string: String::new(),
            password: String::new(),
            folder_name_prefix: String::new(),
            loader: GrouperLoader::default(),
        }
    }
}

impl LoadFromDatabase for Grouper {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "MorphString" => self.morph_string = value,
                "Password" => self.password = value,
                "FolderNamePrefix" => self.folder_name_prefix = value,
                _ => (),
            }

            if key.starts_with("Loader.") {
                self.loader.cfg_set_key(cfg)?;
            }
        }

        Ok(())
    }
}

impl From<Grouper> for Vec<db::ConfigurationValue> {
    fn from(g: Grouper) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if g.section.is_empty() {
            section = "Grouper".to_string();
        } else {
            section = g.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("MorphString".to_string()),
            value: Some(g.morph_string),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Password".to_string()),
            value: Some(g.password),
            value_type: Some("string".to_string()),
        });

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("FolderNamePrefix".to_string()),
            value: Some(g.folder_name_prefix),
            value_type: Some("string".to_string()),
        });

        vec.extend::<Vec<db::ConfigurationValue>>(g.loader.into());

        vec
    }
}

impl Grouper {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
        env: &str,
    ) -> anyhow::Result<()> {
        let morph_string = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Morph String")
            .interact()?;

        let password = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Password")
            .interact()?;

        let folder_name_prefix = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Folder Name Prefix")
            .default(format!("cyverse:de:{}", env).into())
            .interact()?;

        let morph_string_id =
            set_config_value(tx, "Grouper", "MorphString", &morph_string, "string").await?;
        add_env_cfg_value(tx, env_id, morph_string_id).await?;
        self.morph_string = morph_string;

        let password_id = set_config_value(tx, "Grouper", "Password", &password, "string").await?;
        add_env_cfg_value(tx, env_id, password_id).await?;
        self.password = password;

        let folder_name_prefix_id = set_config_value(
            tx,
            "Grouper",
            "FolderNamePrefix",
            &folder_name_prefix,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, folder_name_prefix_id).await?;
        self.folder_name_prefix = folder_name_prefix;

        self.loader.ask_for_info(tx, theme, env_id).await?;

        Ok(())
    }
}
