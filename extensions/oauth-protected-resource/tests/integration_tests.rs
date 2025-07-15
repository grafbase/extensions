use grafbase_sdk::test::{GraphqlSubgraph, TestGateway};

#[tokio::test]
async fn test_just_resource() {
    let gateway = TestGateway::builder()
        .subgraph(
            GraphqlSubgraph::with_schema(
                r#"
                type Query {
                    hi: String
                }
                "#,
            )
            .with_resolver("Query", "hi", "Alice"),
        )
        .toml_config(
            r#"
            [extensions.oauth-protected-resource.config]
            metadata.resource = "https://protected.example.com"
        "#,
        )
        .build()
        .await
        .unwrap();

    let mut endpoint = gateway.url().clone();
    endpoint.set_path("/.well-known/oauth-protected-resource");

    let client = reqwest::Client::new();

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
      "resource": "https://protected.example.com"
    }
    "#);

    // Check that the requests passes through.
    let response = gateway.query("{ hi }").send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "hi": "Alice"
      }
    }
    "#);
}

#[tokio::test]
async fn custom_path() {
    let gateway = TestGateway::builder()
        .subgraph(
            GraphqlSubgraph::with_schema(
                r#"
                type Query {
                    hi: String
                }
                "#,
            )
            .with_resolver("Query", "hi", "Alice"),
        )
        .toml_config(
            r#"
            [extensions.oauth-protected-resource.config]
            metadata_path = "/yolo"
            metadata.resource = "https://protected.example.com"
        "#,
        )
        .build()
        .await
        .unwrap();

    let mut endpoint = gateway.url().clone();
    endpoint.set_path("/yolo");

    let client = reqwest::Client::new();

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
      "resource": "https://protected.example.com"
    }
    "#);
}

#[tokio::test]
async fn test_all_metadata() {
    let gateway = TestGateway::builder()
        .subgraph(
            GraphqlSubgraph::with_schema(
                r#"
                type Query {
                    hi: String
                }
                "#,
            )
            .with_resolver("Query", "hi", "Alice"),
        )
        .toml_config(
            r#"
            [extensions.oauth-protected-resource.config.metadata]
            resource = "https://protected.example.com"
            authorization_servers = ["https://auth.example.com"]
            jwks_uri = "https://auth.example.com/.well-known/jwks.json"
            scopes_supported = ["read", "write"]
            bearer_methods_supported = ["Bearer"]
            resource_signing_alg_values_supported = ["RS256"]
            resource_name = "Protected Resource"
            resource_documentation = "https://example.com/docs/protected-resource"
            resource_policy_uri = "https://example.com/policy/protected-resource"
            resource_tos_uri = "https://example.com/tos/protected-resource"
            tls_client_certificate_bound_access_tokens = true
            authorization_details_types_supported = ["basic", "oauth"]
            dpop_signing_alg_values_supported = ["RS256"]
            dpop_bound_access_tokens_required = true

            custom_metadata = { "abc" = "def" }
        "#,
        )
        .build()
        .await
        .unwrap();

    let mut endpoint = gateway.url().clone();
    endpoint.set_path("/.well-known/oauth-protected-resource");

    let client = reqwest::Client::new();

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
        "Bearer"
      ],
      "custom_metadata": {
        "abc": "def"
      },
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
