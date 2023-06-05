use dialoguer::{console::Style, theme::ColorfulTheme, Input, Password, Select};
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

    pub fn ask_for_info(
        &mut self,
        theme: &ColorfulTheme,
        base_url: &url::Url,
        irods_external: &str,
    ) -> anyhow::Result<()> {
        let df_base_url = base_url.clone().join("/de/agave-cb")?;
        let callback_base_uri = Input::<String>::with_theme(theme)
            .with_prompt("Agave Callback Base URI")
            .default(df_base_url.to_string())
            .interact()?;
        self.callback_base_uri = callback_base_uri;

        let rd_uri = base_url.clone().join("/oauth/callback/agave")?;
        let redirect_uri = Input::<String>::with_theme(theme)
            .with_prompt("Agave Redirect URI")
            .default(rd_uri.to_string())
            .interact()?;
        self.redirect_uri = redirect_uri;

        let agave_key = Input::<String>::with_theme(theme)
            .with_prompt("Agave Key")
            .interact()?;

        self.key = agave_key;

        let secret = Input::<String>::with_theme(theme)
            .with_prompt("Agave Secret")
            .interact()?;

        self.secret = secret;

        let storage_system = Input::<String>::with_theme(theme)
            .with_prompt("Agave Storage System")
            .default(irods_external.into())
            .interact()?;

        self.storage_system = storage_system;
        self.enabled = Some(true);

        let read_timeout = Input::<u32>::with_theme(theme)
            .with_prompt("Agave Read Timeout")
            .default(30000)
            .interact()?;
        self.read_timeout = Some(read_timeout);

        let jobs_enabled = Select::with_theme(theme)
            .with_prompt("Agave Jobs Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;
        self.jobs_enabled = Some(jobs_enabled == 0);

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme, prefix: &str) -> anyhow::Result<()> {
        let user = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} AMQP User", prefix))
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt(format!("{} AMQP Password", prefix))
            .interact()?;

        let host = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} AMQP Host", prefix))
            .interact()?;

        let port = Input::<u16>::with_theme(theme)
            .with_prompt(format!("{} AMQP Port", prefix))
            .default(5672)
            .interact()?;

        let vhost = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} AMQP VHost", prefix))
            .interact()?;

        self.user = user;
        self.password = password;
        self.host = host;
        self.port = port;
        self.vhost = vhost;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let url = Input::<String>::with_theme(theme)
            .with_prompt("Dashboard Website URL")
            .default("https://cyverse.org".into())
            .interact()?;

        self.url = Url::parse(&url).ok();

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let public_group = Input::<String>::with_theme(theme)
            .with_prompt("Dashboard Public Group")
            .interact()?;

        self.public_group = public_group;
        let mut new_website = Website::default();
        new_website.ask_for_info(theme)?;
        self.website = Some(new_website);

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let checkout_url = Input::<String>::with_theme(theme)
            .with_prompt("Subscriptions Checkout URL")
            .default("https://cyverse-subscription.phoenixbioinformatics.org".into())
            .interact()?;

        self.checkout_url = Url::parse(&checkout_url).ok();

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("CoGe Base URI")
            .default("https://genomevolution.org/coge/api/v1".into())
            .interact()?;

        self.base_uri = Url::parse(&base_uri).ok();

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let max_cpu_limit = Input::<u32>::with_theme(theme)
            .with_prompt("Max CPU Limit")
            .default(24)
            .interact()?;

        self.admin.max_cpu_limit = Some(max_cpu_limit);

        let max_memory_limit = Input::<u64>::with_theme(theme)
            .with_prompt("Max Memory Limit")
            .default(75161927680)
            .interact()?;

        self.admin.max_memory_limit = Some(max_memory_limit);

        let max_disk_limit = Input::<u64>::with_theme(theme)
            .with_prompt("Max Disk Limit")
            .default(1099511627776)
            .interact()?;

        self.admin.max_disk_limit = Some(max_disk_limit);

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        self.amqp.ask_for_info(theme, "DE")?;

        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("DE Base URI")
            .interact()?;

        self.base_uri = Url::parse(&base_uri).ok();

        let mut new_subs = DESubscriptions::default();
        new_subs.ask_for_info(theme)?;
        self.subscriptions = Some(new_subs);

        let default_output_folder = Input::<String>::with_theme(theme)
            .with_prompt("DE Default Output Folder")
            .default("analyses".into())
            .interact()?;
        self.default_output_folder = default_output_folder;

        let mut new_coge = DECoge::default();
        new_coge.ask_for_info(theme)?;
        self.coge = Some(new_coge);

        let mut new_tools = DETools::default();
        new_tools.ask_for_info(theme)?;
        self.tools = Some(new_tools);

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let tag = Input::<String>::with_theme(theme)
            .with_prompt("Docker Tag")
            .default("latest".into())
            .interact()?;

        self.tag = tag;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Base URI")
            .default("http://elasticsearch:9200".into())
            .interact()?;

        let username = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Username")
            .allow_empty(true)
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("ElasticSearch Password")
            .allow_empty_password(true)
            .interact()?;

        let index = Input::<String>::with_theme(theme)
            .with_prompt("ElasticSearch Index")
            .default("data".into())
            .interact()?;

        self.base_uri = Url::parse(&base_uri).ok();
        self.username = username;
        self.password = password;
        self.index = index;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let src = Input::<String>::with_theme(theme)
            .with_prompt("Email Source")
            .interact()?;

        let dest = Input::<String>::with_theme(theme)
            .with_prompt("Email Destination")
            .interact()?;

        let perm_id_default = dest.clone();
        let perm_id_request_dest = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID Request Destination")
            .default(perm_id_default)
            .interact()?;

        let support_dest_default = dest.clone();
        let support_dest = Input::<String>::with_theme(theme)
            .with_prompt("Support Destination")
            .default(support_dest_default)
            .interact()?;

        self.src = src;
        self.dest = dest;
        self.perm_id_request_dest = perm_id_request_dest;
        self.support_dest = support_dest;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let uri = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Loader URI")
            .interact()?;

        let user = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Loader User")
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("Grouper Loader Password")
            .interact()?;

        self.uri = Url::parse(&uri).ok();
        self.user = user;
        self.password = password;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme, env: &str) -> anyhow::Result<()> {
        let morph_string = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Morph String")
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("Grouper Password")
            .interact()?;

        let folder_name_prefix = Input::<String>::with_theme(theme)
            .with_prompt("Grouper Folder Name Prefix")
            .default(format!("cyverse:de:{}", env).into())
            .interact()?;

        self.morph_string = morph_string;
        self.password = password;
        self.folder_name_prefix = folder_name_prefix;
        self.loader.ask_for_info(theme)?;
        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let host = Input::<String>::with_theme(theme)
            .with_prompt("ICAT Host")
            .interact()?;

        let port = Input::<u16>::with_theme(theme)
            .with_prompt("ICAT Port")
            .default(1247)
            .interact()?;

        let user = Input::<String>::with_theme(theme)
            .with_prompt("ICAT User")
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("ICAT Password")
            .interact()?;

        self.host = host;
        self.port = port;
        self.user = user;
        self.password = password;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let day_num = Input::<u8>::with_theme(theme)
            .with_prompt("Infosquito Day Number")
            .default(4)
            .interact()?;

        let prefix_length = Input::<u32>::with_theme(theme)
            .with_prompt("Infosquito Prefix Length")
            .default(4)
            .interact()?;

        self.day_num = Some(day_num);
        self.prefix_length = Some(prefix_length);

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let enabled = Select::with_theme(theme)
            .with_prompt("Intercom Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        let app_id = Input::<String>::with_theme(theme)
            .with_prompt("Intercom App ID")
            .interact()?;

        let company_id = Input::<String>::with_theme(theme)
            .with_prompt("Intercom Company ID")
            .interact()?;

        let company_name = Input::<String>::with_theme(theme)
            .with_prompt("Intercom Company Name")
            .interact()?;

        self.enabled = enabled == 0;
        self.app_id = app_id;
        self.company_id = company_id;
        self.company_name = company_name;

        Ok(())
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
            anon_uri: Url::parse("https://data.cyverse.rocks/dav-anon").ok(),
        }
    }
}

impl IrodsWebDav {
    fn merge(&self, right: &IrodsWebDav) -> anyhow::Result<IrodsWebDav> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme, external: &str) -> anyhow::Result<()> {
        let anon_uri = Input::<String>::with_theme(theme)
            .with_prompt("Irods WebDav Anon URI")
            .default(format!("https://{}/dav-anon", external))
            .interact()?;

        self.anon_uri = Url::parse(&anon_uri).ok();

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        self.amqp.ask_for_info(theme, "iRODS")?;

        let host = Input::<String>::with_theme(theme)
            .with_prompt("iRODS Host")
            .interact()?;

        let external_host = Input::<String>::with_theme(theme)
            .with_prompt("iRODS External Host")
            .default(host.clone())
            .interact()?;

        let user = Input::<String>::with_theme(theme)
            .with_prompt("iRODS User")
            .interact()?;

        let zone = Input::<String>::with_theme(theme)
            .with_prompt("iRODS Zone")
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("iRODS Password")
            .interact()?;

        let admin_users = Input::<String>::with_theme(theme)
            .with_prompt("iRODS Admin Users")
            .interact()?;

        let perms_filter = Input::<String>::with_theme(theme)
            .with_prompt("iRODS Perms Filter")
            .interact()?;

        self.host = host;
        self.external_host = Some(external_host);
        self.user = user;
        self.zone = zone;
        self.password = password;
        self.admin_users = admin_users.split(',').map(|s| s.to_string()).collect();
        self.perms_filter = perms_filter.split(',').map(|s| s.to_string()).collect();

        let mut new_web_dav = IrodsWebDav::default();

        // We're okay with unwrap here since it's user input and panicking is fine.
        new_web_dav.ask_for_info(theme, self.external_host.as_ref().unwrap())?;

        self.web_dav = Some(new_web_dav);

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let data_transfer_image = Input::<String>::with_theme(theme)
            .with_prompt("Jobs Data Transfer Image")
            .default("harbor.cyverse.org/de/porklock".into())
            .interact()?;

        self.data_transfer_image = data_transfer_image;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let client_id = Input::<String>::with_theme(theme)
            .with_prompt("Keycloak VICE Client ID")
            .default("de-vice".into())
            .interact()?;

        let client_secret = Password::with_theme(theme)
            .with_prompt("Keycloak VICE Client Secret")
            .interact()?;

        self.client_id = client_id;
        self.client_secret = client_secret;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let server_uri = Input::<String>::with_theme(theme)
            .with_prompt("Keycloak Server URI")
            .interact()?;

        let realm = Input::<String>::with_theme(theme)
            .with_prompt("Keycloak Realm")
            .default("CyVerse".into())
            .interact()?;

        let client_id = Input::<String>::with_theme(theme)
            .with_prompt("Keycloak Client ID")
            .default("de".into())
            .interact()?;

        let client_secret = Password::with_theme(theme)
            .with_prompt("Keycloak Client Secret")
            .interact()?;

        self.server_uri = Url::parse(&server_uri).ok();
        self.realm = realm;
        self.client_id = client_id;
        self.client_secret = client_secret;

        self.vice.ask_for_info(theme)?;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let key_password = Password::with_theme(theme)
            .with_prompt("PGP Key Password")
            .interact()?;

        self.key_password = key_password;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID DataCite Base URI")
            .default("https://api.datacite.org/".into())
            .interact()?;

        let user = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID DataCite User")
            .interact()?;

        let password = Password::with_theme(theme)
            .with_prompt("Permanent ID DataCite Password")
            .interact()?;

        let doi_prefix = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID DataCite DOI Prefix")
            .interact()?;

        self.base_uri = Url::parse(&base_uri).ok();
        self.user = user;
        self.password = password;
        self.doi_prefix = doi_prefix;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let curators_group = Input::<String>::with_theme(theme)
            .with_prompt("Permanent ID Curators Group")
            .default("data-curators".into())
            .interact()?;

        self.curators_group = curators_group;

        self.data_cite.ask_for_info(theme)?;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let base_url = Input::<String>::with_theme(theme)
            .with_prompt("Unleash Base URL")
            .default("http://unleash:4242".into())
            .interact()?;

        let api_path = Input::<String>::with_theme(theme)
            .with_prompt("Unleash API Path")
            .default("/api".into())
            .interact()?;

        let maintenance_flag = Input::<String>::with_theme(theme)
            .with_prompt("Unleash Maintenance Flag")
            .default("DE-Maintenance".into())
            .interact()?;

        let api_token = Password::with_theme(theme)
            .with_prompt("Unleash API Token")
            .interact()?;

        self.base_url = Url::parse(&base_url).ok();
        self.api_path = Some(api_path);
        self.maintenance_flag = Some(maintenance_flag);
        self.api_token = api_token;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let base_uri = Input::<String>::with_theme(theme)
            .with_prompt("User Portal Base URI")
            .interact()?;

        self.base_uri = Some(base_uri);

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let image = Input::<String>::with_theme(theme)
            .with_prompt("Vice File Transfers Image")
            .default("harbor.cyverse.org/de/vice-file-transfers".into())
            .interact()?;

        let tag = Input::<String>::with_theme(theme)
            .with_prompt("Vice File Transfers Tag")
            .default("latest".into())
            .interact()?;

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
    fn merge(&self, right: &ViceDefaultBackend) -> anyhow::Result<ViceDefaultBackend> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let loading_page_template_string = Input::<String>::with_theme(theme)
            .with_prompt("Vice Default Backend Loading Page Template String")
            .interact()?;

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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
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
        new_file_transfers.ask_for_info(theme)?;
        self.file_transfers = Some(new_file_transfers);

        self.default_backend.ask_for_info(theme)?;
        self.base_uri = Url::parse(&base_uri).ok();
        self.image_pull_secret = Some(image_pull_secret);
        self.image_cache = Some(image_cache.split(',').map(|s| s.to_string()).collect());
        self.default_cas_url = Some(default_cas_url);
        self.default_cas_validate = Some(default_cas_validate);
        self.use_case_chars_min = Some(use_case_chars_min);
        self.use_csi_driver = Some(use_csi_data == 0);

        Ok(())
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

    fn ask_for_info(
        &mut self,
        theme: &ColorfulTheme,
        prefix: &str,
        name: &str,
        host: &str,
        user: &str,
        pass: &str,
    ) -> anyhow::Result<()> {
        let user = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} Database User", prefix))
            .default(user.to_string())
            .interact()?;

        let password = Input::with_theme(theme)
            .with_prompt(format!("{} Database Password", prefix))
            .default(pass.to_string())
            .interact()?;

        let host = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} Database Host", prefix))
            .default(host.to_string())
            .interact()?;

        let port = Input::<u32>::with_theme(theme)
            .with_prompt(format!("{} Database Port", prefix))
            .default(5432)
            .interact()?;

        let name = Input::<String>::with_theme(theme)
            .with_prompt(format!("{} Database Name", prefix))
            .default(name.to_string())
            .interact()?;

        self.user = user;
        self.password = password;
        self.host = host;
        self.port = port;
        self.name = name;

        Ok(())
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

    fn ask_for_info(
        &mut self,
        theme: &ColorfulTheme,
        name: &str,
        host: &str,
        user: &str,
        pass: &str,
    ) -> anyhow::Result<()> {
        let user = Input::<String>::with_theme(theme)
            .with_prompt("QMS Database User")
            .default(user.to_string())
            .interact()?;

        let password = Input::with_theme(theme)
            .with_prompt("QMS Database Password")
            .default(pass.to_string())
            .interact()?;

        let host = Input::<String>::with_theme(theme)
            .with_prompt("QMS Database Host")
            .default(host.to_string())
            .interact()?;

        let port = Input::<u32>::with_theme(theme)
            .with_prompt("QMS Database Port")
            .default(5432)
            .interact()?;

        let name = Input::<String>::with_theme(theme)
            .with_prompt("QMS Database Name")
            .default(name.to_string())
            .interact()?;

        let automigrate = Select::with_theme(theme)
            .with_prompt("QMS Database Automigrate")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        let reinitialize = Select::with_theme(theme)
            .with_prompt("QMS Database Reinitialize")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        self.user = user;
        self.password = password;
        self.host = host;
        self.port = Some(port);
        self.name = Some(name);
        self.automigrate = Some(automigrate == 0);
        self.reinitialize = Some(reinitialize == 0);

        Ok(())
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
            groups: Some(String::from("de_admins")),
            attribute: Some(String::from("entitlement")),
        }
    }
}

impl Admin {
    fn merge(&self, right: &Admin) -> anyhow::Result<Admin> {
        Ok(serde_merge::omerge(&self, &right)?)
    }

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let groups = Input::<String>::with_theme(theme)
            .with_prompt("Admin Groups")
            .default("de_admins".into())
            .interact()?;

        let attribute = Input::<String>::with_theme(theme)
            .with_prompt("Admin Attribute")
            .default("entitlement".into())
            .interact()?;

        self.groups = Some(groups);
        self.attribute = Some(attribute);

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let enabled = Select::with_theme(theme)
            .with_prompt("Analytics Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        let id = Input::<String>::with_theme(theme)
            .with_prompt("Analytics ID")
            .default("g-id".into())
            .interact()?;

        self.enabled = Some(enabled == 0);
        self.id = Some(id);

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let url = Input::<String>::with_theme(theme)
            .with_prompt("Harbor URL")
            .default("harbor.cyverse.org".into())
            .interact()?;

        let project_qa_robot_name = Input::<String>::with_theme(theme)
            .with_prompt("Harbor Project QA Robot Name")
            .interact()?;

        let project_qa_robot_secret = Password::with_theme(theme)
            .with_prompt("Harbor Project QA Robot Secret")
            .interact()?;

        self.url = Some(url);
        self.project_qa_robot_name = project_qa_robot_name;
        self.project_qa_robot_secret = project_qa_robot_secret;

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let enabled = Select::with_theme(theme)
            .with_prompt("QMS Enabled")
            .default(0)
            .items(&["Yes", "No"])
            .interact()?;

        self.enabled = Some(enabled == 0);

        Ok(())
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

    fn ask_for_info(&mut self, theme: &ColorfulTheme) -> anyhow::Result<()> {
        let endpoint = Input::<String>::with_theme(theme)
            .with_prompt("Jaeger Endpoint")
            .default("http://jaeger-collector.jaeger.svc.cluster.local:14250".into())
            .interact()?;

        let http_endpoint = Input::<String>::with_theme(theme)
            .with_prompt("Jaeger HTTP Endpoint")
            .default("http://jaeger-collector.jaeger.svc.cluster.local:14268/api/traces".into())
            .interact()?;

        self.endpoint = Url::parse(&endpoint).ok();
        self.http_endpoint = Url::parse(&http_endpoint).ok();

        Ok(())
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
            let mut new_intercom = Intercom::default();
            new_intercom.ask_for_info(&theme)?;
            self.intercom = Some(new_intercom);
        }

        let mut new_jobs = Jobs::default();
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
            let mut new_permanent_id = PermanentId::default();
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
            let mut new_unleash = Unleash::default();
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
            let mut new_qms = Qms::default();
            new_qms.ask_for_info(&theme)?;
            self.qms = Some(new_qms);
        }

        self.user_portal.ask_for_info(&theme)?;
        self.vice.ask_for_info(&theme)?;

        let mut new_admin = Admin::default();
        new_admin.ask_for_info(&theme)?;
        self.admin = Some(new_admin);

        let analytics_enabled = Select::with_theme(&theme)
            .with_prompt("Include Analytics?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if analytics_enabled == 0 {
            let mut new_analytics = Analytics::default();
            new_analytics.ask_for_info(&theme)?;
            self.analytics = Some(new_analytics);
        }

        let mut new_harbor = Harbor::default();
        new_harbor.ask_for_info(&theme)?;
        self.harbor = Some(new_harbor);

        let jaeger_enabled = Select::with_theme(&theme)
            .with_prompt("Include Jaeger?")
            .default(1)
            .items(&["Yes", "No"])
            .interact()?;

        if jaeger_enabled == 0 {
            let mut new_jaeger = Jaeger::default();
            new_jaeger.ask_for_info(&theme)?;
            self.jaeger = Some(new_jaeger);
        }

        Ok(())
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
