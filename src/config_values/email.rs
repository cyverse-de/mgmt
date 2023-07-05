use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Email {
    src: String,
    dest: String,

    #[serde(rename = "PermIDRequestDest")]
    perm_id_request_dest: String,

    support_dest: String,
}

impl Email {
    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let src = Input::<String>::with_theme(theme)
            .with_prompt("Email Source")
            .interact()?;

        let dest = Input::<String>::with_theme(theme)
            .with_prompt("Email Destination")
            .interact()?;

        let perm_id_default = dest.clone();
        let perm_id_request_dest = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID Request Destination")
            .default(perm_id_default)
            .interact()?;

        let support_dest_default = dest.clone();
        let support_dest = Input::<String>::with_theme(theme)
            .with_prompt("Support Destination")
            .default(support_dest_default)
            .interact()?;

        self.src = src;
        self.dest = dest;
        self.perm_id_request_dest = perm_id_request_dest;
        self.support_dest = support_dest;

        Ok(())
    }
}
