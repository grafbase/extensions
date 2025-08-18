use crate::hydra::CoreClientExt as _;

use super::hydra::{self, JWKS_URI};
use grafbase_sdk::test::{GraphqlSubgraph, TestGateway, TestGatewayBuilder};
use indoc::formatdoc;
use reqwest::Client;

fn gateway_builder() -> TestGatewayBuilder {
    TestGateway::builder().subgraph(
        GraphqlSubgraph::with_schema(r#"type Query { hi: String }"#).with_resolver(
            "Query",
            "hi",
            String::from("hello"),
        ),
    )
}

#[tokio::test]
async fn test_just_resource() {
    let gateway = gateway_builder()
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"

            [extensions.jwt.config.oauth.protected_resource.metadata]
            resource = "https://protected.example.com"
        "#})
        .build()
        .await
        .unwrap();

    let mut endpoint = gateway.url().clone();
    endpoint.set_path("/.well-known/oauth-protected-resource");

    let client = Client::new();
    let response = client.get(endpoint).send().await.unwrap();

    assert!(
        response.status().is_success(),
        "Expected status code 200, got {:?}",
        response.status()
    );

    assert_eq!(
        response.headers().get("content-type"),
        Some(&"application/json".parse().unwrap())
    );

    let response: serde_json::Value = response.json().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "jwks_uri": "http://127.0.0.1:4444/.well-known/jwks.json",
      "resource": "https://protected.example.com"
    }
    "#);

    // Ensure the JWT auth still works (rejects unauthenticated requests)
    let graphql_response = gateway.query("{ hi }").send().await;
    insta::assert_json_snapshot!(graphql_response, @r#"
    {
      "errors": [
        {
          "message": "Unauthenticated",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn test_custom_path() {
    let gateway = gateway_builder()
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"

            [extensions.jwt.config.oauth.protected_resource]
            metadata_path = "/custom-oauth-metadata"

            [extensions.jwt.config.oauth.protected_resource.metadata]
            resource = "https://protected.example.com"
        "#})
        .build()
        .await
        .unwrap();

    let mut endpoint = gateway.url().clone();
    endpoint.set_path("/custom-oauth-metadata");

    let client = Client::new();
    let response = client.get(endpoint).send().await.unwrap();

    assert!(
        response.status().is_success(),
        "Expected status code 200, got {:?}",
        response.status()
    );

    assert_eq!(
        response.headers().get("content-type"),
        Some(&"application/json".parse().unwrap())
    );

    let response: serde_json::Value = response.json().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "jwks_uri": "http://127.0.0.1:4444/.well-known/jwks.json",
      "resource": "https://protected.example.com"
    }
    "#);
}

#[tokio::test]
async fn test_jwks_uri_default() {
    // This test verifies that when no jwks_uri is specified in the oauth metadata,
    // it defaults to the main JWT extension's url
    let gateway = gateway_builder()
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"

            [extensions.jwt.config.oauth.protected_resource.metadata]
            resource = "https://protected.example.com"
        "#})
        .build()
        .await
        .unwrap();

    let mut endpoint = gateway.url().clone();
    endpoint.set_path("/.well-known/oauth-protected-resource");

    let client = Client::new();
    let response = client.get(endpoint).send().await.unwrap();
    let response: serde_json::Value = response.json().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "jwks_uri": "http://127.0.0.1:4444/.well-known/jwks.json",
      "resource": "https://protected.example.com"
    }
    "#);
}

#[tokio::test]
async fn test_all_metadata() {
    let gateway = gateway_builder()
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"
            [extensions.jwt.config.oauth.protected_resource.metadata]
            resource = "https://protected.example.com"
            authorization_servers = ["https://auth.example.com"]
            jwks_uri = "https://auth.example.com/.well-known/jwks.json"
            scopes_supported = ["read", "write"]
            bearer_methods_supported = ["header", "body"]
            resource_signing_alg_values_supported = ["RS256"]
            resource_name = "Protected Resource"
            resource_documentation = "https://example.com/docs/protected-resource"
            resource_policy_uri = "https://example.com/policy/protected-resource"
            resource_tos_uri = "https://example.com/tos/protected-resource"
            tls_client_certificate_bound_access_tokens = true
            authorization_details_types_supported = ["basic", "oauth"]
            dpop_signing_alg_values_supported = ["RS256"]
            dpop_bound_access_tokens_required = true
        "#})
        .build()
        .await
        .unwrap();

    let mut endpoint = gateway.url().clone();
    endpoint.set_path("/.well-known/oauth-protected-resource");

    let client = Client::new();
    let response = client.get(endpoint).send().await.unwrap();

    assert!(
        response.status().is_success(),
        "Expected status code 200, got {:?}",
        response.status()
    );

    assert_eq!(
        response.headers().get("content-type"),
        Some(&"application/json".parse().unwrap())
    );

    let response: serde_json::Value = response.json().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "authorization_details_types_supported": [
        "basic",
        "oauth"
      ],
      "authorization_servers": [
        "https://auth.example.com"
      ],
      "bearer_methods_supported": [
        "header",
        "body"
      ],
      "dpop_bound_access_tokens_required": true,
      "dpop_signing_alg_values_supported": [
        "RS256"
      ],
      "jwks_uri": "https://auth.example.com/.well-known/jwks.json",
      "resource": "https://protected.example.com",
      "resource_documentation": "https://example.com/docs/protected-resource",
      "resource_name": "Protected Resource",
      "resource_policy_uri": "https://example.com/policy/protected-resource",
      "resource_signing_alg_values_supported": [
        "RS256"
      ],
      "resource_tos_uri": "https://example.com/tos/protected-resource",
      "scopes_supported": [
        "read",
        "write"
      ],
      "tls_client_certificate_bound_access_tokens": true
    }
    "#);
}

#[tokio::test]
async fn test_metadata_with_authentication() {
    let gateway = gateway_builder()
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"

            [extensions.jwt.config.oauth.protected_resource.metadata]
            resource = "https://protected.example.com"
        "#})
        .build()
        .await
        .unwrap();

    // Get token from OryHydra provider
    let token = hydra::OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[])
        .await;

    // First, verify we can access the metadata endpoint without authentication
    let mut metadata_endpoint = gateway.url().clone();
    metadata_endpoint.set_path("/.well-known/oauth-protected-resource");

    let client = Client::new();
    let metadata_response = client.get(metadata_endpoint).send().await.unwrap();
    assert!(metadata_response.status().is_success());

    // Now verify that the GraphQL endpoint still requires authentication
    let unauthorized_response = gateway.query("{ hi }").send().await;
    insta::assert_json_snapshot!(unauthorized_response, @r#"
    {
      "errors": [
        {
          "message": "Unauthenticated",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);

    // And with a valid token, we can access the GraphQL endpoint
    let authorized_response = gateway
        .query("{ hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;

    insta::assert_json_snapshot!(authorized_response, @r#"
    {
      "data": {
        "hi": "hello"
      }
    }
    "#);
}
