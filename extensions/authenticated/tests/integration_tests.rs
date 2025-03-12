mod hydra;

use grafbase_sdk::test::{DynamicSchema, TestConfig, TestRunner};
use hydra::{CoreClientExt as _, JWKS_URI, OryHydraOpenIDProvider};
use indoc::formatdoc;

#[tokio::test]
async fn test_authenticated() {
    let extension_path = std::env::current_dir().unwrap().join("build");
    let path_str = format!("file://{}", extension_path.display());

    let schema = formatdoc! {r#"
        extend schema
            @link(url: "{path_str}", import: ["@authenticated"])

        type Query {{
            public: String
            private: String @authenticated
        }}
    "#};

    // Create a subgraph with a single field
    let subgraph = DynamicSchema::builder(schema)
        .with_resolver("Query", "public", String::from("public"))
        .with_resolver("Query", "private", String::from("private"))
        .into_subgraph("test")
        .unwrap();

    let config = format!(
        r#"
        [[authentication.providers]]

        [authentication.providers.jwt]
        name = "my-jwt"

        [authentication.providers.jwt.jwks]
        url = "{JWKS_URI}"

        [[authentication.providers]]

        [authentication.providers.anonymous]
        "#
    );

    let config = TestConfig::builder().with_subgraph(subgraph).build(config).unwrap();
    let runner = TestRunner::new(config).await.unwrap();

    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&[])
        .await;

    let result: serde_json::Value = runner
        .graphql_query(r#"query { public private }"#)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "public": "public",
        "private": null
      },
      "errors": [
        {
          "message": "Not authenticated",
          "locations": [
            {
              "line": 1,
              "column": 16
            }
          ],
          "path": [
            "private"
          ],
          "extensions": {
            "code": "UNAUTHORIZED"
          }
        }
      ]
    }
    "#);

    let result: serde_json::Value = runner
        .graphql_query(r#"query { public private }"#)
        .with_header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "public": "public",
        "private": "private"
      }
    }
    "#);
}
