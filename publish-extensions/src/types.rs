use semver::Version;

#[derive(serde::Serialize)]
pub struct GraphQLRequest {
    pub operation_name: String,
    pub query: String,
    pub variables: serde_json::Value,
}

#[derive(serde::Deserialize, Debug)]
pub struct GraphQLResponse {
    pub data: ResponseData,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub extension_by_name: Option<ExtensionData>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionData {
    pub highest_version: VersionInfo,
}

#[derive(serde::Deserialize, Debug)]
pub struct VersionInfo {
    pub version: Version,
}

#[derive(Debug, serde::Deserialize)]
pub struct ExtensionToml {
    pub extension: Extension,
}

#[derive(Debug, serde::Deserialize)]
pub struct Extension {
    pub name: String,
    pub version: Version,
}
