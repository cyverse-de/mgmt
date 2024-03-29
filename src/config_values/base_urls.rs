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
                id: 0,
                section: section.clone(),
                key: "Analyses".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.app_exposer {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "AppExposer".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.apps {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Apps".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.async_tasks {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "AsyncTasks".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.dashboard_aggregator {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DashboardAggregator".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.data_info {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "DataInfo".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.grouper_web_services {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "GrouperWebServices".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.iplant_email {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "IplantEmail".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.iplant_groups {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "IplantGroups".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.jex_adapter {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "JexAdapter".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.job_status_listener {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "JobStatusListener".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.metadata {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Metadata".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.notifications {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Notifications".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.permissions {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Permissions".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.qms {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "QMS".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.requests {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Requests".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.search {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Search".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.terrain {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "Terrain".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
            });
        }

        if let Some(url) = base_urls.user_info {
            vec.push(db::ConfigurationValue {
                id: 0,
                section: section.clone(),
                key: "UserInfo".to_string(),
                value: url.to_string(),
                value_type: "string".to_string(),
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
        let key = cfg.key.clone();
        let value = cfg.value.clone();

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

        Ok(())
    }
}

impl Default for BaseURLs {
    fn default() -> Self {
        BaseURLs {
            section: "BaseURLs".to_string(),
            analyses: None,
            app_exposer: None,
            apps: None,
            async_tasks: None,
            dashboard_aggregator: None,
            data_info: None,
            grouper_web_services: None,
            iplant_email: None,
            iplant_groups: None,
            jex_adapter: None,
            job_status_listener: None,
            metadata: None,
            notifications: None,
            permissions: None,
            qms: None,
            requests: None,
            search: None,
            terrain: None,
            user_info: None,
        }
    }
}
