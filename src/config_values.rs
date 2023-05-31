use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Agave {
    key: String,
    secret: String,

    #[serde(rename = "RedirectURI")]
    redirect_uri: String,

    storage_system: String,

    #[serde(rename = "CallbackBaseURI")]
    callback_base_uri: String,

    read_timeout: Option<u32>,
    enabled: Option<bool>,
    jobs_enabled: Option<bool>,
}

impl Default for Agave {
    fn default() -> Self {
        Agave {
            key: String::new(),
            secret: String::new(),
            redirect_uri: String::new(),
            storage_system: String::new(),
            callback_base_uri: String::new(),
            read_timeout: Some(30000),
            enabled: Some(false),
            jobs_enabled: Some(false),
        }
    }
}

impl Agave {
    pub fn merge(&self, right: &Agave) -> anyhow::Result<Agave> {
        Ok(serde_merge::omerge(self, right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "AMQP")]
pub struct Amqp {
    user: String,
    password: String,
    host: String,
    port: u16,
    vhost: String,
}

impl Amqp {
    fn merge(&self, right: &Amqp) -> anyhow::Result<Amqp> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BaseURLs {
    analyses: Option<Url>,
    app_exposer: Option<Url>,
    apps: Option<Url>,
    async_tasks: Option<Url>,
    dashboard_aggregator: Option<Url>,
    data_info: Option<Url>,
    grouper_web_services: Option<Url>,
    iplant_email: Option<Url>,
    iplant_groups: Option<Url>,
    jex_adapter: Option<Url>,
    job_status_listener: Option<Url>,
    metadata: Option<Url>,
    notifications: Option<Url>,
    permissions: Option<Url>,

    #[serde(rename = "QMS")]
    qms: Option<Url>,

    requests: Option<Url>,
    search: Option<Url>,
    terrain: Option<Url>,
    user_info: Option<Url>,
}

impl Default for BaseURLs {
    fn default() -> Self {
        BaseURLs {
            analyses: Url::parse("http://analyses").ok(),
            app_exposer: Url::parse("http://app-exposer").ok(),
            apps: Url::parse("http://apps").ok(),
            async_tasks: Url::parse("http://async-tasks").ok(),
            dashboard_aggregator: Url::parse("http://dashboard-aggregator").ok(),
            data_info: Url::parse("http://data-info").ok(),
            grouper_web_services: Url::parse("http://grouper-ws/grouper-ws").ok(),
            iplant_email: Url::parse("http://de-mailer").ok(),
            iplant_groups: Url::parse("http://iplant-groups").ok(),
            jex_adapter: Url::parse("http://jex-adapter").ok(),
            job_status_listener: Url::parse("http://job-status-listener").ok(),
            metadata: Url::parse("http://metadata").ok(),
            notifications: Url::parse("http://notifications").ok(),
            permissions: Url::parse("http://permissions").ok(),
            qms: Url::parse("http://qms").ok(),
            requests: Url::parse("http://requests").ok(),
            search: Url::parse("http://search").ok(),
            terrain: Url::parse("http://terrain").ok(),
            user_info: Url::parse("http://user_info").ok(),
        }
    }
}

impl BaseURLs {
    fn merge(&self, right: &BaseURLs) -> anyhow::Result<BaseURLs> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Website {
    #[serde(rename = "URL")]
    url: Option<url::Url>,
}

impl Default for Website {
    fn default() -> Self {
        Website {
            url: Url::parse("https://cyverse.org").ok(),
        }
    }
}

impl Website {
    fn merge(&self, right: &Website) -> anyhow::Result<Website> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DashboardAggregator {
    public_group: String,
    website: Option<Website>,
}

impl DashboardAggregator {
    fn merge(&self, right: &DashboardAggregator) -> anyhow::Result<DashboardAggregator> {
        let mut merged: DashboardAggregator = serde_merge::omerge(&self, &right)?;
        if let Some(website) = &self.website {
            if let Some(right_website) = &right.website {
                merged.website = Some(website.merge(right_website)?);
            }
        }
        Ok(merged)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DESubscriptions {
    #[serde(rename = "CheckoutURL")]
    checkout_url: Option<Url>,

    enforce: bool,
}

impl DESubscriptions {
    fn merge(&self, right: &DESubscriptions) -> anyhow::Result<DESubscriptions> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DECoge {
    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,
}

impl DECoge {
    fn merge(&self, right: &DECoge) -> anyhow::Result<DECoge> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

impl Default for DECoge {
    fn default() -> Self {
        DECoge {
            base_uri: Url::parse("https://genomevolution.org/coge/api/v1").ok(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DETools {
    admin: DEToolsAdmin,
}

impl DETools {
    fn merge(&self, right: &DETools) -> anyhow::Result<DETools> {
        let mut merged: DETools = serde_merge::omerge(&self, &right)?;
        merged.admin = self.admin.merge(&right.admin)?;
        Ok(merged)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DEToolsAdmin {
    max_cpu_limit: Option<u32>,
    max_memory_limit: Option<u64>,
    max_disk_limit: Option<u64>,
}

impl Default for DEToolsAdmin {
    fn default() -> Self {
        DEToolsAdmin {
            max_cpu_limit: Some(24),
            max_memory_limit: Some(75161927680),
            max_disk_limit: Some(1099511627776),
        }
    }
}

impl DEToolsAdmin {
    fn merge(&self, right: &DEToolsAdmin) -> anyhow::Result<DEToolsAdmin> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DE {
    #[serde(rename = "AMQP")]
    amqp: Amqp,

    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>, //Required before deployment.

    subscriptions: Option<DESubscriptions>,
    default_output_folder: String,
    coge: Option<DECoge>,
    tools: Option<DETools>,
}

impl Default for DE {
    fn default() -> Self {
        DE {
            amqp: Amqp::default(),
            base_uri: Url::parse("https://de.cyverse.org").ok(),
            subscriptions: Some(DESubscriptions::default()),
            default_output_folder: String::from("analyses"),
            coge: Some(DECoge::default()),
            tools: Some(DETools::default()),
        }
    }
}

impl DE {
    fn merge(&self, right: &DE) -> anyhow::Result<DE> {
        let mut merged: DE = serde_merge::omerge(&self, &right)?;
        if let Some(subscriptions) = &self.subscriptions {
            if let Some(right_subscriptions) = &right.subscriptions {
                merged.subscriptions = Some(subscriptions.merge(right_subscriptions)?);
            }
        }
        if let Some(coge) = &self.coge {
            if let Some(right_coge) = &right.coge {
                merged.coge = Some(coge.merge(&right_coge)?);
            }
        }
        if let Some(tools) = &self.tools {
            if let Some(right_tools) = &right.tools {
                merged.tools = Some(tools.merge(&right_tools)?);
            }
        }
        Ok(merged)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Docker {
    trusted_registries: Option<Vec<String>>,
    tag: String,
}

impl Default for Docker {
    fn default() -> Self {
        Docker {
            tag: String::from("latest"),
            trusted_registries: Some(vec![
                String::from("harbor.cyverse.org"),
                String::from("docker.cyverse.org"),
            ]),
        }
    }
}

impl Docker {
    fn merge(&self, right: &Docker) -> anyhow::Result<Docker> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ElasticSearch {
    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,

    username: String,
    password: String,
    index: String,
}

impl ElasticSearch {
    fn merge(&self, right: &ElasticSearch) -> anyhow::Result<ElasticSearch> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
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

impl Email {
    fn merge(&self, right: &Email) -> anyhow::Result<Email> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GrouperLoader {
    #[serde(rename = "URI")]
    uri: Option<Url>,

    user: String,
    password: String,
}

impl GrouperLoader {
    fn merge(&self, right: &GrouperLoader) -> anyhow::Result<GrouperLoader> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Grouper {
    morph_string: String,
    password: String,
    folder_name_prefix: String,
    loader: GrouperLoader,
}

impl Grouper {
    fn merge(&self, right: &Grouper) -> anyhow::Result<Grouper> {
        let mut merged: Grouper = serde_merge::omerge(&self, &right)?;
        merged.loader = self.loader.merge(&right.loader)?;
        Ok(merged)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "ICAT")]
pub struct Icat {
    host: String,
    port: u16,
    user: String,
    password: String,
}

impl Icat {
    fn merge(&self, right: &Icat) -> anyhow::Result<Icat> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Infosquito {
    day_num: Option<u8>,
    prefix_length: Option<u32>,
}

impl Default for Infosquito {
    fn default() -> Self {
        Infosquito {
            day_num: Some(4),
            prefix_length: Some(4),
        }
    }
}

impl Infosquito {
    fn merge(&self, right: &Infosquito) -> anyhow::Result<Infosquito> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
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

impl Intercom {
    fn merge(&self, right: &Intercom) -> anyhow::Result<Intercom> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IrodsWebDav {
    #[serde(rename = "AnonURI")]
    anon_uri: Option<Url>,
}

impl Default for IrodsWebDav {
    fn default() -> Self {
        IrodsWebDav {
            anon_uri: Url::parse("https://data.cyverse.rocksi/dav-anon").ok(),
        }
    }
}

impl IrodsWebDav {
    fn merge(&self, right: &IrodsWebDav) -> anyhow::Result<IrodsWebDav> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
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
    external_host: Option<String>,
    web_dav: Option<IrodsWebDav>,
    quota_root_resources: Option<String>,
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
            web_dav: Some(IrodsWebDav::default()),
            external_host: Some(String::from("data.cyverse.rocks")),
            quota_root_resources: Some(String::from("mainIngestRes,mainReplRes")),
        }
    }
}

impl Irods {
    fn merge(&self, right: &Irods) -> anyhow::Result<Irods> {
        let mut merged: Irods = serde_merge::omerge(&self, &right)?;
        if let Some(web_dav) = &self.web_dav {
            if let Some(right_web_dav) = &right.web_dav {
                merged.web_dav = Some(web_dav.merge(right_web_dav)?);
            }
        }
        merged.amqp = self.amqp.merge(&right.amqp)?;
        Ok(merged)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Jobs {
    data_transfer_image: String,
}

impl Default for Jobs {
    fn default() -> Self {
        Jobs {
            data_transfer_image: String::from("harbor.cyverse.org/de/porklock"),
        }
    }
}

impl Jobs {
    fn merge(&self, right: &Jobs) -> anyhow::Result<Jobs> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct KeycloakVice {
    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,
}

impl KeycloakVice {
    fn merge(&self, right: &KeycloakVice) -> anyhow::Result<KeycloakVice> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Keycloak {
    server_uri: Option<Url>,
    realm: String,

    #[serde(rename = "ClientID")]
    client_id: String,

    client_secret: String,

    #[serde(rename = "VICE")]
    vice: KeycloakVice,
}

impl Keycloak {
    fn merge(&self, right: &Keycloak) -> anyhow::Result<Keycloak> {
        let mut merged: Keycloak = serde_merge::omerge(&self, &right)?;
        merged.vice = self.vice.merge(&right.vice)?;
        Ok(merged)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "PGP")]
pub struct Pgp {
    key_password: String,
}

impl Pgp {
    fn merge(&self, right: &Pgp) -> anyhow::Result<Pgp> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PermanentIdDataCite {
    #[serde(rename = "BaseURI")]
    base_uri: Option<Url>,

    user: String,
    password: String,

    #[serde(rename = "DOIPrefix")]
    doi_prefix: String,
}

impl PermanentIdDataCite {
    fn merge(&self, right: &PermanentIdDataCite) -> anyhow::Result<PermanentIdDataCite> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PermanentId {
    curators_group: String,
    data_cite: PermanentIdDataCite,
}

impl PermanentId {
    fn merge(&self, right: &PermanentId) -> anyhow::Result<PermanentId> {
        let mut merged: PermanentId = serde_merge::omerge(&self, &right)?;
        merged.data_cite = self.data_cite.merge(&right.data_cite)?;
        Ok(merged)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Unleash {
    #[serde(rename = "BaseURL")]
    base_url: Option<Url>,

    #[serde(rename = "APIPath")]
    api_path: Option<String>,

    #[serde(rename = "APIToken")]
    api_token: String,

    maintenance_flag: Option<String>,
}

impl Default for Unleash {
    fn default() -> Self {
        Unleash {
            base_url: Url::parse("http://unleash:4242").ok(),
            api_path: Some(String::from("/api")),
            maintenance_flag: Some(String::from("DE-Maintenance")),
            api_token: String::new(),
        }
    }
}

impl Unleash {
    fn merge(&self, right: &Unleash) -> anyhow::Result<Unleash> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UserPortal {
    #[serde(rename = "BaseURI")]
    base_uri: Option<String>,
}

impl UserPortal {
    fn merge(&self, right: &UserPortal) -> anyhow::Result<UserPortal> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

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
    fn merge(&self, right: &ViceFileTransfers) -> anyhow::Result<ViceFileTransfers> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ViceDefaultBackend {
    loading_page_template_string: String,
}

impl ViceDefaultBackend {
    fn merge(&self, right: &ViceDefaultBackend) -> anyhow::Result<ViceDefaultBackend> {
        Ok(serde_merge::omerge(&self, &right)?)
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
    fn merge(&self, right: &Vice) -> anyhow::Result<Vice> {
        let mut merged: Vice = serde_merge::omerge(&self, &right)?;
        if let Some(ft) = &merged.file_transfers {
            if let Some(right_ft) = &right.file_transfers {
                merged.file_transfers = Some(ft.merge(right_ft)?);
            }
        }
        merged.default_backend = self.default_backend.merge(&right.default_backend)?;
        Ok(merged)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DatabaseConfig {
    user: String,
    password: String,
    host: String,
    port: u32,
    name: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            user: String::new(),
            password: String::new(),
            host: String::new(),
            port: 5432,
            name: String::new(),
        }
    }
}

impl DatabaseConfig {
    fn merge(&self, right: &DatabaseConfig) -> anyhow::Result<DatabaseConfig> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct QMSDatabaseConfig {
    user: String,
    password: String,
    host: String,
    port: Option<u32>,
    name: Option<String>,
    automigrate: Option<bool>,
    reinitialize: Option<bool>,
}

impl Default for QMSDatabaseConfig {
    fn default() -> Self {
        QMSDatabaseConfig {
            user: String::new(),
            password: String::new(),
            host: String::new(),
            port: Some(5432),
            name: Some(String::from("qms")),
            automigrate: Some(false),
            reinitialize: Some(false),
        }
    }
}

impl QMSDatabaseConfig {
    fn merge(&self, right: &QMSDatabaseConfig) -> anyhow::Result<QMSDatabaseConfig> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Admin {
    groups: Option<String>,
    attribute: Option<String>,
}

impl Default for Admin {
    fn default() -> Self {
        Admin {
            groups: Some(String::from(
                "core-service,tito-admins,tito-qa-admins,dev,staff",
            )),
            attribute: Some(String::from("entitlement")),
        }
    }
}

impl Admin {
    fn merge(&self, right: &Admin) -> anyhow::Result<Admin> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Analytics {
    enabled: Option<bool>,
    id: Option<String>,
}

impl Default for Analytics {
    fn default() -> Self {
        Analytics {
            enabled: Some(false),
            id: Some(String::from("g-id")),
        }
    }
}

impl Analytics {
    fn merge(&self, right: &Analytics) -> anyhow::Result<Analytics> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Harbor {
    #[serde(rename = "URL")]
    url: Option<String>, // called a URL, but it's actually a host name.

    #[serde(rename = "ProjectQARobotName")]
    project_qa_robot_name: String,

    #[serde(rename = "ProjectQARobotSecret")]
    project_qa_robot_secret: String,
}

impl Default for Harbor {
    fn default() -> Self {
        Harbor {
            url: Some(String::from("harbor.cyverse.org")),
            project_qa_robot_name: String::new(),
            project_qa_robot_secret: String::new(),
        }
    }
}

impl Harbor {
    fn merge(&self, right: &Harbor) -> anyhow::Result<Harbor> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Qms {
    enabled: Option<bool>,
}

impl Default for Qms {
    fn default() -> Self {
        Qms {
            enabled: Some(true),
        }
    }
}

impl Qms {
    fn merge(&self, right: &Qms) -> anyhow::Result<Qms> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Jaeger {
    endpoint: Option<Url>,
    http_endpoint: Option<Url>,
}

impl Default for Jaeger {
    fn default() -> Self {
        Jaeger {
            endpoint: Url::parse("http://jaeger-collector.jaeger.svc.cluster.local:14250").ok(),
            http_endpoint: Url::parse(
                "http://jaeger-collector.jaeger.svc.cluster.local:14268/api/traces",
            )
            .ok(),
        }
    }
}

impl Jaeger {
    fn merge(&self, right: &Jaeger) -> anyhow::Result<Jaeger> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}

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
    intercom: Option<Intercom>,

    // Must be configured for deployment.
    #[serde(rename = "IRODS")]
    irods: Irods,

    // Defaults are provided for deployment.
    jobs: Option<Jobs>,

    // Must be configured for deployment.
    keycloak: Keycloak,

    // Must be configured for deployment.
    #[serde(rename = "PGP")]
    pgp: Pgp,

    // Optional for deployment.
    #[serde(rename = "PermanentID")]
    permanent_id: Option<PermanentId>,

    // Defaults provided for deployment.
    timezone: Option<String>,

    // Optional for deployment.
    unleash: Option<Unleash>,

    // Required for deployment
    user_portal: UserPortal,

    // Required for deployment.
    #[serde(rename = "VICE")]
    vice: Vice,

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
    admin: Option<Admin>,

    // Optional for deployment.
    analytics: Option<Analytics>,

    // Defaults provided for deployment.
    harbor: Option<Harbor>,

    // Optional for deployment.
    #[serde(rename = "QMS")]
    qms: Option<Qms>,

    // Optional for deployment.
    jaeger: Option<Jaeger>,
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
            intercom: Some(Intercom::default()),
            irods: Irods::default(),
            jobs: Some(Jobs::default()),
            keycloak: Keycloak::default(),
            pgp: Pgp::default(),
            permanent_id: Some(PermanentId::default()),
            timezone: Some(String::new()),
            unleash: Some(Unleash::default()),
            user_portal: UserPortal::default(),
            vice: Vice::default(),
            de_db: DatabaseConfig::default(),
            grouper_db: DatabaseConfig::default(),
            new_notifications_db: DatabaseConfig::default(),
            notifications_db: DatabaseConfig::default(),
            permissions_db: DatabaseConfig::default(),
            qms_db: QMSDatabaseConfig::default(),
            metadata_db: DatabaseConfig::default(),
            unleash_db: DatabaseConfig::default(),
            admin: Some(Admin::default()),
            analytics: Some(Analytics::default()),
            harbor: Some(Harbor::default()),
            qms: Some(Qms::default()),
            jaeger: Some(Jaeger::default()),
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
