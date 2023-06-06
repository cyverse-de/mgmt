use serde::{Deserialize, Serialize};
use url::Url;

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
    pub fn merge(&self, right: &BaseURLs) -> anyhow::Result<BaseURLs> {
        Ok(serde_merge::omerge(&self, &right)?)
    }
}
