use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Docker {
    trusted_registries: Option<Vec<String>>,
    tag: String,
}

impl Default for Docker {
    fn default() -> Self {
        Docker {
            tag: String::from("latest"),
            trusted_registries: Some(vec![
                String::from("harbor.cyverse.org"),
                String::from("docker.cyverse.org"),
            ]),
        }
    }
}

impl Docker {
    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let tag = Input::<String>::with_theme(theme)
            .with_prompt("Docker Tag")
            .default("latest".into())
            .interact()?;

        self.tag = tag;

        Ok(())
    }
}
