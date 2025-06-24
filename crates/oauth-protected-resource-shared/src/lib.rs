use std::collections::BTreeMap;

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct OAuthConfig {
    /// https://datatracker.ietf.org/doc/html/rfc9728#name-obtaining-protected-resourc
    #[serde(default = "default_path")]
    pub path: String,
    pub metadata: Metadata,
}

fn default_path() -> String {
    "/.well-known/oauth-protected-resource".to_owned()
}

/// See https://datatracker.ietf.org/doc/html/rfc9728#section-2
#[derive(serde::Deserialize, Debug)]
pub struct Metadata {
    pub resource: String,

    #[serde(flatten)]
    pub other_parameters: BTreeMap<String, serde_json::Value>,
}
