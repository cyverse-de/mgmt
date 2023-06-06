use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
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
    pub fn merge(&self, right: &GrouperLoader) -> anyhow::Result<GrouperLoader> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let uri = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Loader URI")
            .interact()?;

        let user = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Loader User")
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("Grouper Loader Password")
            .interact()?;

        self.uri = Url::parse(&uri).ok();
        self.user = user;
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
    pub fn merge(&self, right: &Grouper) -> anyhow::Result<Grouper> {
        let mut merged: Grouper = serde_merge::omerge(&self, &right)?;
        merged.loader = self.loader.merge(&right.loader)?;
        Ok(merged)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme, env: &str) -> anyhow::Result<()> {
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

        self.morph_string = morph_string;
        self.password = password;
        self.folder_name_prefix = folder_name_prefix;
        self.loader.ask_for_info(theme)?;
        Ok(())
    }
}
