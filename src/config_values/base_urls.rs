use serde::{Deserialize, Serialize};
use url::Url;

use crate::db::{self, LoadFromDatabase};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BaseURLs {
    #[serde(skip)]
    section: String,

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

impl From<BaseURLs> for Vec<db::ConfigurationValue> {
    fn from(base_urls: BaseURLs) -> Vec<db::ConfigurationValue> {
        let mut vec: Vec<db::ConfigurationValue> = Vec::new();
        let section: String;
        if base_urls.section.is_empty() {
            section = "BaseURLs".to_string();
        } else {
            section = base_urls.section.clone();
        }

        if let Some(url) = base_urls.analyses {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Analyses".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.app_exposer {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("AppExposer".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.apps {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Apps".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.async_tasks {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("AsyncTasks".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.dashboard_aggregator {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("DashboardAggregator".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.data_info {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("DataInfo".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.grouper_web_services {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("GrouperWebServices".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.iplant_email {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("IplantEmail".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.iplant_groups {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("IplantGroups".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.jex_adapter {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("JexAdapter".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.job_status_listener {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("JobStatusListener".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.metadata {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Metadata".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.notifications {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Notifications".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.permissions {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Permissions".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.qms {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("QMS".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.requests {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Requests".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.search {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Search".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.terrain {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("Terrain".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        if let Some(url) = base_urls.user_info {
            vec.push(db::ConfigurationValue {
                id: None,
                section: Some(section.clone()),
                key: Some("UserInfo".to_string()),
                value: Some(url.to_string()),
                value_type: Some("string".to_string()),
            });
        }

        vec
    }
}

impl LoadFromDatabase for BaseURLs {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Analyses" => self.analyses = Url::parse(&value).ok(),
                "AppExposer" => self.app_exposer = Url::parse(&value).ok(),
                "Apps" => self.apps = Url::parse(&value).ok(),
                "AsyncTasks" => self.async_tasks = Url::parse(&value).ok(),
                "DashboardAggregator" => self.dashboard_aggregator = Url::parse(&value).ok(),
                "DataInfo" => self.data_info = Url::parse(&value).ok(),
                "GrouperWebServices" => self.grouper_web_services = Url::parse(&value).ok(),
                "IplantEmail" => self.iplant_email = Url::parse(&value).ok(),
                "IplantGroups" => self.iplant_groups = Url::parse(&value).ok(),
                "JexAdapter" => self.jex_adapter = Url::parse(&value).ok(),
                "JobStatusListener" => self.job_status_listener = Url::parse(&value).ok(),
                "Metadata" => self.metadata = Url::parse(&value).ok(),
                "Notifications" => self.notifications = Url::parse(&value).ok(),
                "Permissions" => self.permissions = Url::parse(&value).ok(),
                "QMS" => self.qms = Url::parse(&value).ok(),
                "Requests" => self.requests = Url::parse(&value).ok(),
                "Search" => self.search = Url::parse(&value).ok(),
                "Terrain" => self.terrain = Url::parse(&value).ok(),
                "UserInfo" => self.user_info = Url::parse(&value).ok(),
                _ => (),
            }
        }

        Ok(())
    }
}

impl Default for BaseURLs {
    fn default() -> Self {
        BaseURLs {
            section: "BaseURLs".to_string(),
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
