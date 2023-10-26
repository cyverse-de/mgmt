use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ViceFileTransfers {
    #[serde(skip)]
    section: String,
    image: Option<String>,
    tag: Option<String>,
}

impl Default for ViceFileTransfers {
    fn default() -> Self {
        ViceFileTransfers {
            section: "VICE".to_string(),
            image: None,
            tag: None,
        }
    }
}

impl LoadFromDatabase for ViceFileTransfers {
    fn get_section(&self) -> String {
        self.section.clone()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "FileTransfers.Image" => self.image = Some(value),
                "FileTransfers.Tag" => self.tag = Some(value),
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<ViceFileTransfers> for Vec<db::ConfigurationValue> {
    fn from(vft: ViceFileTransfers) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();

        let section: String;
        if vft.section.is_empty() {
            section = "VICE".to_string();
        } else {
            section = vft.section.clone();
        }

        if let Some(image) = vft.image {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("FileTransfers.Image".to_string()),
                value: Some(image),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(tag) = vft.tag {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("FileTransfers.Tag".to_string()),
                value: Some(tag),
                value_type: Some("string".to_string()),
            });
        }

        vec
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

        let image_id =
            set_config_value(tx, "VICE", "FileTransfers.Image", &image, "string").await?;
        add_env_cfg_value(tx, env_id, image_id).await?;

        let tag_id = set_config_value(tx, "VICE", "FileTransfers.Tag", &tag, "string").await?;
        add_env_cfg_value(tx, env_id, tag_id).await?;

        self.image = Some(image);
        self.tag = Some(tag);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ViceDefaultBackend {
    #[serde(skip)]
    section: String,

    loading_page_template_string: String,
}

impl Default for ViceDefaultBackend {
    fn default() -> Self {
        ViceDefaultBackend {
            section: "VICE".to_string(),
            loading_page_template_string: String::new(),
        }
    }
}

impl LoadFromDatabase for ViceDefaultBackend {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "DefaultBackend.LoadingPageTemplateString" => {
                    self.loading_page_template_string = value
                }
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<ViceDefaultBackend> for Vec<db::ConfigurationValue> {
    fn from(vdb: ViceDefaultBackend) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;
        if vdb.section.is_empty() {
            section = "VICE".to_string();
        } else {
            section = vdb.section.clone();
        }

        vec.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("DefaultBackend.LoadingPageTemplateString".to_string()),
            value: Some(vdb.loading_page_template_string),
            value_type: Some("string".to_string()),
        });

        vec
    }
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
            "VICE",
            "DefaultBackend.LoadingPageTemplateString",
            &loading_page_template_string,
            "string",
        )
        .await?;
        add_env_cfg_value(tx, env_id, lpt_id).await?;
        self.loading_page_template_string = loading_page_template_string;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Vice {
    #[serde(skip)]
    section: String,

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
            section: "VICE".to_string(),
            base_uri: None,
            file_transfers: Some(ViceFileTransfers::default()),
            image_pull_secret: None,
            image_cache: None,
            use_csi_driver: None,
            default_cas_url: None,
            default_cas_validate: None,
            use_case_chars_min: None,
            default_backend: ViceDefaultBackend::default(),
        }
    }
}

impl LoadFromDatabase for Vice {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "BaseURI" => self.base_uri = Url::parse(&value).ok(),
                "FileTransfers.Image" => {
                    if let Some(ft) = &mut self.file_transfers {
                        ft.image = Some(value);
                    }
                }
                "FileTransfers.Tag" => {
                    if let Some(ft) = &mut self.file_transfers {
                        ft.tag = Some(value);
                    }
                }
                "ImagePullSecret" => self.image_pull_secret = Some(value),
                "ImageCache" => {
                    self.image_cache = Some(value.split(',').map(|s| s.to_string()).collect())
                }
                "UseCSIDriver" => self.use_csi_driver = Some(value.parse::<bool>()?),
                "DefaultCasUrl" => self.default_cas_url = Some(value),
                "DefaultCasValidate" => self.default_cas_validate = Some(value),
                "UseCaseCharsMin" => self.use_case_chars_min = Some(value.parse::<u32>()?),
                "DefaultBackend.LoadingPageTemplateString" => {
                    self.default_backend.loading_page_template_string = value
                }
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<Vice> for Vec<db::ConfigurationValue> {
    fn from(v: Vice) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;

        if v.section.is_empty() {
            section = "VICE".to_string();
        } else {
            section = v.section.clone();
        }

        if let Some(base_uri) = v.base_uri {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("BaseURI".to_string()),
                value: Some(base_uri.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(ft) = v.file_transfers {
            vec.extend::<Vec<db::ConfigurationValue>>(ft.into());
        }

        if let Some(ips) = v.image_pull_secret {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("ImagePullSecret".to_string()),
                value: Some(ips),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(ic) = v.image_cache {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("ImageCache".to_string()),
                value: Some(ic.join(",")),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(ucd) = v.use_csi_driver {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("UseCSIDriver".to_string()),
                value: Some(format!("{}", ucd)),
                value_type: Some("bool".to_string()),
            });
        }

        if let Some(dcu) = v.default_cas_url {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("DefaultCasUrl".to_string()),
                value: Some(dcu),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(dcv) = v.default_cas_validate {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("DefaultCasValidate".to_string()),
                value: Some(dcv),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(ucm) = v.use_case_chars_min {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("UseCaseCharsMin".to_string()),
                value: Some(format!("{}", ucm)),
                value_type: Some("int".to_string()),
            });
        }

        vec.extend::<Vec<db::ConfigurationValue>>(v.default_backend.into());

        vec
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
            "int",
        )
        .await?;
        add_env_cfg_value(tx, env_id, use_case_chars_min_id).await?;
        self.use_case_chars_min = Some(use_case_chars_min);

        let use_csi_data_id = set_config_value(
            tx,
            "VICE",
            "UseCSIDriver",
            &format!("{}", use_csi_data == 0),
            "bool",
        )
        .await?;
        add_env_cfg_value(tx, env_id, use_csi_data_id).await?;
        self.use_csi_driver = Some(use_csi_data == 0);

        Ok(())
    }
}
