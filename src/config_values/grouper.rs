use crate::db::{add_env_cfg_value, set_config_value};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GrouperLoader {
    #[serde(rename = "URI")]
    uri: Option<Url>,

    user: String,
    password: String,
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

        let password = Password::with_theme(theme)
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

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Grouper {
    morph_string: String,
    password: String,
    folder_name_prefix: String,
    loader: GrouperLoader,
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

        let password = Password::with_theme(theme)
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
