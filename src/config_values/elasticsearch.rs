use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ElasticSearch {
    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,

    username: String,
    password: String,
    index: String,
}

impl ElasticSearch {
    pub fn merge(&self, right: &ElasticSearch) -> anyhow::Result<ElasticSearch> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Base URI")
            .default("http://elasticsearch:9200".into())
            .interact()?;

        let username = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Username")
            .allow_empty(true)
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("ElasticSearch Password")
            .allow_empty_password(true)
            .interact()?;

        let index = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Index")
            .default("data".into())
            .interact()?;

        self.base_uri = Url::parse(&base_uri).ok();
        self.username = username;
        self.password = password;
        self.index = index;

        Ok(())
    }
}
