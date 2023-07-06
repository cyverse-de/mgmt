use crate::db::{add_env_cfg_value, set_config_value};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ViceFileTransfers {
    image: Option<String>,
    tag: Option<String>,
}

impl Default for ViceFileTransfers {
    fn default() -> Self {
        ViceFileTransfers {
            image: Some(String::from("harbor.cyverse.org/de/vice-file-transfers")),
            tag: Some(String::from("latest")),
        }
    }
}

impl ViceFileTransfers {
    async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let image = Input::<String>::with_theme(theme)
            .with_prompt("Vice File Transfers Image")
            .default("harbor.cyverse.org/de/vice-file-transfers".into())
            .interact()?;

        let tag = Input::<String>::with_theme(theme)
            .with_prompt("Vice File Transfers Tag")
            .default("latest".into())
            .interact()?;

        let image_id = set_config_value(tx, "ViceFileTransfers", "Image", &image, "string").await?;
        add_env_cfg_value(tx, env_id, image_id).await?;

        let tag_id = set_config_value(tx, "ViceFileTransfers", "Tag", &tag, "string").await?;
        add_env_cfg_value(tx, env_id, tag_id).await?;

        self.image = Some(image);
        self.tag = Some(tag);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ViceDefaultBackend {
    loading_page_template_string: String,
}

impl ViceDefaultBackend {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
        base_url: &url::Url,
    ) -> anyhow::Result<()> {
        let lpt = base_url.join("/vice/{{.URL}}")?;
        let loading_page_template_string = Input::<String>::with_theme(theme)
            .with_prompt("Vice Default Backend Loading Page Template String")
            .default(lpt.to_string())
            .interact()?;

        let lpt_id = set_config_value(
            tx,
            "ViceDefaultBackend",
            "LoadingPageTemplateString",
            &loading_page_template_string,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, lpt_id).await?;
        self.loading_page_template_string = loading_page_template_string;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Vice {
    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,

    file_transfers: Option<ViceFileTransfers>,
    image_pull_secret: Option<String>,
    image_cache: Option<Vec<String>>,

    #[serde(rename = "UseCSIDriver")]
    use_csi_driver: Option<bool>,

    default_cas_url: Option<String>,
    default_cas_validate: Option<String>,
    use_case_chars_min: Option<u32>,
    default_backend: ViceDefaultBackend,
}

impl Default for Vice {
    fn default() -> Self {
        Vice {
            base_uri: None,
            file_transfers: Some(ViceFileTransfers::default()),
            image_pull_secret: Some(String::from("vice-image-pull-secret")),
            image_cache: Some(vec![
                String::from("harbor.cyverse.org/de/vice-proxy:latest"),
                String::from("harbor.cyverse.org/de/porklock:latest"),
                String::from("harbor.cyverse.org/de/vice-file-transfers:latest"),
                String::from("harbor.cyverse.org/vice/cli/bash:latest"),
                String::from("harbor.cyverse.org/legacy/datahog:beta"),
                String::from("harbor.cyverse.org/vice/jupyter/datascience:latest"),
                String::from("harbor.cyverse.org/vice/jupyter/rstudio:latest"),
                String::from("harbor.cyverse.org/vice/jupyter/geospatial:latest"),
                String::from("harbor.cyverse.org/vice/rstudio/rstudio"),
                String::from("harbor.cyverse.org/vice/rstudio/geospatial:latest"),
                String::from("harbor.cyverse.org/vice/rstudio/verse:latest"),
                String::from("harbor.cyverse.org/vice/rstudio/verse:latest"),
                String::from("harbor.cyverse.org/vice/vscode:latest"),
                String::from("harbor.cyverse.org/vice/xpra/qgis:20.04"),
                String::from("harbor.cyverse.org/vice/rstudio/stan:latest"),
            ]),
            use_csi_driver: Some(true),
            default_cas_url: Some(String::from("https://auth.cyverse.org/cas5")),
            default_cas_validate: Some(String::from("validate")),
            use_case_chars_min: Some(60),
            default_backend: ViceDefaultBackend::default(),
        }
    }
}

impl Vice {
    pub async fn ask_for_info(
        &mut self,
        tx: &mut Transaction<'_, MySql>,
        theme: &ColorfulTheme,
        env_id: u64,
    ) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("Vice Base URI")
            .interact()?;

        let image_pull_secret = Input::<String>::with_theme(theme)
            .with_prompt("Vice Image Pull Secret")
            .default("vice-image-pull-secret".into())
            .interact()?;

        let image_cache = Input::<String>::with_theme(theme)
            .with_prompt("Vice Image Cache")
            .default(
                "harbor.cyverse.org/de/vice-proxy:latest,harbor.cyverse.org/de/porklock:latest,harbor.cyverse.org/de/vice-file-transfers:latest,harbor.cyverse.org/vice/cli/bash:latest,harbor.cyverse.org/legacy/datahog:beta,harbor.cyverse.org/vice/jupyter/datascience:latest,harbor.cyverse.org/vice/jupyter/rstudio:latest,harbor.cyverse.org/vice/jupyter/geospatial:latest,harbor.cyverse.org/vice/rstudio/rstudio,harbor.cyverse.org/vice/rstudio/geospatial:latest,harbor.cyverse.org/vice/rstudio/verse:latest,harbor.cyverse.org/vice/rstudio/verse:latest,harbor.cyverse.org/vice/vscode:latest,harbor.cyverse.org/vice/xpra/qgis:20.04,harbor.cyverse.org/vice/rstudio/stan:latest"
                    .into(),
            )
            .interact()?;

        let default_cas_url = Input::<String>::with_theme(theme)
            .with_prompt("Vice Default CAS URL")
            .default("https://auth.cyverse.org/cas5".into())
            .interact()?;

        let default_cas_validate = Input::<String>::with_theme(theme)
            .with_prompt("Vice Default CAS Validate")
            .default("validate".into())
            .interact()?;

        let use_csi_data = Select::with_theme(theme)
            .with_prompt("Vice Use CSI Driver")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        let use_case_chars_min = Input::<u32>::with_theme(theme)
            .with_prompt("Vice Use Case Chars Min")
            .default(60)
            .interact()?;

        let mut new_file_transfers = ViceFileTransfers::default();
        new_file_transfers.ask_for_info(tx, theme, env_id).await?;
        self.file_transfers = Some(new_file_transfers);

        let base_uri_id = set_config_value(tx, "VICE", "BaseURI", &base_uri, "string").await?;
        add_env_cfg_value(tx, env_id, base_uri_id).await?;
        self.base_uri = Url::parse(&base_uri).ok();

        self.default_backend
            .ask_for_info(tx, theme, env_id, &self.base_uri.as_ref().unwrap())
            .await?;

        let image_pull_secret_id =
            set_config_value(tx, "VICE", "ImagePullSecret", &image_pull_secret, "string").await?;
        add_env_cfg_value(tx, env_id, image_pull_secret_id).await?;
        self.image_pull_secret = Some(image_pull_secret);

        let image_cache_id =
            set_config_value(tx, "VICE", "ImageCache", &image_cache, "string").await?;
        add_env_cfg_value(tx, env_id, image_cache_id).await?;
        self.image_cache = Some(image_cache.split(',').map(|s| s.to_string()).collect());

        let default_cas_url_id =
            set_config_value(tx, "VICE", "DefaultCasUrl", &default_cas_url, "string").await?;
        add_env_cfg_value(tx, env_id, default_cas_url_id).await?;
        self.default_cas_url = Some(default_cas_url);

        let default_cas_validate_id = set_config_value(
            tx,
            "VICE",
            "DefaultCasValidate",
            &default_cas_validate,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, default_cas_validate_id).await?;
        self.default_cas_validate = Some(default_cas_validate);

        let use_case_chars_min_id = set_config_value(
            tx,
            "VICE",
            "UseCaseCharsMin",
            &format!("{}", use_case_chars_min),
            "integer",
        )
        .await?;
        add_env_cfg_value(tx, env_id, use_case_chars_min_id).await?;
        self.use_case_chars_min = Some(use_case_chars_min);

        let use_csi_data_id = set_config_value(
            tx,
            "VICE",
            "UseCSIDriver",
            &format!("{}", use_csi_data == 0),
            "boolean",
        )
        .await?;
        add_env_cfg_value(tx, env_id, use_csi_data_id).await?;
        self.use_csi_driver = Some(use_csi_data == 0);

        Ok(())
    }
}
