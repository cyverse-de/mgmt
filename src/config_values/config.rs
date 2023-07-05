use crate::config_values::{
    self, agave::Agave, base_urls::BaseURLs, dashboard_aggregator::DashboardAggregator,
    db::DatabaseConfig, db::QMSDatabaseConfig, de::DE, docker::Docker,
    elasticsearch::ElasticSearch, email::Email, grouper::Grouper, icat::Icat,
    infosquito::Infosquito,
};
use crate::db::{add_env_cfg_value, set_config_value, upsert_environment};
use dialoguer::{console::Style, theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigValues {
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
    elasticsearch: ElasticSearch,

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
            environment: String::new(),
            namespace: String::new(),
            uid_domain: String::new(),
            agave: Some(Agave::default()),
            base_urls: Some(BaseURLs::default()),
            dashboard_aggregator: Some(DashboardAggregator::default()),
            de: DE::default(),
            docker: Some(Docker::default()),
            elasticsearch: ElasticSearch::default(),
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
        self.de.ask_for_info(&theme)?;
        self.irods.ask_for_info(&theme)?;

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
            new_agave.ask_for_info(&theme, &base_uri, &irods_external)?;
            self.agave = Some(new_agave);
        }

        let mut new_da = DashboardAggregator::default();
        new_da.ask_for_info(&theme)?;
        self.dashboard_aggregator = Some(new_da);

        let mut new_docker = Docker::default();
        new_docker.ask_for_info(&theme)?;
        self.docker = Some(new_docker);

        self.elasticsearch.ask_for_info(&theme)?;
        self.email.ask_for_info(&theme)?;
        self.grouper.ask_for_info(&theme, &self.environment)?;
        self.icat.ask_for_info(&theme)?;

        let mut new_infosquito = Infosquito::default();
        new_infosquito.ask_for_info(&theme)?;
        self.infosquito = Some(new_infosquito);

        let intercom_enabled = Select::with_theme(&theme)
            .with_prompt("Include Intercom?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if intercom_enabled == 0 {
            let mut new_intercom = config_values::intercom::Intercom::default();
            new_intercom.ask_for_info(&theme)?;
            self.intercom = Some(new_intercom);
        }

        let mut new_jobs = config_values::misc::Jobs::default();
        new_jobs.ask_for_info(&theme)?;
        self.jobs = Some(new_jobs);

        self.keycloak.ask_for_info(&theme)?;
        self.pgp.ask_for_info(&theme)?;

        let permanent_id_enabled = Select::with_theme(&theme)
            .with_prompt("Include Permanent ID?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if permanent_id_enabled == 0 {
            let mut new_permanent_id = config_values::misc::PermanentId::default();
            new_permanent_id.ask_for_info(&theme)?;
            self.permanent_id = Some(new_permanent_id);
        }

        self.de_db.ask_for_info(&theme, "DE", "de", "", "de", "")?;
        self.grouper_db.ask_for_info(
            &theme,
            "Grouper",
            "grouper",
            &self.de_db.host,
            &self.de_db.user,
            &self.de_db.password,
        )?;
        self.notifications_db.ask_for_info(
            &theme,
            "Notifications",
            "notifications",
            &self.de_db.host,
            &self.de_db.user,
            &self.de_db.password,
        )?;
        self.permissions_db.ask_for_info(
            &theme,
            "Permissions",
            "permissions",
            &self.de_db.host,
            &self.de_db.user,
            &self.de_db.password,
        )?;
        self.metadata_db.ask_for_info(
            &theme,
            "Metadata",
            "metadata",
            &self.de_db.host,
            &self.de_db.user,
            &self.de_db.password,
        )?;

        let unleash_enabled = Select::with_theme(&theme)
            .with_prompt("Include Unleash?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if unleash_enabled == 0 {
            let mut new_unleash = config_values::misc::Unleash::default();
            new_unleash.ask_for_info(&theme)?;
            self.unleash = Some(new_unleash);
            self.unleash_db.ask_for_info(
                &theme,
                "Unleash",
                "unleash",
                &self.de_db.host,
                &self.de_db.user,
                &self.de_db.password,
            )?;
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
            self.qms_db.ask_for_info(
                &theme,
                "qms",
                &self.de_db.host,
                &self.de_db.user,
                &self.de_db.password,
            )?;
            let mut new_qms = config_values::misc::Qms::default();
            new_qms.ask_for_info(&theme)?;
            self.qms = Some(new_qms);
        }

        self.user_portal.ask_for_info(&theme)?;
        self.vice.ask_for_info(&theme)?;

        let mut new_admin = config_values::misc::Admin::default();
        new_admin.ask_for_info(&theme)?;
        self.admin = Some(new_admin);

        let analytics_enabled = Select::with_theme(&theme)
            .with_prompt("Include Analytics?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if analytics_enabled == 0 {
            let mut new_analytics = config_values::misc::Analytics::default();
            new_analytics.ask_for_info(&theme)?;
            self.analytics = Some(new_analytics);
        }

        let mut new_harbor = config_values::misc::Harbor::default();
        new_harbor.ask_for_info(&theme)?;
        self.harbor = Some(new_harbor);

        let jaeger_enabled = Select::with_theme(&theme)
            .with_prompt("Include Jaeger?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if jaeger_enabled == 0 {
            let mut new_jaeger = config_values::misc::Jaeger::default();
            new_jaeger.ask_for_info(&theme)?;
            self.jaeger = Some(new_jaeger);
        }

        Ok(())
    }
}
