use anyhow::anyhow;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Agave {
    #[garde(ascii, length(min = 3))]
    key: String,

    #[garde(ascii, length(min = 3))]
    secret: String,

    #[serde(rename = "RedirectURI")]
    #[garde(url)]
    redirect_uri: String,

    #[garde(length(min = 0))]
    storage_system: String,

    #[serde(rename = "CallbackBaseURI")]
    #[garde(url)]
    callback_base_uri: String,

    #[garde(range(min=0, max=u32::MAX))]
    read_timeout: u32,

    #[garde(skip)]
    enabled: bool,

    #[garde(skip)]
    jobs_enabled: bool,
}

impl Default for Agave {
    fn default() -> Self {
        Agave {
            key: String::new(),
            secret: String::new(),
            redirect_uri: String::new(),
            storage_system: String::new(),
            callback_base_uri: String::new(),
            read_timeout: 30000,
            enabled: false,
            jobs_enabled: false,
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "AMQP")]
pub struct Amqp {
    #[garde(ascii, length(min = 3))]
    user: String,

    #[garde(ascii, length(min = 3))]
    password: String,

    #[garde(length(min = 3))]
    host: String,

    #[garde(range(min = 1024, max = 65535))]
    port: u16,

    #[garde(length(min = 3))]
    vhost: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BaseURLs {
    #[garde(url)]
    analyses: String,

    #[garde(url)]
    app_exposer: String,

    #[garde(url)]
    apps: String,

    #[garde(url)]
    async_tasks: String,

    #[garde(url)]
    dashboard_aggregator: String,

    #[garde(url)]
    data_info: String,

    #[garde(url)]
    grouper_web_services: String,

    #[garde(url)]
    iplant_email: String,

    #[garde(url)]
    iplant_groups: String,

    #[garde(url)]
    jex_adapter: String,

    #[garde(url)]
    job_status_listener: String,

    #[garde(url)]
    metadata: String,

    #[garde(url)]
    notifications: String,

    #[garde(url)]
    permissions: String,

    #[garde(url)]
    #[serde(rename = "QMS")]
    qms: String,

    #[garde(url)]
    requests: String,

    #[garde(url)]
    search: String,

    #[garde(url)]
    terrain: String,

    #[garde(url)]
    user_info: String,
}

impl Default for BaseURLs {
    fn default() -> Self {
        BaseURLs {
            analyses: String::from("http://analyses"),
            app_exposer: String::from("http://app-exposer"),
            apps: String::from("http://apps"),
            async_tasks: String::from("http://async-tasks"),
            dashboard_aggregator: String::from("http://dashboard-aggregator"),
            data_info: String::from("http://data-info"),
            grouper_web_services: String::from("http://grouper-ws/grouper-ws"),
            iplant_email: String::from("http://de-mailer"),
            iplant_groups: String::from("http://iplant-groups"),
            jex_adapter: String::from("http://jex-adapter"),
            job_status_listener: String::from("http://job-status-listener"),
            metadata: String::from("http://metadata"),
            notifications: String::from("http://notifications"),
            permissions: String::from("http://permissions"),
            qms: String::from("http://qms"),
            requests: String::from("http://requests"),
            search: String::from("http://search"),
            terrain: String::from("http://terrain"),
            user_info: String::from("http://user_info"),
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct Website {
    #[serde(rename = "URL")]
    #[garde(url)]
    url: String,
}

impl Default for Website {
    fn default() -> Self {
        Website {
            url: String::from("https://cyverse.org"),
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DashboardAggregator {
    #[garde(length(min = 3))]
    public_group: String,

    #[garde(dive)]
    website: Website,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DESubscriptions {
    #[serde(rename = "CheckoutURL")]
    #[garde(url)]
    checkout_url: String,

    #[garde(skip)]
    enforce: bool,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DECoge {
    #[serde(rename = "BaseURI")]
    #[garde(url)]
    base_uri: String,
}

impl Default for DECoge {
    fn default() -> Self {
        DECoge {
            base_uri: String::from("https://genomevolution.org/coge/api/v1"),
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DETools {
    #[garde(dive)]
    admin: DEToolsAdmin,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DEToolsAdmin {
    #[garde(range(min=0, max=u32::MAX))]
    max_cpu_limit: u32,

    #[garde(range(min=0, max=u64::MAX))]
    max_memory_limit: u64,

    #[garde(range(min=0, max=u64::MAX))]
    max_disk_limit: u64,
}

impl Default for DEToolsAdmin {
    fn default() -> Self {
        DEToolsAdmin {
            max_cpu_limit: 24,
            max_memory_limit: 75161927680,
            max_disk_limit: 1099511627776,
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DE {
    #[serde(rename = "AMQP")]
    #[garde(dive)]
    amqp: Amqp,

    #[serde(rename = "BaseURI")]
    #[garde(url)]
    base_uri: String,

    #[garde(skip)]
    subscriptions: Option<DESubscriptions>,

    #[garde(length(min = 3))]
    default_output_folder: String,

    #[garde(dive)]
    coge: DECoge,

    #[garde(dive)]
    tools: DETools,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Docker {
    #[garde(skip)]
    trusted_registries: Vec<String>,

    #[garde(length(min = 1))]
    tag: String,
}

impl Default for Docker {
    fn default() -> Self {
        Docker {
            tag: String::from("latest"),
            trusted_registries: vec![
                String::from("harbor.cyverse.org"),
                String::from("docker.cyverse.org"),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ElasticSearch {
    #[serde(rename = "BaseURI")]
    #[garde(url)]
    base_uri: String,

    #[garde(ascii, length(min = 3))]
    username: String,

    #[garde(ascii, length(min = 8))]
    password: String,

    #[garde(range(min=0, max=u32::MAX))]
    index: u32,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Email {
    #[garde(email)]
    src: String,

    #[garde(email)]
    dest: String,

    #[serde(rename = "PermIDRequestDest")]
    #[garde(email)]
    perm_id_request_dest: String,

    #[garde(email)]
    support_dest: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GrouperLoader {
    #[serde(rename = "URI")]
    #[garde(url)]
    uri: String,

    #[garde(ascii, length(min = 3))]
    user: String,

    #[garde(ascii, length(min = 8))]
    password: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Grouper {
    #[garde(alphanumeric, length(min = 1))]
    morph_string: String,

    #[garde(ascii, length(min = 3))]
    password: String,

    #[garde(alphanumeric)]
    folder_name_prefix: String,

    #[garde(dive)]
    loader: GrouperLoader,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "ICAT")]
pub struct Icat {
    #[garde(length(min = 3))]
    host: String,

    #[garde(range(min = 1024, max = 65535))]
    port: u16,

    #[garde(ascii, length(min = 3))]
    user: String,

    #[garde(ascii, length(min = 3))]
    password: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Infosquito {
    #[garde(range(min = 1, max = 7))]
    day_num: u8,

    #[garde(range(min=1, max = u32::MAX))]
    prefix_length: u32,
}

impl Default for Infosquito {
    fn default() -> Self {
        Infosquito {
            day_num: 4,
            prefix_length: 4,
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Intercom {
    #[garde(skip)]
    enabled: bool,

    #[serde(rename = "AppID")]
    #[garde(length(min = 1))]
    app_id: String,

    #[serde(rename = "CompanyID")]
    #[garde(length(min = 1))]
    company_id: String,

    #[garde(length(min = 1))]
    company_name: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IrodsWebDav {
    #[serde(rename = "AnonURI")]
    #[garde(url)]
    anon_uri: String,
}

impl Default for IrodsWebDav {
    fn default() -> Self {
        IrodsWebDav {
            anon_uri: String::from("https://data.cyverse.rocksi/dav-anon"),
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Irods {
    #[serde(rename = "AMQP")]
    #[garde(dive)]
    amqp: Amqp,

    #[garde(length(min = 3))]
    host: String,

    #[garde(ascii, length(min = 3))]
    user: String,

    #[garde(length(min = 1))]
    zone: String,

    #[garde(ascii, length(min = 8))]
    password: String,

    #[garde(skip)]
    admin_users: Vec<String>,

    #[garde(skip)]
    perms_filter: Vec<String>,

    #[garde(length(min = 3))]
    external_host: String,

    #[garde(dive)]
    web_dav: IrodsWebDav,

    #[garde(length(min = 1))]
    quota_root_resources: String,
}

impl Default for Irods {
    fn default() -> Self {
        Irods {
            amqp: Amqp::default(),
            host: String::new(),
            user: String::new(),
            zone: String::new(),
            password: String::new(),
            admin_users: Vec::new(),
            perms_filter: Vec::new(),
            web_dav: IrodsWebDav::default(),
            external_host: String::from("data.cyverse.rocks"),
            quota_root_resources: String::from("mainIngestRes,mainReplRes"),
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Jobs {
    #[garde(length(min = 1))]
    data_transfer_image: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct KeycloakVice {
    #[serde(rename = "ClientID")]
    #[garde(length(min = 1))]
    client_id: String,

    #[garde(length(min = 8))]
    client_secret: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Keycloak {
    #[garde(url)]
    server_uri: String,

    #[garde(length(min = 1))]
    realm: String,

    #[serde(rename = "ClientID")]
    #[garde(length(min = 1))]
    client_id: String,

    #[garde(length(min = 1))]
    client_secret: String,

    #[serde(rename = "VICE")]
    #[garde(dive)]
    vice: KeycloakVice,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "PGP")]
pub struct Pgp {
    #[garde(ascii, length(min = 8))]
    key_password: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PermanentIdDataCite {
    #[serde(rename = "BaseURI")]
    #[garde(url)]
    base_uri: String,

    #[garde(ascii, length(min = 3))]
    user: String,

    #[garde(ascii, length(min = 8))]
    password: String,

    #[serde(rename = "DOIPrefix")]
    #[garde(length(min = 1))]
    doi_prefix: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PermanentId {
    #[garde(length(min = 1))]
    curators_group: String,

    #[garde(dive)]
    data_cite: PermanentIdDataCite,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Unleash {
    #[serde(rename = "BaseURL")]
    #[garde(url)]
    base_url: String,

    #[serde(rename = "APIPath")]
    #[garde(length(min = 1))]
    api_path: String,

    #[serde(rename = "APIToken")]
    #[garde(length(min = 1))]
    api_token: String,

    #[garde(length(min = 1))]
    maintenance_flag: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UserPortal {
    #[serde(rename = "BaseURI")]
    #[garde(url)]
    base_uri: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ViceFileTransfers {
    #[garde(length(min = 1))]
    image: String,

    #[garde(length(min = 1))]
    tag: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ViceDefaultBackend {
    #[garde(length(min = 1))]
    loading_page_template_string: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Vice {
    #[serde(rename = "BaseURI")]
    #[garde(url)]
    base_uri: String,

    #[garde(dive)]
    file_transfers: ViceFileTransfers,

    #[garde(ascii, length(min = 8))]
    image_pull_secret: String,

    #[garde(skip)]
    image_cache: Vec<String>,

    #[serde(rename = "UseCSIDriver")]
    #[garde(skip)]
    use_csi_driver: bool,

    #[garde(url)]
    default_cas_url: String,

    #[garde(length(min = 3))]
    default_cas_validate: String,

    #[garde(range(min=0, max=u32::MAX))]
    use_case_chars_min: u32,

    #[garde(dive)]
    default_backend: ViceDefaultBackend,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DatabaseConfig {
    #[garde(ascii, length(min = 3))]
    user: String,

    #[garde(ascii, length(min = 8))]
    password: String,

    #[garde(length(min = 1))]
    host: String,

    #[garde(range(min = 1025, max = 65535))]
    port: u32,

    #[garde(length(min = 1))]
    name: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct QMSDatabaseConfig {
    #[garde(ascii, length(min = 3))]
    user: String,

    #[garde(ascii, length(min = 8))]
    password: String,

    #[garde(length(min = 1))]
    host: String,

    #[garde(range(min = 1025, max = 65535))]
    port: u32,

    #[garde(length(min = 1))]
    name: String,

    #[garde(skip)]
    automigrate: bool,

    #[garde(skip)]
    reinitialize: bool,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Admin {
    #[garde(length(min = 1))]
    groups: String,
    #[garde(length(min = 1))]
    attribute: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Analytics {
    #[garde(skip)]
    enabled: bool,

    #[garde(length(min = 1))]
    id: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Harbor {
    #[serde(rename = "URL")]
    #[garde(url)]
    url: String,

    #[serde(rename = "ProjectQARobotName")]
    #[garde(length(min = 1))]
    project_qa_robot_name: String,

    #[serde(rename = "ProjectQARobotSecret")]
    #[garde(length(min = 1))]
    project_qa_robot_secret: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Qms {
    #[garde(skip)]
    enabled: bool,
}

#[derive(Serialize, Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Jaeger {
    #[garde(url)]
    endpoint: String,

    #[garde(url)]
    http_endpoint: String,
}

#[derive(Serialize, Deserialize, Default, Validate, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigValues {
    #[garde(length(min = 1))]
    environment: String,

    #[garde(length(min = 1))]
    namespace: String,

    #[serde(rename = "UIDDomain")]
    #[garde(length(min = 1))]
    uid_domain: String,

    #[garde(skip)]
    agave: Option<Agave>,

    #[serde(rename = "BaseURLs")]
    #[garde(dive)]
    base_urls: BaseURLs,

    #[garde(dive)]
    dashboard_aggregator: DashboardAggregator,

    #[garde(dive)]
    de: DE,

    #[garde(dive)]
    docker: Docker,

    #[garde(dive)]
    elasticsearch: ElasticSearch,

    #[garde(dive)]
    email: Email,

    #[garde(dive)]
    grouper: Grouper,

    #[serde(rename = "ICAT")]
    #[garde(dive)]
    icat: Icat,

    #[garde(dive)]
    infosquito: Infosquito,

    #[garde(skip)]
    intercom: Option<Intercom>,

    #[serde(rename = "IRODS")]
    #[garde(dive)]
    irods: Irods,

    #[garde(dive)]
    jobs: Jobs,

    #[garde(dive)]
    keycloak: Keycloak,

    #[serde(rename = "PGP")]
    #[garde(dive)]
    pgp: Pgp,

    #[serde(rename = "PermanentID")]
    #[garde(skip)]
    permanent_id: Option<PermanentId>,

    #[garde(length(min = 1))]
    timezone: String,

    #[garde(skip)]
    unleash: Option<Unleash>,

    #[garde(dive)]
    user_portal: UserPortal,

    #[serde(rename = "VICE")]
    #[garde(dive)]
    vice: Vice,

    #[serde(rename = "DEDB")]
    #[garde(dive)]
    de_db: DatabaseConfig,

    #[serde(rename = "GrouperDB")]
    #[garde(dive)]
    grouper_db: DatabaseConfig,

    #[serde(rename = "NewNotificationsDB")]
    #[garde(dive)]
    new_notifications_db: DatabaseConfig,

    #[serde(rename = "NotificationsDB")]
    #[garde(dive)]
    notifications_db: DatabaseConfig,

    #[serde(rename = "PermissionsDB")]
    #[garde(dive)]
    permissions_db: DatabaseConfig,

    #[serde(rename = "QMSDB")]
    #[garde(skip)]
    qms_db: Option<DatabaseConfig>,

    #[serde(rename = "MetadataDB")]
    #[garde(dive)]
    metadata_db: DatabaseConfig,

    #[serde(rename = "UnleashDB")]
    #[garde(skip)]
    unleash_db: Option<DatabaseConfig>,

    #[garde(dive)]
    admin: Admin,

    #[garde(skip)]
    analytics: Option<Analytics>,

    #[garde(dive)]
    harbor: Harbor,

    #[serde(rename = "QMS")]
    #[garde(skip)]
    qms: Option<Qms>,

    #[garde(skip)]
    jaeger: Option<Jaeger>,
}

impl ConfigValues {
    fn new(features: &[OptionalFeatures]) -> Self {
        let mut config_values = ConfigValues::default();

        for feature in features.iter() {
            match feature {
                OptionalFeatures::HighThroughput => {
                    let mut a = Agave::default();
                    a.read_timeout = 30000;
                    a.enabled = false;
                    a.jobs_enabled = false;
                    config_values.agave = Some(a);
                }
                OptionalFeatures::Subscriptions => {
                    let mut s = DESubscriptions::default();
                    s.enforce = false;
                    config_values.de.subscriptions = Some(s);
                }
                OptionalFeatures::Support => {
                    let mut sup = Intercom::default();
                    sup.enabled = false;
                    config_values.intercom = Some(sup);
                }
                OptionalFeatures::DOI => {}
                OptionalFeatures::Analytics => {}
                OptionalFeatures::QuotaEnforcement => {}
                OptionalFeatures::Tracing => {}
            }
        }

        config_values
    }
}

// These are features that are truly optional. In other words, they do not need
// to be present in an installation. This is in contrast to features that are
// optional in an environment specific config_values file because they have
// sane defaults defined in defaults.yaml.
#[derive(PartialEq)]
pub enum OptionalFeatures {
    HighThroughput,
    Subscriptions,
    Support,
    DOI,
    Analytics,
    QuotaEnforcement,
    Tracing,
}
