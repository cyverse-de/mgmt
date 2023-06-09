pub mod agave;
pub mod amqp;
pub mod base_urls;
pub mod config;
pub mod dashboard_aggregator;
pub mod db;
pub mod de;
pub mod docker;
pub mod elasticsearch;
pub mod email;
pub mod grouper;
pub mod icat;
pub mod infosquito;
pub mod intercom;
pub mod irods;
pub mod keycloak;
pub mod misc;
pub mod qa;
pub mod vice;

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
