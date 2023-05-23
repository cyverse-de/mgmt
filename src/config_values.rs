use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Agave {
    key: String,
    secret: String,
    redirect_uri: String,
    callback_base_uri: String,
    read_timeout: i32,
    enabled: bool,
    jobs_enabled: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Amqp {
    user: String,
    password: String,
    host: String,
    port: i32,
    vhost: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
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
    qms: String,
    requests: String,
    search: String,
    terrain: String,
    user_info: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Website {
    url: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DashboardAggregator {
    public_group: String,
    website: Website,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DESubscriptions {
    checkout_url: String,
    enforce: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DECoge {
    base_uri: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DETools {
    admin: DEToolsAdmin,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DEToolsAdmin {
    max_cpu_limit: u32,
    max_memory_limit: u64,
    max_disk_limit: u64,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DE {
    amqp: Amqp,
    base_uri: String,
    subscriptions: DESubscriptions,
    default_output_folder: String,
    coge: DECoge,
    tools: DETools,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Docker {
    trusted_registries: Vec<String>,
    tag: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ElasticSearch {
    base_uri: String,
    username: String,
    password: String,
    index: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Email {
    src: String,
    dest: String,
    perm_id_request_dest: String,
    support_dest: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct GrouperLoader {
    uri: String,
    user: String,
    password: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Grouper {
    morph_string: String,
    password: String,
    folder_name_prefix: String,
    loader: GrouperLoader,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ICAT {
    host: String,
    port: u32,
    user: String,
    password: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Infosquito {
    day_num: u8,
    prefix_length: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Intercom {
    enabled: bool,
    app_id: String,
    company_id: String,
    company_name: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct IrodsWebDav {
    anon_uri: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Irods {
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
pub struct Jobs {
    data_transfer_image: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct KeycloakVice {
    client_id: String,
    client_secret: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Keycloak {
    server_uri: String,
    realm: String,
    client_id: String,
    client_secret: String,
    vice: KeycloakVice,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Pgp {
    key_password: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PermanentIdDataCite {
    base_uri: String,
    user: String,
    password: String,
    doi_prefix: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PermanentId {
    curators_group: String,
    data_cite: PermanentIdDataCite,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Unleash {
    base_url: String,
    api_path: String,
    api_token: String,
    maintenance_flag: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UserPortal {
    base_uri: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ViceFileTransfers {
    image: String,
    tag: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ViceDefaultBackend {
    loading_page_template_string: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Vice {
    base_uri: String,
    file_transfers: ViceFileTransfers,
    image_pull_secret: String,
    image_cache: Vec<String>,
    use_csi_driver: bool,
    default_cas_url: String,
    default_cas_validate: String,
    use_case_chars_min: u32,
    default_backend: ViceDefaultBackend,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DatabaseConfig {
    user: String,
    password: String,
    host: String,
    port: u32,
    name: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Admin {
    groups: String,
    attribute: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Analytics {
    enabled: bool,
    id: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Harbor {
    url: String,
    project_qa_robot_name: String,
    project_qa_robot_secret: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Qms {
    enabled: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Jaeger {
    endpoint: String,
    http_endpoint: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigValues {
    environment: String,
    namespace: String,
    uid_domain: String,
    agave: Agave,
    base_urls: BaseURLs,
    dashboard_aggregator: DashboardAggregator,
    de: DE,
    docker: Docker,
    elasticsearch: ElasticSearch,
    email: Email,
    grouper: Grouper,
    icat: ICAT,
    infosquito: Infosquito,
    intercom: Intercom,
    irods: Irods,
    jobs: Jobs,
    keycloak: Keycloak,
    pgp: Pgp,
    permanent_id: PermanentId,
    timezone: String,
    unleash: Unleash,
    user_portal: UserPortal,
    vice: Vice,
    de_db: DatabaseConfig,
    grouper_db: DatabaseConfig,
    new_notifications_db: DatabaseConfig,
    notifications_db: DatabaseConfig,
    permissions_db: DatabaseConfig,
    qms_db: DatabaseConfig,
    metadata_db: DatabaseConfig,
    unleash_db: DatabaseConfig,
    admin: Admin,
    analytics: Analytics,
    harbor: Harbor,
    qms: Qms,
    jaeger: Jaeger,
}
