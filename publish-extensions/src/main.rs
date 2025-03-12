mod types;

use indoc::formatdoc;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use semver::Version;
use std::{env, fs};
use types::{GraphQLRequest, GraphQLResponse};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_token = env::var("GRAFBASE_API_TOKEN").unwrap();

    for entry in fs::read_dir("./extensions")? {
        let Ok(entry) = entry else {
            continue;
        };

        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let extension_toml = fs::read_to_string(path.join("extension.toml"))?;
        let extension_toml: types::ExtensionToml = toml::from_str(&extension_toml)?;
        let extension = extension_toml.extension;

        match fetch_extension_version(&extension.name, &api_token).await? {
            Some(version) if version == extension.version => continue,
            _ => {}
        }

        let args = vec!["extension", "build"];
        duct::cmd("grafbase", args).dir(&path).run()?;

        let args = vec!["extension", "publish"];

        duct::cmd("grafbase", args)
            .dir(&path)
            .env("GRAFBASE_API_TOKEN", &api_token)
            .run()?;
    }

    Ok(())
}

async fn fetch_extension_version(name: &str, api_token: &str) -> anyhow::Result<Option<Version>> {
    let client = reqwest::Client::new();

    // Prepare headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_token))?);

    let query = formatdoc! {r#"
        query ExtensionByName {{
            extensionByName(name: "{name}") {{
                highestVersion {{
                    version
                }}
            }}
        }}
    "#};

    // Prepare GraphQL query
    let request = GraphQLRequest {
        operation_name: "ExtensionByName".to_string(),
        query,
        variables: serde_json::json!({}),
    };

    // Send request
    let response = client
        .post("https://api.grafbase.com/graphql")
        .headers(headers)
        .json(&request)
        .send()
        .await?;

    // Parse response
    let result: GraphQLResponse = response.json().await?;

    // Return the version if available
    Ok(result.data.extension_by_name.map(|ext| ext.highest_version.version))
}
