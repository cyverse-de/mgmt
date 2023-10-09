use crate::config_values::{
    self, agave::Agave, base_urls::BaseURLs, dashboard_aggregator::DashboardAggregator,
    db::DatabaseConfig, db::QMSDatabaseConfig, de::DE, docker::Docker,
    elasticsearch::Elasticsearch, email::Email, grouper::Grouper, icat::Icat,
    infosquito::Infosquito,
};
use crate::db::{self, add_env_cfg_value, set_config_value, upsert_environment, LoadFromDatabase};
use anyhow::Context;
use dialoguer::{console::Style, theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool, Transaction};

#[derive(Debug, Default, Clone, Copy)]
pub struct SectionOptions {
    include_admin: bool,
    include_analytics: bool,
    include_agave: bool,
    include_base_urls: bool,
    include_cas: bool,
    include_docker: bool,
    include_infosquito: bool,
    include_intercom: bool,
    include_jaeger: bool,
    include_jobs: bool,
    include_jvmpopts: bool,
    include_permanent_id: bool,
    include_qa: bool,
    include_qms: bool,
    include_unleash: bool,
}

impl From<db::FeatureFlags> for SectionOptions {
    fn from(ff: db::FeatureFlags) -> Self {
        Self {
            include_admin: ff.administration.unwrap_or_default(),
            include_analytics: ff.analytics.unwrap_or_default(),
            include_agave: ff.agave.unwrap_or_default(),
            include_base_urls: ff.base_urls.unwrap_or_default(),
            include_cas: ff.cas.unwrap_or_default(),
            include_docker: ff.docker.unwrap_or_default(),
            include_infosquito: ff.infosquito.unwrap_or_default(),
            include_intercom: ff.intercom.unwrap_or_default(),
            include_jaeger: ff.jaeger.unwrap_or_default(),
            include_jobs: ff.jobs.unwrap_or_default(),
            include_jvmpopts: ff.jvmopts.unwrap_or_default(),
            include_permanent_id: ff.permanent_id.unwrap_or_default(),
            include_qa: ff.qa.unwrap_or_default(),
            include_qms: ff.qms.unwrap_or_default(),
            include_unleash: ff.unleash.unwrap_or_default(),
        }
    }
}

impl From<SectionOptions> for db::FeatureFlags {
    fn from(so: SectionOptions) -> Self {
        Self {
            administration: Some(so.include_admin),
            analytics: Some(so.include_analytics),
            agave: Some(so.include_agave),
            base_urls: Some(so.include_base_urls),
            cas: Some(so.include_cas),
            docker: Some(so.include_docker),
            infosquito: Some(so.include_infosquito),
            intercom: Some(so.include_intercom),
            jaeger: Some(so.include_jaeger),
            jobs: Some(so.include_jobs),
            jvmopts: Some(so.include_jvmpopts),
            permanent_id: Some(so.include_permanent_id),
            qa: Some(so.include_qa),
            qms: Some(so.include_qms),
            unleash: Some(so.include_unleash),
        }
    }
}

impl SectionOptions {
    pub async fn new_from_db(pool: &Pool<MySql>, env: &str) -> anyhow::Result<SectionOptions> {
        let mut tx = pool.begin().await?;
        let ff = db::get_feature_flags(&mut tx, env).await?;
        tx.commit().await?;

        Ok(ff.into())
    }

    pub fn new(sub_m: &clap::ArgMatches) -> Self {
        let include_all = if sub_m.contains_id("include-all") {
            sub_m.get_flag("include-all")
        } else {
            false
        };

        if include_all {
            Self {
                include_admin: true,
                include_analytics: true,
                include_agave: true,
                include_base_urls: true,
                include_cas: true,
                include_docker: true,
                include_infosquito: true,
                include_intercom: true,
                include_jaeger: true,
                include_jobs: true,
                include_jvmpopts: true,
                include_permanent_id: true,
                include_qa: true,
                include_qms: true,
                include_unleash: true,
            }
        } else {
            Self {
                include_admin: if sub_m.contains_id("include-admin") {
                    sub_m.get_flag("include-admin")
                } else {
                    false
                },

                include_analytics: if sub_m.contains_id("include-analytics") {
                    sub_m.get_flag("include-analytics")
                } else {
                    false
                },

                include_agave: if sub_m.contains_id("include-agave") {
                    sub_m.get_flag("include-agave")
                } else {
                    false
                },

                include_base_urls: if sub_m.contains_id("include-base-urls") {
                    sub_m.get_flag("include-base-urls")
                } else {
                    false
                },

                include_cas: if sub_m.contains_id("include-cas") {
                    sub_m.get_flag("include-cas")
                } else {
                    false
                },

                include_docker: if sub_m.contains_id("include-docker") {
                    sub_m.get_flag("include-docker")
                } else {
                    false
                },

                include_infosquito: if sub_m.contains_id("include-infosquito") {
                    sub_m.get_flag("include-infosquito")
                } else {
                    false
                },

                include_intercom: if sub_m.contains_id("include-intercom") {
                    sub_m.get_flag("include-intercom")
                } else {
                    false
                },

                include_jaeger: if sub_m.contains_id("include-jaeger") {
                    sub_m.get_flag("include-jaeger")
                } else {
                    false
                },

                include_jobs: if sub_m.contains_id("include-jobs") {
                    sub_m.get_flag("include-jobs")
                } else {
                    false
                },

                include_jvmpopts: if sub_m.contains_id("include-jvmpopts") {
                    sub_m.get_flag("include-jvmpopts")
                } else {
                    false
                },

                include_permanent_id: if sub_m.contains_id("include-permanent-id") {
                    sub_m.get_flag("include-permanent-id")
                } else {
                    false
                },

                include_qa: if sub_m.contains_id("include-qa") {
                    sub_m.get_flag("include-qa")
                } else {
                    false
                },

                include_qms: if sub_m.contains_id("include-qms") {
                    sub_m.get_flag("include-qms")
                } else {
                    false
                },

                include_unleash: if sub_m.contains_id("include-unleash") {
                    sub_m.get_flag("include-unleash")
                } else {
                    false
                },
            }
        }
    }

    pub fn set_all(&mut self, all: bool) -> anyhow::Result<()> {
        self.include_admin = all;
        self.include_analytics = all;
        self.include_agave = all;
        self.include_base_urls = all;
        self.include_cas = all;
        self.include_docker = all;
        self.include_infosquito = all;
        self.include_intercom = all;
        self.include_jaeger = all;
        self.include_jobs = all;
        self.include_jvmpopts = all;
        self.include_permanent_id = all;
        self.include_qa = all;
        self.include_qms = all;
        self.include_unleash = all;

        Ok(())
    }

    pub fn include_section(&self, section: &str) -> bool {
        match section {
            "Admin" => self.include_admin,
            "Analytics" => self.include_analytics,
            "Agave" => self.include_agave,
            "BaseURLs" => self.include_base_urls,
            "CAS" => self.include_cas,
            "Docker" => self.include_docker,
            "InfoSquito" => self.include_infosquito,
            "Intercom" => self.include_intercom,
            "Jaeger" => self.include_jaeger,
            "Jobs" => self.include_jobs,
            "JVMOpts" => self.include_jvmpopts,
            "PermanentID" => self.include_permanent_id,
            "QA" => self.include_qa,
            "QMS" => self.include_qms,
            "Unleash" => self.include_unleash,
            _ => true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigValues {
    #[serde(skip)]
    section: String,

    #[serde(skip)]
    section_options: SectionOptions,

    // Must be user supplied.
    pub environment: String,

    // Must be user supplied.
    namespace: String,

    // Must be user supplied.
    #[serde(rename = "UIDDomain")]
    uid_domain: String,

    // Optional for deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    agave: Option<Agave>,

    // Defaults are provided for deployment.
    #[serde(rename = "BaseURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    base_urls: Option<BaseURLs>,

    // Defaults are provided for deployment (or will be).
    dashboard_aggregator: Option<DashboardAggregator>,

    // Contains settings that must be provided for deployment.
    #[serde(rename = "DE")]
    de: DE,

    // Defaults are provided for deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    infosquito: Option<Infosquito>,

    // Optional for deployment
    #[serde(skip_serializing_if = "Option::is_none")]
    intercom: Option<config_values::intercom::Intercom>,

    // Must be configured for deployment.
    #[serde(rename = "IRODS")]
    irods: config_values::irods::Irods,

    // Defaults are provided for deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    jobs: Option<config_values::misc::Jobs>,

    // Must be configured for deployment.
    keycloak: config_values::keycloak::Keycloak,

    // Must be configured for deployment.
    #[serde(rename = "PGP")]
    pgp: config_values::misc::Pgp,

    // Optional for deployment.
    #[serde(rename = "PermanentID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    permanent_id: Option<config_values::misc::PermanentId>,

    // Defaults provided for deployment.
    timezone: Option<String>,

    // Optional for deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    unleash_db: Option<DatabaseConfig>,

    // Defaults provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    admin: Option<config_values::misc::Admin>,

    // Optional for deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics: Option<config_values::misc::Analytics>,

    // Defaults provided for deployment.
    harbor: Option<config_values::misc::Harbor>,

    // Optional for deployment.
    #[serde(rename = "QMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    qms: Option<config_values::misc::Qms>,

    // Optional for deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    jaeger: Option<config_values::misc::Jaeger>,

    // Optional for deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "QA")]
    qa: Option<config_values::qa::QA>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "CAS")]
    cas: Option<config_values::cas::CAS>,
}

impl Default for ConfigValues {
    fn default() -> Self {
        ConfigValues {
            section: "TopLevel".to_string(),
            section_options: SectionOptions::default(),
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
            notifications_db: DatabaseConfig::default(),
            permissions_db: DatabaseConfig::default(),
            qms_db: QMSDatabaseConfig::default(),
            metadata_db: DatabaseConfig::default(),
            unleash_db: Some(DatabaseConfig::default()),
            admin: Some(config_values::misc::Admin::default()),
            analytics: Some(config_values::misc::Analytics::default()),
            harbor: Some(config_values::misc::Harbor::default()),
            qms: Some(config_values::misc::Qms::default()),
            jaeger: Some(config_values::misc::Jaeger::default()),
            qa: Some(config_values::qa::QA::default()),
            cas: Some(config_values::cas::CAS::default()),
        }
    }
}

impl LoadFromDatabase for ConfigValues {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Environment" => self.environment = value,
                "Namespace" => self.namespace = value,
                "UIDDomain" => self.uid_domain = value,
                "Timezone" => self.timezone = Some(value),
                _ => (),
            }
        }
        Ok(())
    }

    fn cfg_set_keys(&mut self, cfgs: Vec<crate::db::ConfigurationValue>) -> anyhow::Result<()> {
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
                        if let Some(unleash_db) = &mut self.unleash_db {
                            unleash_db.cfg_set_key(cfg).ok();
                        }
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

                    "QA" => {
                        if let Some(qa) = &mut self.qa {
                            qa.cfg_set_key(cfg).ok();
                        }
                    }

                    "CAS" => {
                        if let Some(cas) = &mut self.cas {
                            cas.cfg_set_key(cfg).ok();
                        }
                    }

                    _ => (),
                }
            }
        });
        Ok(())
    }
}

impl From<Vec<db::ConfigurationValue>> for ConfigValues {
    fn from(cfgs: Vec<db::ConfigurationValue>) -> Self {
        let mut cv = ConfigValues::default();
        cv.cfg_set_keys(cfgs).ok();
        cv
    }
}

impl From<ConfigValues> for Vec<db::ConfigurationValue> {
    fn from(cv: ConfigValues) -> Vec<db::ConfigurationValue> {
        let mut cfgs: Vec<db::ConfigurationValue> = Vec::new();

        let section: String;
        if cv.section.is_empty() {
            section = "TopLevel".to_string();
        } else {
            section = cv.section.clone();
        }

        cfgs.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Environment".to_string()),
            value: Some(cv.environment),
            value_type: Some("string".to_string()),
        });

        cfgs.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("Namespace".to_string()),
            value: Some(cv.namespace),
            value_type: Some("string".to_string()),
        });

        cfgs.push(db::ConfigurationValue {
            id: None,
            section: Some(section.clone()),
            key: Some("UIDDomain".to_string()),
            value: Some(cv.uid_domain),
            value_type: Some("string".to_string()),
        });

        // Agave section is optional, so check before adding it.
        if cv.section_options.include_section("Agave") {
            if let Some(agave) = cv.agave {
                cfgs.extend::<Vec<db::ConfigurationValue>>(agave.into());
            }
        }

        // BaseURLs is optional, so check before adding it.
        if cv.section_options.include_section("BaseURLs") {
            if let Some(base_urls) = cv.base_urls {
                cfgs.extend::<Vec<db::ConfigurationValue>>(base_urls.into());
            }
        }

        if let Some(dashboard_aggregator) = cv.dashboard_aggregator {
            cfgs.extend::<Vec<db::ConfigurationValue>>(dashboard_aggregator.into());
        }

        cfgs.extend::<Vec<db::ConfigurationValue>>(cv.de.into());

        // Docker is optional, so check before adding it.
        if cv.section_options.include_section("Docker") {
            if let Some(docker) = cv.docker {
                cfgs.extend::<Vec<db::ConfigurationValue>>(docker.into());
            }
        }

        cfgs.extend::<Vec<db::ConfigurationValue>>(cv.elasticsearch.into());

        cfgs.extend::<Vec<db::ConfigurationValue>>(cv.email.into());

        cfgs.extend::<Vec<db::ConfigurationValue>>(cv.grouper.into());

        cfgs.extend::<Vec<db::ConfigurationValue>>(cv.icat.into());

        // Infosquito is optional, so check before adding it.
        if cv.section_options.include_section("Infosquito") {
            if let Some(infosquito) = cv.infosquito {
                cfgs.extend::<Vec<db::ConfigurationValue>>(infosquito.into());
            }
        }

        // Intercom is optional, so check before adding it.
        if cv.section_options.include_section("Intercom") {
            if let Some(intercom) = cv.intercom {
                cfgs.extend::<Vec<db::ConfigurationValue>>(intercom.into());
            }
        }

        cfgs.extend::<Vec<db::ConfigurationValue>>(cv.irods.into());

        // Jobs is optional, so check before adding it.
        if cv.section_options.include_section("Jobs") {
            if let Some(jobs) = cv.jobs {
                cfgs.extend::<Vec<db::ConfigurationValue>>(jobs.into());
            }
        }

        cfgs.extend::<Vec<db::ConfigurationValue>>(cv.keycloak.into());

        cfgs.extend::<Vec<db::ConfigurationValue>>(cv.pgp.into());

        // PermanentID is optional, so check before adding it.
        if cv.section_options.include_section("PermanentID") {
            if let Some(permanent_id) = cv.permanent_id {
                cfgs.extend::<Vec<db::ConfigurationValue>>(permanent_id.into());
            }
        }

        if let Some(timezone) = cv.timezone {
            cfgs.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Timezone".to_string()),
                value: Some(timezone),
                value_type: Some("string".to_string()),
            });
        }

        // Unleash is optional, so check before adding it.
        if cv.section_options.include_section("Unleash") {
            if let Some(unleash) = cv.unleash {
                cfgs.extend::<Vec<db::ConfigurationValue>>(unleash.into());
            }
        }

        cfgs.extend::<Vec<db::ConfigurationValue>>(cv.user_portal.into());

        cfgs.extend::<Vec<db::ConfigurationValue>>(cv.vice.into());

        let mut de_db_cfgs: Vec<db::ConfigurationValue> = cv.de_db.into();
        de_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("DEDB".to_string());
        });
        cfgs.extend::<Vec<db::ConfigurationValue>>(de_db_cfgs.into());

        let mut grouper_db_cfgs: Vec<db::ConfigurationValue> = cv.grouper_db.into();
        grouper_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("GrouperDB".to_string());
        });
        cfgs.extend::<Vec<db::ConfigurationValue>>(grouper_db_cfgs.into());

        let mut notifications_db_cfgs: Vec<db::ConfigurationValue> = cv.notifications_db.into();
        notifications_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("NotificationsDB".to_string());
        });
        cfgs.extend::<Vec<db::ConfigurationValue>>(notifications_db_cfgs.into());

        let mut permissions_db_cfgs: Vec<db::ConfigurationValue> = cv.permissions_db.into();
        permissions_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("PermissionsDB".to_string());
        });
        cfgs.extend::<Vec<db::ConfigurationValue>>(permissions_db_cfgs.into());

        let mut qms_db_cfgs: Vec<db::ConfigurationValue> = cv.qms_db.into();
        qms_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("QMSDB".to_string());
        });
        cfgs.extend::<Vec<db::ConfigurationValue>>(qms_db_cfgs.into());

        let mut metadata_db_cfgs: Vec<db::ConfigurationValue> = cv.metadata_db.into();
        metadata_db_cfgs.iter_mut().for_each(|cfg| {
            cfg.section = Some("MetadataDB".to_string());
        });
        cfgs.extend::<Vec<db::ConfigurationValue>>(metadata_db_cfgs.into());

        // UnleashDB is optional, so check before adding it.
        if cv.section_options.include_section("Unleash") {
            if let Some(unleash_db) = cv.unleash_db {
                let mut unleash_db_cfgs: Vec<db::ConfigurationValue> = unleash_db.into();
                unleash_db_cfgs.iter_mut().for_each(|cfg| {
                    cfg.section = Some("UnleashDB".to_string());
                });
                cfgs.extend::<Vec<db::ConfigurationValue>>(unleash_db_cfgs.into());
            }
        }

        // Admin is optional, so check before adding it.
        if cv.section_options.include_section("Admin") {
            if let Some(admin) = cv.admin {
                cfgs.extend::<Vec<db::ConfigurationValue>>(admin.into());
            }
        }

        // Analytics is optional, so check before adding it.
        if cv.section_options.include_section("Analytics") {
            if let Some(analytics) = cv.analytics {
                cfgs.extend::<Vec<db::ConfigurationValue>>(analytics.into());
            }
        }

        if let Some(harbor) = cv.harbor {
            cfgs.extend::<Vec<db::ConfigurationValue>>(harbor.into());
        }

        // QMS is optional, so check before adding it.
        if cv.section_options.include_section("QMS") {
            if let Some(qms) = cv.qms {
                cfgs.extend::<Vec<db::ConfigurationValue>>(qms.into());
            }
        }

        // Jaeger is optional, so check before adding it.
        if cv.section_options.include_section("Jaeger") {
            if let Some(jaeger) = cv.jaeger {
                cfgs.extend::<Vec<db::ConfigurationValue>>(jaeger.into());
            }
        }

        // QA is optional, so check before adding it.
        if cv.section_options.include_section("QA") {
            if let Some(qa) = cv.qa {
                cfgs.extend::<Vec<db::ConfigurationValue>>(qa.into());
            }
        }

        if let Some(cas) = cv.cas {
            cfgs.extend::<Vec<db::ConfigurationValue>>(cas.into());
        }

        cfgs
    }
}

impl ConfigValues {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_section_options(&mut self, section_options: SectionOptions) {
        self.section_options = section_options;
    }

    pub fn reset_sections(&mut self) -> anyhow::Result<()> {
        if !self.section_options.include_section("Agave") {
            self.agave = None;
        }

        if !self.section_options.include_section("BaseURLs") {
            self.base_urls = None;
        }

        if !self.section_options.include_section("Docker") {
            self.docker = None;
        }

        if !self.section_options.include_section("Infosquito") {
            self.infosquito = None;
        }

        if !self.section_options.include_section("Intercom") {
            self.intercom = None;
        }

        if !self.section_options.include_section("Jobs") {
            self.jobs = None;
        }

        if !self.section_options.include_section("PermanentID") {
            self.permanent_id = None;
        }

        if !self.section_options.include_section("Unleash") {
            self.unleash = None;
        }

        if !self.section_options.include_section("Admin") {
            self.admin = None;
        }

        if !self.section_options.include_section("Analytics") {
            self.analytics = None;
        }

        if !self.section_options.include_section("QMS") {
            self.qms = None;
        }

        if !self.section_options.include_section("Jaeger") {
            self.jaeger = None;
        }

        if !self.section_options.include_section("QA") {
            self.qa = None;
        }

        Ok(())
    }

    pub async fn ask_for_info(&mut self, tx: &mut Transaction<'_, MySql>) -> anyhow::Result<()> {
        let mut theme = ColorfulTheme::default();
        theme.hint_style = Style::new().yellow();
        let mut section_options = SectionOptions::default();

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
        self.environment = environment.clone();
        self.namespace = namespace.clone();
        self.uid_domain = uid_domain.clone();
        self.timezone = Some(timezone.clone());

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

        // We don't prompt for this yet.
        section_options.include_base_urls = true;

        // We also don't prompt for this yet.
        section_options.include_cas = false;

        // Fill in the DE and iRODS settings first, since they have some
        // values that can be used as defaults later.
        self.de.ask_for_info(tx, &theme, env_id).await?;
        self.irods.ask_for_info(tx, &theme, env_id).await?;

        // We need the base URI and external host for other settings.
        let base_uri = self
            .de
            .base_uri
            .clone()
            .context("Base URI not set in DE settings.")?;
        let irods_external = self.irods.external_host.clone().context(
            "External host not set in iRODS settings.  This is required for DE settings.",
        )?;

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
            section_options.include_agave = true;
        }

        let mut new_da = DashboardAggregator::default();
        new_da.ask_for_info(tx, &theme, env_id).await?;
        self.dashboard_aggregator = Some(new_da);

        let docker_enabled = Select::with_theme(&theme)
            .with_prompt("Include Docker?")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        if docker_enabled == 0 {
            let mut new_docker = Docker::default();
            new_docker.ask_for_info(tx, &theme, env_id).await?;
            self.docker = Some(new_docker);
            section_options.include_docker = true;
        }

        self.elasticsearch.ask_for_info(tx, &theme, env_id).await?;
        self.email.ask_for_info(tx, &theme, env_id).await?;
        self.grouper
            .ask_for_info(tx, &theme, env_id, &self.environment)
            .await?;
        self.icat.ask_for_info(tx, &theme, env_id).await?;

        let infosquito_enabled = Select::with_theme(&theme)
            .with_prompt("Include Infosquito?")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        if infosquito_enabled == 0 {
            let mut new_infosquito = Infosquito::default();
            new_infosquito.ask_for_info(tx, &theme, env_id).await?;
            self.infosquito = Some(new_infosquito);
            section_options.include_infosquito = true;
        }

        let intercom_enabled = Select::with_theme(&theme)
            .with_prompt("Include Intercom?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if intercom_enabled == 0 {
            let mut new_intercom = config_values::intercom::Intercom::default();
            new_intercom.ask_for_info(tx, &theme, env_id).await?;
            self.intercom = Some(new_intercom);
            section_options.include_intercom = true;
        }

        let jobs_enabled = Select::with_theme(&theme)
            .with_prompt("Include Jobs?")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        if jobs_enabled == 0 {
            let mut new_jobs = config_values::misc::Jobs::default();
            new_jobs.ask_for_info(tx, &theme, env_id).await?;
            self.jobs = Some(new_jobs);
            section_options.include_jobs = true;
        }

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
            section_options.include_permanent_id = true;
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
            let mut new_unleash_db = DatabaseConfig::default();
            new_unleash_db
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
            self.unleash_db = Some(new_unleash_db);
            section_options.include_unleash = true;
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
            section_options.include_qms = true;
        }

        self.user_portal.ask_for_info(tx, &theme, env_id).await?;
        self.vice.ask_for_info(tx, &theme, env_id).await?;

        let admin_enabled = Select::with_theme(&theme)
            .with_prompt("Include Admin?")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;
        if admin_enabled == 0 {
            let mut new_admin = config_values::misc::Admin::default();
            new_admin.ask_for_info(tx, &theme, env_id).await?;
            self.admin = Some(new_admin);
            section_options.include_admin = true;
        }

        let analytics_enabled = Select::with_theme(&theme)
            .with_prompt("Include Analytics?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if analytics_enabled == 0 {
            let mut new_analytics = config_values::misc::Analytics::default();
            new_analytics.ask_for_info(tx, &theme, env_id).await?;
            self.analytics = Some(new_analytics);
            section_options.include_analytics = true;
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
            section_options.include_jaeger = true;
        }

        let qa_enabled = Select::with_theme(&theme)
            .with_prompt("Include QA?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if qa_enabled == 0 {
            let mut new_qa = config_values::qa::QA::default();
            new_qa.ask_for_info(tx, &theme, env_id).await?;
            self.qa = Some(new_qa);
            section_options.include_qa = true;
        }

        let cas_enabled = Select::with_theme(&theme)
            .with_prompt("Include CAS?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if cas_enabled == 0 {
            let mut new_cas = config_values::cas::CAS::default();
            new_cas.ask_for_info(tx, &theme, env_id).await?;
            self.cas = Some(new_cas);
            //section_options.include_cas = true;
        }

        self.section_options = section_options;
        db::upsert_feature_flags(tx, &self.environment, &section_options.into()).await?;

        Ok(())
    }
}
