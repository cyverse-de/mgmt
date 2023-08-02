use crate::config_values::{
    self, agave::Agave, base_urls::BaseURLs, dashboard_aggregator::DashboardAggregator,
    db::DatabaseConfig, db::QMSDatabaseConfig, de::DE, docker::Docker,
    elasticsearch::Elasticsearch, email::Email, grouper::Grouper, icat::Icat,
    infosquito::Infosquito,
};
use crate::db::{
    self, add_env_cfg_value, set_config_value, upsert_environment, LoadFromConfiguration,
};
use dialoguer::{console::Style, theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigValues {
    #[serde(skip)]
    section: String,

    // Must be user supplied.
    environment: String,

    // Must be user supplied.
    namespace: String,

    // Must be user supplied.
    #[serde(rename = "UIDDomain")]
    uid_domain: String,

    // Optional for deployment.
    agave: Option<Agave>,

    // Defaults are provided for deployment.
    #[serde(rename = "BaseURLs")]
    base_urls: Option<BaseURLs>,

    // Defaults are provided for deployment (or will be).
    dashboard_aggregator: Option<DashboardAggregator>,

    // Contains settings that must be provided for deployment.
    #[serde(rename = "DE")]
    de: DE,

    // Defaults are provided for deployment.
    docker: Option<Docker>,

    // Must be configured for deplyoment.
    elasticsearch: Elasticsearch,

    // Must be configured for deployment.
    email: Email,

    // Must be configured for deployment.
    grouper: Grouper,

    // Must be configured for deployment.
    #[serde(rename = "ICAT")]
    icat: Icat,

    // Defaults provided for deployment.
    infosquito: Option<Infosquito>,

    // Optional for deployment
    intercom: Option<config_values::intercom::Intercom>,

    // Must be configured for deployment.
    #[serde(rename = "IRODS")]
    irods: config_values::irods::Irods,

    // Defaults are provided for deployment.
    jobs: Option<config_values::misc::Jobs>,

    // Must be configured for deployment.
    keycloak: config_values::keycloak::Keycloak,

    // Must be configured for deployment.
    #[serde(rename = "PGP")]
    pgp: config_values::misc::Pgp,

    // Optional for deployment.
    #[serde(rename = "PermanentID")]
    permanent_id: Option<config_values::misc::PermanentId>,

    // Defaults provided for deployment.
    timezone: Option<String>,

    // Optional for deployment.
    unleash: Option<config_values::misc::Unleash>,

    // Required for deployment
    user_portal: config_values::misc::UserPortal,

    // Required for deployment.
    #[serde(rename = "VICE")]
    vice: config_values::vice::Vice,

    // Required for deployment.
    #[serde(rename = "DEDB")]
    de_db: DatabaseConfig,

    // Required for deployment.
    #[serde(rename = "GrouperDB")]
    grouper_db: DatabaseConfig,

    // Required for deployment.
    #[serde(rename = "NewNotificationsDB")]
    new_notifications_db: DatabaseConfig,

    // Required for deployment.
    #[serde(rename = "NotificationsDB")]
    notifications_db: DatabaseConfig,

    // Required for deployment.
    #[serde(rename = "PermissionsDB")]
    permissions_db: DatabaseConfig,

    // Not required for a deployment
    #[serde(rename = "QA")]
    qa: Option<config_values::qa::Qa>,

    // Required for deployment.
    #[serde(rename = "QMSDB")]
    qms_db: QMSDatabaseConfig,

    // Required for deployment.
    #[serde(rename = "MetadataDB")]
    metadata_db: DatabaseConfig,

    // Required for deployment.
    #[serde(rename = "UnleashDB")]
    unleash_db: DatabaseConfig,

    // Defaults provided.
    admin: Option<config_values::misc::Admin>,

    // Optional for deployment.
    analytics: Option<config_values::misc::Analytics>,

    // Defaults provided for deployment.
    harbor: Option<config_values::misc::Harbor>,

    // Optional for deployment.
    #[serde(rename = "QMS")]
    qms: Option<config_values::misc::Qms>,

    // Optional for deployment.
    jaeger: Option<config_values::misc::Jaeger>,
}

impl Default for ConfigValues {
    fn default() -> Self {
        ConfigValues {
            section: "TopLevel".to_string(),
            environment: String::new(),
            namespace: String::new(),
            uid_domain: String::new(),
            agave: Some(Agave::default()),
            base_urls: Some(BaseURLs::default()),
            dashboard_aggregator: Some(DashboardAggregator::default()),
            de: DE::default(),
            docker: Some(Docker::default()),
            elasticsearch: Elasticsearch::default(),
            email: Email::default(),
            grouper: Grouper::default(),
            icat: Icat::default(),
            infosquito: Some(Infosquito::default()),
            intercom: Some(config_values::intercom::Intercom::default()),
            irods: config_values::irods::Irods::default(),
            jobs: Some(config_values::misc::Jobs::default()),
            keycloak: config_values::keycloak::Keycloak::default(),
            pgp: config_values::misc::Pgp::default(),
            permanent_id: Some(config_values::misc::PermanentId::default()),
            timezone: Some(String::new()),
            unleash: Some(config_values::misc::Unleash::default()),
            user_portal: config_values::misc::UserPortal::default(),
            vice: config_values::vice::Vice::default(),
            de_db: DatabaseConfig::default(),
            grouper_db: DatabaseConfig::default(),
            new_notifications_db: DatabaseConfig::default(),
            notifications_db: DatabaseConfig::default(),
            permissions_db: DatabaseConfig::default(),
            qms_db: QMSDatabaseConfig::default(),
            metadata_db: DatabaseConfig::default(),
            unleash_db: DatabaseConfig::default(),
            admin: Some(config_values::misc::Admin::default()),
            analytics: Some(config_values::misc::Analytics::default()),
            harbor: Some(config_values::misc::Harbor::default()),
            qa: Some(config_values::qa::Qa::default()),
            qms: Some(config_values::misc::Qms::default()),
            jaeger: Some(config_values::misc::Jaeger::default()),
        }
    }
}

impl LoadFromConfiguration for ConfigValues {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::Configuration) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Environment" => self.environment = value,
                "Namespace" => self.namespace = value,
                "UIDDomain" => self.uid_domain = value,
                _ => (),
            }
        }
        Ok(())
    }

    fn cfg_set_keys(&mut self, cfgs: Vec<crate::db::Configuration>) -> anyhow::Result<()> {
        cfgs.iter().for_each(|cfg| {
            if let Some(section) = cfg.section.clone() {
                match section.as_str() {
                    "Agave" => {
                        if let Some(agave) = &mut self.agave {
                            agave.cfg_set_key(cfg).ok();
                        }
                    }
                    "BaseURLs" => {
                        if let Some(base_urls) = &mut self.base_urls {
                            base_urls.cfg_set_key(cfg).ok();
                        }
                    }
                    "DashboardAggregator" => {
                        if let Some(dashboard_aggregator) = &mut self.dashboard_aggregator {
                            dashboard_aggregator.cfg_set_key(cfg).ok();
                        }
                    }
                    "DE" => {
                        self.de.cfg_set_key(cfg).ok();
                    }
                    "Docker" => {
                        if let Some(docker) = &mut self.docker {
                            docker.cfg_set_key(cfg).ok();
                        }
                    }
                    "Elasticsearch" => {
                        self.elasticsearch.cfg_set_key(cfg).ok();
                    }
                    "Email" => {
                        self.email.cfg_set_key(cfg).ok();
                    }
                    "Grouper" => {
                        self.grouper.cfg_set_key(cfg).ok();
                    }
                    "ICAT" => {
                        self.icat.cfg_set_key(cfg).ok();
                    }
                    "Infosquito" => {
                        if let Some(infosquito) = &mut self.infosquito {
                            infosquito.cfg_set_key(cfg).ok();
                        }
                    }
                    "Intercom" => {
                        if let Some(intercom) = &mut self.intercom {
                            intercom.cfg_set_key(cfg).ok();
                        }
                    }
                    "IRODS" => {
                        self.irods.cfg_set_key(cfg).ok();
                    }
                    "Jobs" => {
                        if let Some(jobs) = &mut self.jobs {
                            jobs.cfg_set_key(cfg).ok();
                        }
                    }
                    "Keycloak" => {
                        self.keycloak.cfg_set_key(cfg).ok();
                    }
                    "PGP" => {
                        self.pgp.cfg_set_key(cfg).ok();
                    }
                    "PermanentID" => {
                        if let Some(permanent_id) = &mut self.permanent_id {
                            permanent_id.cfg_set_key(cfg).ok();
                        }
                    }
                    "Unleash" => {
                        if let Some(unleash) = &mut self.unleash {
                            unleash.cfg_set_key(cfg).ok();
                        }
                    }
                    "UserPortal" => {
                        self.user_portal.cfg_set_key(cfg).ok();
                    }
                    "VICE" => {
                        self.vice.cfg_set_key(cfg).ok();
                    }
                    "DEDB" => {
                        self.de_db.cfg_set_key(cfg).ok();
                    }
                    "GrouperDB" => {
                        self.grouper_db.cfg_set_key(cfg).ok();
                    }
                    "NotificationsDB" => {
                        self.notifications_db.cfg_set_key(cfg).ok();
                    }
                    "PermissionsDB" => {
                        self.permissions_db.cfg_set_key(cfg).ok();
                    }
                    "QMSDB" => {
                        self.qms_db.cfg_set_key(cfg).ok();
                    }
                    "MetadataDB" => {
                        self.metadata_db.cfg_set_key(cfg).ok();
                    }
                    "UnleashDB" => {
                        self.unleash_db.cfg_set_key(cfg).ok();
                    }
                    "Admin" => {
                        if let Some(admin) = &mut self.admin {
                            admin.cfg_set_key(cfg).ok();
                        }
                    }
                    "Analytics" => {
                        if let Some(analytics) = &mut self.analytics {
                            analytics.cfg_set_key(cfg).ok();
                        }
                    }
                    "Harbor" => {
                        if let Some(harbor) = &mut self.harbor {
                            harbor.cfg_set_key(cfg).ok();
                        }
                    }
                    "QMS" => {
                        if let Some(qms) = &mut self.qms {
                            qms.cfg_set_key(cfg).ok();
                        }
                    }
                    "Jaeger" => {
                        if let Some(jaeger) = &mut self.jaeger {
                            jaeger.cfg_set_key(cfg).ok();
                        }
                    }
                    "TopLevel" => {
                        self.cfg_set_key(cfg).ok();
                    }

                    "" => {
                        self.cfg_set_key(cfg).ok();
                    }

                    _ => (),
                }
            }
        });
        Ok(())
    }
}

impl From<ConfigValues> for Vec<db::Configuration> {
    fn from(cv: ConfigValues) -> Vec<db::Configuration> {
        let mut cfgs: Vec<db::Configuration> = Vec::new();

        let section: String;
        if cv.section.is_empty() {
            section = "TopLevel".to_string();
        } else {
            section = cv.section.clone();
        }

        cfgs.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("Environment".to_string()),
            value: Some(cv.environment),
            value_type: Some("string".to_string()),
        });

        cfgs.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("Namespace".to_string()),
            value: Some(cv.namespace),
            value_type: Some("string".to_string()),
        });

        cfgs.push(db::Configuration {
            id: None,
            section: Some(section.clone()),
            key: Some("UIDDomain".to_string()),
            value: Some(cv.uid_domain),
            value_type: Some("string".to_string()),
        });

        if let Some(agave) = cv.agave {
            cfgs.extend::<Vec<db::Configuration>>(agave.into());
        }

        if let Some(base_urls) = cv.base_urls {
            cfgs.extend::<Vec<db::Configuration>>(base_urls.into());
        }

        if let Some(dashboard_aggregator) = cv.dashboard_aggregator {
            cfgs.extend::<Vec<db::Configuration>>(dashboard_aggregator.into());
        }

        cfgs.extend::<Vec<db::Configuration>>(cv.de.into());

        if let Some(docker) = cv.docker {
            cfgs.extend::<Vec<db::Configuration>>(docker.into());
        }

        cfgs.extend::<Vec<db::Configuration>>(cv.elasticsearch.into());

        cfgs.extend::<Vec<db::Configuration>>(cv.email.into());

        cfgs.extend::<Vec<db::Configuration>>(cv.grouper.into());

        cfgs.extend::<Vec<db::Configuration>>(cv.icat.into());

        if let Some(infosquito) = cv.infosquito {
            cfgs.extend::<Vec<db::Configuration>>(infosquito.into());
        }

        if let Some(intercom) = cv.intercom {
            cfgs.extend::<Vec<db::Configuration>>(intercom.into());
        }

        cfgs.extend::<Vec<db::Configuration>>(cv.irods.into());

        if let Some(jobs) = cv.jobs {
            cfgs.extend::<Vec<db::Configuration>>(jobs.into());
        }

        cfgs.extend::<Vec<db::Configuration>>(cv.keycloak.into());

        cfgs.extend::<Vec<db::Configuration>>(cv.pgp.into());

        if let Some(permanent_id) = cv.permanent_id {
            cfgs.extend::<Vec<db::Configuration>>(permanent_id.into());
        }

        if let Some(timezone) = cv.timezone {
            cfgs.push(db::Configuration {
                id: None,
                section: Some(section.clone()),
                key: Some("Timezone".to_string()),
                value: Some(timezone),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(unleash) = cv.unleash {
            cfgs.extend::<Vec<db::Configuration>>(unleash.into());
        }

        cfgs.extend::<Vec<db::Configuration>>(cv.user_portal.into());

        cfgs.extend::<Vec<db::Configuration>>(cv.vice.into());

        let mut de_db_cfgs: Vec<db::Configuration> = cv.de_db.into();
        de_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("DEDB".to_string());
        });
        cfgs.extend::<Vec<db::Configuration>>(de_db_cfgs.into());

        let mut grouper_db_cfgs: Vec<db::Configuration> = cv.grouper_db.into();
        grouper_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("GrouperDB".to_string());
        });
        cfgs.extend::<Vec<db::Configuration>>(grouper_db_cfgs.into());

        // let mut new_notifications_db_cfgs: Vec<db::Configuration> = cv.new_notifications_db.into();
        // new_notifications_db_cfgs.iter_mut().for_each(|cfg| {
        //     cfg.section = Some("NewNotificationsDB".to_string());
        // });
        // cfgs.extend::<Vec<db::Configuration>>(new_notifications_db_cfgs.into());

        let mut notifications_db_cfgs: Vec<db::Configuration> = cv.notifications_db.into();
        notifications_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("NotificationsDB".to_string());
        });
        cfgs.extend::<Vec<db::Configuration>>(notifications_db_cfgs.into());

        let mut permissions_db_cfgs: Vec<db::Configuration> = cv.permissions_db.into();
        permissions_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("PermissionsDB".to_string());
        });
        cfgs.extend::<Vec<db::Configuration>>(permissions_db_cfgs.into());

        let mut qms_db_cfgs: Vec<db::Configuration> = cv.qms_db.into();
        qms_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("QMSDB".to_string());
        });
        cfgs.extend::<Vec<db::Configuration>>(qms_db_cfgs.into());

        let mut metadata_db_cfgs: Vec<db::Configuration> = cv.metadata_db.into();
        metadata_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("MetadataDB".to_string());
        });
        cfgs.extend::<Vec<db::Configuration>>(metadata_db_cfgs.into());

        let mut unleash_db_cfgs: Vec<db::Configuration> = cv.unleash_db.into();
        unleash_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("UnleashDB".to_string());
        });
        cfgs.extend::<Vec<db::Configuration>>(unleash_db_cfgs.into());

        if let Some(admin) = cv.admin {
            cfgs.extend::<Vec<db::Configuration>>(admin.into());
        }

        if let Some(analytics) = cv.analytics {
            cfgs.extend::<Vec<db::Configuration>>(analytics.into());
        }

        if let Some(harbor) = cv.harbor {
            cfgs.extend::<Vec<db::Configuration>>(harbor.into());
        }

        if let Some(qms) = cv.qms {
            cfgs.extend::<Vec<db::Configuration>>(qms.into());
        }

        if let Some(jaeger) = cv.jaeger {
            cfgs.extend::<Vec<db::Configuration>>(jaeger.into());
        }

        cfgs
    }
}

impl ConfigValues {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn ask_for_info(&mut self, tx: &mut Transaction<'_, MySql>) -> anyhow::Result<()> {
        let mut theme = ColorfulTheme::default();
        theme.hint_style = Style::new().yellow();

        let environment = Input::<String>::with_theme(&theme)
            .with_prompt("Environment")
            .interact()?;

        let namespace = Input::<String>::with_theme(&theme)
            .with_prompt("Namespace")
            .default(environment.clone())
            .interact()?;

        let uid_domain = Input::<String>::with_theme(&theme)
            .with_prompt("UID Domain")
            .interact()?;

        let timezone = Input::<String>::with_theme(&theme)
            .with_prompt("Timezone")
            .default("America/Phoenix".to_string())
            .interact()?;

        let env_id = upsert_environment(tx, &environment, &namespace).await?;

        let env_cfg_id =
            set_config_value(tx, "TopLevel", "Environment", &environment, "string").await?;
        add_env_cfg_value(tx, env_id, env_cfg_id).await?;

        let namespace_id =
            set_config_value(tx, "TopLevel", "Namespace", &namespace, "string").await?;
        add_env_cfg_value(tx, env_id, namespace_id).await?;

        let uid_id = set_config_value(tx, "TopLevel", "UIDDomain", &uid_domain, "string").await?;
        add_env_cfg_value(tx, env_id, uid_id).await?;

        let timezone_id = set_config_value(tx, "TopLevel", "Timezone", &timezone, "string").await?;
        add_env_cfg_value(tx, env_id, timezone_id).await?;

        // Fill in the DE and iRODS settings first, since they have some
        // values that can be used as defaults later.
        self.de.ask_for_info(tx, &theme, env_id).await?;
        self.irods.ask_for_info(tx, &theme, env_id).await?;

        // We need the base URI and external host for other settings.
        let base_uri = self.de.base_uri.clone().unwrap();
        let irods_external = self.irods.external_host.clone().unwrap();

        let agave_enabled = Select::with_theme(&theme)
            .with_prompt("Include Agave?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if agave_enabled == 0 {
            let mut new_agave = Agave::default();
            new_agave
                .ask_for_info(tx, &theme, env_id, &base_uri, &irods_external)
                .await?;
            self.agave = Some(new_agave);
        }

        let mut new_da = DashboardAggregator::default();
        new_da.ask_for_info(tx, &theme, env_id).await?;
        self.dashboard_aggregator = Some(new_da);

        let mut new_docker = Docker::default();
        new_docker.ask_for_info(tx, &theme, env_id).await?;
        self.docker = Some(new_docker);

        self.elasticsearch.ask_for_info(tx, &theme, env_id).await?;
        self.email.ask_for_info(tx, &theme, env_id).await?;
        self.grouper
            .ask_for_info(tx, &theme, env_id, &self.environment)
            .await?;
        self.icat.ask_for_info(tx, &theme, env_id).await?;

        let mut new_infosquito = Infosquito::default();
        new_infosquito.ask_for_info(tx, &theme, env_id).await?;
        self.infosquito = Some(new_infosquito);

        let intercom_enabled = Select::with_theme(&theme)
            .with_prompt("Include Intercom?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if intercom_enabled == 0 {
            let mut new_intercom = config_values::intercom::Intercom::default();
            new_intercom.ask_for_info(tx, &theme, env_id).await?;
            self.intercom = Some(new_intercom);
        }

        let mut new_jobs = config_values::misc::Jobs::default();
        new_jobs.ask_for_info(tx, &theme, env_id).await?;
        self.jobs = Some(new_jobs);

        self.keycloak.ask_for_info(tx, &theme, env_id).await?;
        self.pgp.ask_for_info(tx, &theme, env_id).await?;

        let permanent_id_enabled = Select::with_theme(&theme)
            .with_prompt("Include Permanent ID?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if permanent_id_enabled == 0 {
            let mut new_permanent_id = config_values::misc::PermanentId::default();
            new_permanent_id.ask_for_info(tx, &theme, env_id).await?;
            self.permanent_id = Some(new_permanent_id);
        }

        self.de_db
            .ask_for_info(tx, &theme, env_id, "DE", "DEDB", "de", "", "de", "")
            .await?;
        self.grouper_db
            .ask_for_info(
                tx,
                &theme,
                env_id,
                "Grouper",
                "GrouperDB",
                "grouper",
                &self.de_db.host,
                &self.de_db.user,
                &self.de_db.password,
            )
            .await?;
        self.notifications_db
            .ask_for_info(
                tx,
                &theme,
                env_id,
                "Notifications",
                "NotificationsDB",
                "notifications",
                &self.de_db.host,
                &self.de_db.user,
                &self.de_db.password,
            )
            .await?;
        self.permissions_db
            .ask_for_info(
                tx,
                &theme,
                env_id,
                "Permissions",
                "PermissionsDB",
                "permissions",
                &self.de_db.host,
                &self.de_db.user,
                &self.de_db.password,
            )
            .await?;
        self.metadata_db
            .ask_for_info(
                tx,
                &theme,
                env_id,
                "Metadata",
                "MetadataDB",
                "metadata",
                &self.de_db.host,
                &self.de_db.user,
                &self.de_db.password,
            )
            .await?;

        let unleash_enabled = Select::with_theme(&theme)
            .with_prompt("Include Unleash?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if unleash_enabled == 0 {
            let mut new_unleash = config_values::misc::Unleash::default();
            new_unleash.ask_for_info(tx, &theme, env_id).await?;
            self.unleash = Some(new_unleash);
            self.unleash_db
                .ask_for_info(
                    tx,
                    &theme,
                    env_id,
                    "Unleash",
                    "UnleashDB",
                    "unleash",
                    &self.de_db.host,
                    &self.de_db.user,
                    &self.de_db.password,
                )
                .await?;
        }

        let qa_enabled = Select::with_theme(&theme)
            .with_prompt("Include QA?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if qa_enabled == 0 {
            let mut new_qa = config_values::qa::Qa::default();
            new_qa.ask_for_info(&theme)?;
            self.qa = Some(new_qa);
        }

        let qms_enabled = Select::with_theme(&theme)
            .with_prompt("Include QMS?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if qms_enabled == 0 {
            self.qms_db
                .ask_for_info(
                    tx,
                    &theme,
                    env_id,
                    "qms",
                    &self.de_db.host,
                    &self.de_db.user,
                    &self.de_db.password,
                )
                .await?;
            let mut new_qms = config_values::misc::Qms::default();
            new_qms.ask_for_info(tx, &theme, env_id).await?;
            self.qms = Some(new_qms);
        }

        self.user_portal.ask_for_info(tx, &theme, env_id).await?;
        self.vice.ask_for_info(tx, &theme, env_id).await?;

        let mut new_admin = config_values::misc::Admin::default();
        new_admin.ask_for_info(tx, &theme, env_id).await?;
        self.admin = Some(new_admin);

        let analytics_enabled = Select::with_theme(&theme)
            .with_prompt("Include Analytics?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if analytics_enabled == 0 {
            let mut new_analytics = config_values::misc::Analytics::default();
            new_analytics.ask_for_info(tx, &theme, env_id).await?;
            self.analytics = Some(new_analytics);
        }

        let mut new_harbor = config_values::misc::Harbor::default();
        new_harbor.ask_for_info(tx, &theme, env_id).await?;
        self.harbor = Some(new_harbor);

        let jaeger_enabled = Select::with_theme(&theme)
            .with_prompt("Include Jaeger?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if jaeger_enabled == 0 {
            let mut new_jaeger = config_values::misc::Jaeger::default();
            new_jaeger.ask_for_info(tx, &theme, env_id).await?;
            self.jaeger = Some(new_jaeger);
        }

        Ok(())
    }
}
