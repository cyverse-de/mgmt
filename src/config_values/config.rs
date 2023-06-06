use crate::config_values::{
    self, agave::Agave, base_urls::BaseURLs, dashboard_aggregator::DashboardAggregator,
    db::DatabaseConfig, db::QMSDatabaseConfig, de::DE, docker::Docker,
    elasticsearch::ElasticSearch, email::Email, grouper::Grouper, icat::Icat,
    infosquito::Infosquito,
};
use dialoguer::{console::Style, theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};

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
            qms: Some(config_values::misc::Qms::default()),
            jaeger: Some(config_values::misc::Jaeger::default()),
        }
    }
}

impl ConfigValues {
    pub fn merge(&self, right: &ConfigValues) -> anyhow::Result<ConfigValues> {
        let mut merged: ConfigValues = serde_merge::omerge(&self, &right)?;
        if let Some(agave) = &self.agave {
            if let Some(right_agave) = &right.agave {
                merged.agave = Some(agave.merge(right_agave)?);
            }
        }
        if let Some(base_urls) = &self.base_urls {
            if let Some(right_base_urls) = &right.base_urls {
                merged.base_urls = Some(base_urls.merge(right_base_urls)?);
            }
        }
        if let Some(dashboard_aggregator) = &self.dashboard_aggregator {
            if let Some(right_dashboard_aggregator) = &right.dashboard_aggregator {
                merged.dashboard_aggregator =
                    Some(dashboard_aggregator.merge(right_dashboard_aggregator)?);
            }
        }
        merged.de = self.de.merge(&right.de)?;
        if let Some(docker) = &self.docker {
            if let Some(right_docker) = &right.docker {
                merged.docker = Some(docker.merge(right_docker)?);
            }
        }
        merged.elasticsearch = self.elasticsearch.merge(&right.elasticsearch)?;
        merged.email = self.email.merge(&right.email)?;
        merged.grouper = self.grouper.merge(&right.grouper)?;
        merged.icat = self.icat.merge(&right.icat)?;
        if let Some(infosquito) = &self.infosquito {
            if let Some(right_infosquito) = &right.infosquito {
                merged.infosquito = Some(infosquito.merge(right_infosquito)?);
            }
        }
        if let Some(intercom) = &self.intercom {
            if let Some(right_intercom) = &right.intercom {
                merged.intercom = Some(intercom.merge(right_intercom)?);
            }
        }
        merged.irods = self.irods.merge(&right.irods)?;
        if let Some(jobs) = &self.jobs {
            if let Some(right_jobs) = &right.jobs {
                merged.jobs = Some(jobs.merge(right_jobs)?);
            }
        }
        merged.keycloak = self.keycloak.merge(&right.keycloak)?;
        merged.pgp = self.pgp.merge(&right.pgp)?;
        if let Some(permanent_id) = &self.permanent_id {
            if let Some(right_permanent_id) = &right.permanent_id {
                merged.permanent_id = Some(permanent_id.merge(right_permanent_id)?);
            }
        }
        if let Some(unleash) = &self.unleash {
            if let Some(right_unleash) = &right.unleash {
                merged.unleash = Some(unleash.merge(right_unleash)?);
            }
        }
        merged.user_portal = self.user_portal.merge(&right.user_portal)?;
        merged.vice = self.vice.merge(&right.vice)?;
        merged.de_db = self.de_db.merge(&right.de_db)?;
        merged.grouper_db = self.grouper_db.merge(&right.grouper_db)?;
        merged.new_notifications_db = self
            .new_notifications_db
            .merge(&right.new_notifications_db)?;
        merged.notifications_db = self.notifications_db.merge(&right.notifications_db)?;
        merged.permissions_db = self.permissions_db.merge(&right.permissions_db)?;
        merged.qms_db = self.qms_db.merge(&right.qms_db)?;
        merged.metadata_db = self.metadata_db.merge(&right.metadata_db)?;
        merged.unleash_db = self.unleash_db.merge(&right.unleash_db)?;
        if let Some(admin) = &self.admin {
            if let Some(right_admin) = &right.admin {
                merged.admin = Some(admin.merge(right_admin)?);
            }
        }
        if let Some(analytics) = &self.analytics {
            if let Some(right_analytics) = &right.analytics {
                merged.analytics = Some(analytics.merge(right_analytics)?);
            }
        }
        if let Some(harbor) = &self.harbor {
            if let Some(right_harbor) = &right.harbor {
                merged.harbor = Some(harbor.merge(right_harbor)?);
            }
        }
        if let Some(qms) = &self.qms {
            if let Some(right_qms) = &right.qms {
                merged.qms = Some(qms.merge(right_qms)?);
            }
        }
        if let Some(jaeger) = &self.jaeger {
            if let Some(right_jaeger) = &right.jaeger {
                merged.jaeger = Some(jaeger.merge(right_jaeger)?);
            }
        }
        Ok(merged)
    }

    pub fn ask_for_info(&mut self) -> anyhow::Result<()> {
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

        self.environment = environment;
        self.namespace = namespace;
        self.uid_domain = uid_domain;
        self.timezone = Some(timezone);

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
