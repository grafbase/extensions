use super::*;
use reqwest::StatusCode;

#[tokio::test]
async fn test_unauthenticated_headers() {
    let gateway = gateway_builder()
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"

            [[extensions.jwt.config.unauthenticated_headers]]
            name = "Www-Authenticate"
            value = "Bearer realm=\"hydra\", error=\"invalid_token\", error_description=\"The token is invalid\""
        "#})
        .build()
        .await
        .unwrap();

    let response = gateway.query("query { hi }").send().await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let header = response
        .headers()
        .get("www-authenticate")
        .expect("WWW-Authenticate header should be returned");

    assert_eq!(
        header,
        "Bearer realm=\"hydra\", error=\"invalid_token\", error_description=\"The token is invalid\""
    )
}
