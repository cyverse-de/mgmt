use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Agave {
    key: String,
    secret: String,

    #[serde(rename = "RedirectURI")]
    redirect_uri: String,

    #[serde(rename = "CallbackBaseURI")]
    callback_base_uri: String,

    read_timeout: i32,
    enabled: bool,
    jobs_enabled: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "AMQP")]
pub struct Amqp {
    user: String,
    password: String,
    host: String,
    port: i32,
    vhost: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BaseURLs {
    analyses: String,
    app_exposer: String,
    apps: String,
    async_tasks: String,
    dashboard_aggregator: String,
    data_info: String,
    grouper_web_services: String,
    iplant_email: String,
    iplant_groups: String,
    jex_adapter: String,
    job_status_listener: String,
    metadata: String,
    notifications: String,
    permissions: String,
    #[serde(rename = "QMS")]
    qms: String,
    requests: String,
    search: String,
    terrain: String,
    user_info: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Website {
    #[serde(rename = "URL")]
    url: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DashboardAggregator {
    public_group: String,
    website: Website,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DESubscriptions {
    #[serde(rename = "CheckoutURL")]
    checkout_url: String,
    enforce: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DECoge {
    #[serde(rename = "BaseURI")]
    base_uri: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DETools {
    admin: DEToolsAdmin,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DEToolsAdmin {
    max_cpu_limit: u32,
    max_memory_limit: u64,
    max_disk_limit: u64,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DE {
    amqp: Amqp,
    base_uri: String,
    subscriptions: DESubscriptions,
    default_output_folder: String,
    coge: DECoge,
    tools: DETools,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Docker {
    trusted_registries: Vec<String>,
    tag: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ElasticSearch {
    #[serde(rename = "BaseURI")]
    base_uri: String,
    username: String,
    password: String,
    index: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Email {
    src: String,
    dest: String,

    #[serde(rename = "PermIDRequestDest")]
    perm_id_request_dest: String,

    support_dest: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GrouperLoader {
    #[serde(rename = "URI")]
    uri: String,
    user: String,
    password: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Grouper {
    morph_string: String,
    password: String,
    folder_name_prefix: String,
    loader: GrouperLoader,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "ICAT")]
pub struct Icat {
    host: String,
    port: u32,
    user: String,
    password: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Infosquito {
    day_num: u8,
    prefix_length: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Intercom {
    enabled: bool,

    #[serde(rename = "AppID")]
    app_id: String,

    #[serde(rename = "CompanyID")]
    company_id: String,

    company_name: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IrodsWebDav {
    #[serde(rename = "AnonURI")]
    anon_uri: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Irods {
    #[serde(rename = "AMQP")]
    amqp: Amqp,

    host: String,
    user: String,
    zone: String,
    password: String,
    admin_users: Vec<String>,
    perms_filter: Vec<String>,
    external_host: String,
    web_dav: IrodsWebDav,
    quota_root_resources: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Jobs {
    data_transfer_image: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct KeycloakVice {
    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Keycloak {
    server_uri: String,
    realm: String,

    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,

    #[serde(rename = "VICE")]
    vice: KeycloakVice,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "PGP")]
pub struct Pgp {
    key_password: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PermanentIdDataCite {
    #[serde(rename = "BaseURI")]
    base_uri: String,

    user: String,

    password: String,

    #[serde(rename = "DOIPrefix")]
    doi_prefix: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PermanentId {
    curators_group: String,
    data_cite: PermanentIdDataCite,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Unleash {
    #[serde(rename = "BaseURL")]
    base_url: String,

    #[serde(rename = "APIPath")]
    api_path: String,

    #[serde(rename = "APIToken")]
    api_token: String,

    maintenance_flag: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UserPortal {
    #[serde(rename = "BaseURI")]
    base_uri: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ViceFileTransfers {
    image: String,
    tag: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ViceDefaultBackend {
    loading_page_template_string: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Vice {
    #[serde(rename = "BaseURI")]
    base_uri: String,

    file_transfers: ViceFileTransfers,
    image_pull_secret: String,
    image_cache: Vec<String>,

    #[serde(rename = "UseCSIDriver")]
    use_csi_driver: bool,

    default_cas_url: String,
    default_cas_validate: String,
    use_case_chars_min: u32,
    default_backend: ViceDefaultBackend,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DatabaseConfig {
    user: String,
    password: String,
    host: String,
    port: u32,
    name: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct QMSDatabaseConfig {
    user: String,
    password: String,
    host: String,
    port: u32,
    name: String,
    automigrate: bool,
    reinitialize: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Admin {
    groups: String,
    attribute: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Analytics {
    enabled: bool,
    id: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Harbor {
    #[serde(rename = "URL")]
    url: String,

    #[serde(rename = "ProjectQARobotName")]
    project_qa_robot_name: String,

    #[serde(rename = "ProjectQARobotSecret")]
    project_qa_robot_secret: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Qms {
    enabled: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Jaeger {
    endpoint: String,
    http_endpoint: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigValues {
    environment: String,
    namespace: String,

    #[serde(rename = "UIDDomain")]
    uid_domain: String,

    agave: Agave,

    #[serde(rename = "BaseURLs")]
    base_urls: BaseURLs,

    dashboard_aggregator: DashboardAggregator,
    de: DE,
    docker: Docker,
    elasticsearch: ElasticSearch,
    email: Email,
    grouper: Grouper,

    #[serde(rename = "ICAT")]
    icat: Icat,

    infosquito: Infosquito,
    intercom: Intercom,

    #[serde(rename = "IRODS")]
    irods: Irods,

    jobs: Jobs,
    keycloak: Keycloak,

    #[serde(rename = "PGP")]
    pgp: Pgp,

    #[serde(rename = "PermanentID")]
    permanent_id: PermanentId,

    timezone: String,
    unleash: Unleash,
    user_portal: UserPortal,

    #[serde(rename = "VICE")]
    vice: Vice,

    #[serde(rename = "DEDB")]
    de_db: DatabaseConfig,

    #[serde(rename = "GrouperDB")]
    grouper_db: DatabaseConfig,

    #[serde(rename = "NewNotificationsDB")]
    new_notifications_db: DatabaseConfig,

    #[serde(rename = "NotificationsDB")]
    notifications_db: DatabaseConfig,

    #[serde(rename = "PermissionsDB")]
    permissions_db: DatabaseConfig,

    #[serde(rename = "QMSDB")]
    qms_db: DatabaseConfig,

    #[serde(rename = "MetadataDB")]
    metadata_db: DatabaseConfig,

    #[serde(rename = "UnleashDB")]
    unleash_db: DatabaseConfig,

    admin: Admin,
    analytics: Analytics,
    harbor: Harbor,

    #[serde(rename = "QMS")]
    qms: Qms,

    jaeger: Jaeger,
}
