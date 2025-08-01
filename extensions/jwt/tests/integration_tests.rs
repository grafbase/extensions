mod hydra;
mod oauth_metadata_tests;
mod unauthenticated_headers_tests;

use std::collections::HashMap;

use grafbase_sdk::test::{GraphqlSubgraph, TestGateway, TestGatewayBuilder};
use hydra::{AUDIENCE, CoreClientExt, JWKS_URI, OTHER_AUDIENCE, OryHydraOpenIDProvider, THIRD_AUDIENCE};
use indoc::formatdoc;

fn gateway_builder() -> TestGatewayBuilder {
    TestGateway::builder()
        .subgraph(
            GraphqlSubgraph::with_schema(r#"type Query { hi: String }"#).with_resolver(
                "Query",
                "hi",
                String::from("hello"),
            ),
        )
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"
        "#})
}

#[allow(clippy::panic)]
fn tamper_jwt(token: String) -> String {
    use base64::{Engine as _, engine::general_purpose};
    #[allow(clippy::panic)]
    let [header, payload, signature] = token.split('.').collect::<Vec<_>>()[..] else {
        panic!("Invalid JWT");
    };
    let mut payload = serde_json::from_slice::<HashMap<String, serde_json::Value>>(
        &general_purpose::URL_SAFE_NO_PAD.decode(payload).unwrap(),
    )
    .unwrap();
    payload.insert("sub".to_string(), serde_json::Value::String("evil admin".to_string()));
    let payload = general_purpose::URL_SAFE_NO_PAD.encode(serde_json::to_vec(&header).unwrap());
    let new_token = format!("{}.{}.{}", header, payload, signature);

    // Sanity check
    assert!(new_token != token);
    new_token
}

#[tokio::test]
async fn without_token() {
    let gateway = gateway_builder().build().await.unwrap();

    let response = gateway.query("query { hi }").send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Unauthorized",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn with_invalid_token() {
    let gateway = gateway_builder().build().await.unwrap();

    let response = gateway
        .query("query { hi }")
        .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c")
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Unauthorized",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn with_valid_token() {
    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[])
        .await;

    let gateway = gateway_builder().build().await.unwrap();

    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "hi": "hello"
      }
    }
    "#);
}

#[tokio::test]
async fn test_different_header_location() {
    let gateway = gateway_builder()
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"
            header_name = "X-My-JWT"
            header_value_prefix = "Bearer2 "
        "#})
        .build()
        .await
        .unwrap();

    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[])
        .await;

    let response = gateway
        .query("query { hi }")
        .header("X-My-JWT", &format!("Bearer2 {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "hi": "hello"
      }
    }
    "#);
}

#[tokio::test]
async fn test_cookie_name_location() {
    let gateway = gateway_builder()
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"
            cookie_name = "my_jwt"
        "#})
        .build()
        .await
        .unwrap();

    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[])
        .await;

    let response = gateway
        .query("query { hi }")
        .header(
            "Cookie",
            &format!("name=value; name2=value2; my_jwt={token}; name3=value3"),
        )
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "hi": "hello"
      }
    }
    "#);

    let response = gateway
        .query("query { hi }")
        .header("Cookie", "name=value; name2=value2; name3=value3")
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Unauthorized",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn test_tampered_jwt() {
    let gateway = gateway_builder().build().await.unwrap();

    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[])
        .await;

    let token = tamper_jwt(token);

    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Unauthorized",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn test_wrong_provider() {
    let gateway = gateway_builder().build().await.unwrap();

    let token = OryHydraOpenIDProvider::second_provider()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[])
        .await;

    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Unauthorized",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn test_single_audience() {
    let gateway = gateway_builder()
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"
            audience = "{AUDIENCE}"
        "#})
        .build()
        .await
        .unwrap();

    // -- CORRECT AUDIENCE --
    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[("audience", AUDIENCE)])
        .await;
    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "hi": "hello"
      }
    }
    "#);

    // -- MULTIPLE AUDIENCES --
    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[("audience", &format!("{AUDIENCE} {OTHER_AUDIENCE}"))])
        .await;
    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "hi": "hello"
      }
    }
    "#);

    // -- INCORRECT AUDIENCE --
    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[("audience", OTHER_AUDIENCE)])
        .await;
    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Unauthorized",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);

    // -- MISSING AUDIENCE --
    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[])
        .await;
    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Unauthorized",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn test_multiple_audience() {
    let gateway = gateway_builder()
        .toml_config(formatdoc! {r#"
            [extensions.jwt.config]
            url = "{JWKS_URI}"
            audience = ["{AUDIENCE}", "{OTHER_AUDIENCE}"]
        "#})
        .build()
        .await
        .unwrap();

    // -- CORRECT AUDIENCE --
    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[("audience", AUDIENCE)])
        .await;
    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "hi": "hello"
      }
    }
    "#);

    // -- MULTIPLE AUDIENCES --
    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[("audience", &format!("{AUDIENCE} {OTHER_AUDIENCE}"))])
        .await;
    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "hi": "hello"
      }
    }
    "#);

    // -- OTHER AUDIENCE --
    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[("audience", OTHER_AUDIENCE)])
        .await;
    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "hi": "hello"
      }
    }
    "#);

    // -- INCORRECT AUDIENCE --
    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[("audience", THIRD_AUDIENCE)])
        .await;
    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Unauthorized",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);

    // -- MISSING AUDIENCE --
    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[])
        .await;
    let response = gateway
        .query("query { hi }")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Unauthorized",
          "extensions": {
            "code": "UNAUTHENTICATED"
          }
        }
      ]
    }
    "#);
}
