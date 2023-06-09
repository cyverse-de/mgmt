use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Ceph {
    password: String,
    username: String,
    first_name: String,
    last_name: String,
}

impl Ceph {
    fn merge(&self, right: &Ceph) -> anyhow::Result<Ceph> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let username = Input::<String>::with_theme(theme)
            .with_prompt("Ceph Username")
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("Ceph Password")
            .interact()?;

        let first_name = Input::<String>::with_theme(theme)
            .with_prompt("Ceph First Name")
            .interact()?;

        let last_name = Input::<String>::with_theme(theme)
            .with_prompt("Ceph Last Name")
            .interact()?;

        self.username = username;
        self.password = password;
        self.first_name = first_name;
        self.last_name = last_name;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DE {
    password: String,
    admin_password: String,
    username: String,
    first_name: String,
    last_name: String,
    local_user: String,
    jwt_name: String,
}

impl DE {
    fn merge(&self, right: &DE) -> anyhow::Result<DE> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let username = Input::<String>::with_theme(theme)
            .with_prompt("DE Username")
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("DE Password")
            .interact()?;

        let admin_password = Password::with_theme(theme)
            .with_prompt("DE Admin Password")
            .interact()?;

        let first_name = Input::<String>::with_theme(theme)
            .with_prompt("DE First Name")
            .interact()?;

        let last_name = Input::<String>::with_theme(theme)
            .with_prompt("DE Last Name")
            .interact()?;

        let local_user = Input::<String>::with_theme(theme)
            .with_prompt("DE Local User")
            .interact()?;

        let jwt_name = Input::<String>::with_theme(theme)
            .with_prompt("DE JWT Name")
            .interact()?;

        self.username = username;
        self.password = password;
        self.admin_password = admin_password;
        self.first_name = first_name;
        self.last_name = last_name;
        self.local_user = local_user;
        self.jwt_name = jwt_name;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Legacy {
    admin_password: String,
    admin_password2: String,
    password: String,
    jwt_priv_pass: String,
    jwt_name: String,
    username: String,
    first_name: String,
    last_name: String,
    local_user: String,
}

impl Legacy {
    fn merge(&self, right: &Legacy) -> anyhow::Result<Legacy> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let admin_password = Password::with_theme(theme)
            .with_prompt("Legacy Admin Password")
            .interact()?;
        let admin_password2 = Password::with_theme(theme)
            .with_prompt("Legacy Admin Password (again)")
            .interact()?;
        let password = Password::with_theme(theme)
            .with_prompt("Legacy Password")
            .interact()?;
        let jwt_priv_pass = Password::with_theme(theme)
            .with_prompt("Legacy JWT Private Pass")
            .interact()?;
        let jwt_name = Input::<String>::with_theme(theme)
            .with_prompt("Legacy JWT Name")
            .interact()?;
        let username = Input::<String>::with_theme(theme)
            .with_prompt("Legacy Username")
            .interact()?;
        let first_name = Input::<String>::with_theme(theme)
            .with_prompt("Legacy First Name")
            .interact()?;
        let last_name = Input::<String>::with_theme(theme)
            .with_prompt("Legacy Last Name")
            .interact()?;
        let local_user = Input::<String>::with_theme(theme)
            .with_prompt("Legacy Local User")
            .interact()?;
        self.admin_password = admin_password;
        self.admin_password2 = admin_password2;
        self.password = password;
        self.jwt_priv_pass = jwt_priv_pass;
        self.jwt_name = jwt_name;
        self.username = username;
        self.first_name = first_name;
        self.last_name = last_name;
        self.local_user = local_user;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Qa {
    ceph: Ceph,
    #[serde(rename = "DE")]
    de: DE,
    legacy: Legacy,
}

impl Qa {
    pub fn merge(&self, right: &Qa) -> anyhow::Result<Qa> {
        let mut merged: Qa = serde_merge::omerge(&self, &right)?;
        merged.ceph = self.ceph.merge(&right.ceph)?;
        merged.de = self.de.merge(&right.de)?;
        merged.legacy = self.legacy.merge(&right.legacy)?;
        Ok(merged)
    }

    pub fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        self.ceph.ask_for_info(theme).unwrap();
        self.de.ask_for_info(theme).unwrap();
        self.legacy.ask_for_info(theme).unwrap();
        Ok(())
    }
}
