use duration_str::deserialize_duration;
use std::time::Duration;
use url::Url;

#[derive(Debug)]
pub(crate) struct Config {
    pub url: Url,
    pub poll_interval: Duration,
    pub issuer: Option<String>,
    pub audience: Option<Vec<String>>,
    pub locations: Vec<Location>,
    pub oauth: Option<OAuthConfig>,
    pub unauthenticated_headers: Vec<StaticHeader>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OAuthConfig {
    pub protected_resource: oauth_protected_resource_shared::OAuthConfig,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StaticHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub enum Location {
    Header { name: String, value_prefix: Option<String> },
    Cookie { name: String },
}

impl<'de> serde::Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let TomlConfig {
            url,
            issuer,
            audience,
            poll_interval,
            header_name,
            header_value_prefix,
            cookie_name,
            oauth,
            unauthenticated_headers,
        } = TomlConfig::deserialize(deserializer)?;
        let mut locations = Vec::new();
        if let Some(header_name) = header_name {
            locations.push(Location::Header {
                name: header_name,
                value_prefix: header_value_prefix,
            });
        }
        if let Some(cookie_name) = cookie_name {
            locations.push(Location::Cookie { name: cookie_name });
        }
        if locations.is_empty() {
            locations.push(Location::Header {
                name: "Authorization".into(),
                value_prefix: Some("Bearer ".into()),
            })
        }
        Ok(Config {
            url,
            poll_interval,
            issuer,
            audience,
            locations,
            oauth,
            unauthenticated_headers,
        })
    }
}

#[serde_with::serde_as]
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct TomlConfig {
    url: Url,
    issuer: Option<String>,
    #[serde_as(deserialize_as = "Option<serde_with::OneOrMany<_>>")]
    audience: Option<Vec<String>>,
    #[serde(default = "default_poll_interval", deserialize_with = "deserialize_duration")]
    poll_interval: Duration,
    header_name: Option<String>,
    header_value_prefix: Option<String>,
    cookie_name: Option<String>,
    #[serde(default)]
    oauth: Option<OAuthConfig>,
    #[serde(default)]
    unauthenticated_headers: Vec<StaticHeader>,
}

fn default_poll_interval() -> Duration {
    Duration::from_secs(60)
}
