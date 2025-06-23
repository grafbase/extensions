mod hydra;

use grafbase_sdk::test::{GraphqlSubgraph, TestGateway};
use hydra::{ADMIN_SCOPE, CoreClientExt as _, JWKS_URI, OryHydraOpenIDProvider, READ_SCOPE, WRITE_SCOPE};
use indoc::{formatdoc, indoc};

async fn setup(scopes: Option<&str>) -> (TestGateway, String) {
    let schema = formatdoc! {r#"
        extend schema
            @link(url: "<self>", import: ["@requiresScopes"])

        type Query {{
            public: String
            hasReadScope: String @requiresScopes(scopes: "read")
            hasReadAndWriteScope: String @requiresScopes(scopes: [["read", "write"]])
            hasReadOrWriteScope: String @requiresScopes(scopes: [["read"], ["write"]])
        }}
    "#};

    // Create a subgraph with a single field
    let subgraph = GraphqlSubgraph::with_schema(schema)
        .with_resolver("Query", "public", String::from("public"))
        .with_resolver("Query", "hasReadScope", String::from("Has read scope"))
        .with_resolver(
            "Query",
            "hasReadAndWriteScope",
            String::from("Has read and write scope"),
        )
        .with_resolver("Query", "hasReadOrWriteScope", String::from("Has read or write scope"));

    let config = formatdoc! {r#"
        [[authentication.providers]]

        [authentication.providers.jwt]
        name = "my-jwt"

        [authentication.providers.jwt.jwks]
        url = "{JWKS_URI}"

        [[authentication.providers]]

        [authentication.providers.anonymous]
    "#};

    let gateway = TestGateway::builder()
        .subgraph(subgraph)
        .toml_config(config)
        .build()
        .await
        .unwrap();

    let extra_params = if let Some(scopes) = scopes {
        vec![("scope", scopes)]
    } else {
        vec![]
    };

    let token = OryHydraOpenIDProvider::default()
        .create_client()
        .await
        .get_access_token_with_client_credentials(&extra_params)
        .await;

    (gateway, token)
}

const QUERY: &str = indoc! {r#"
    query {
        public
        hasReadScope
        hasReadAndWriteScope
        hasReadOrWriteScope
    }
"#};

#[tokio::test]
async fn anonymous_token() {
    let (gateway, _) = setup(None).await;

    let response = gateway.query(QUERY).send().await;

    // The result is compared against a snapshot.
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "public": "public",
        "hasReadScope": null,
        "hasReadAndWriteScope": null,
        "hasReadOrWriteScope": null
      },
      "errors": [
        {
          "message": "Not authorized",
          "locations": [
            {
              "line": 3,
              "column": 5
            }
          ],
          "path": [
            "hasReadScope"
          ],
          "extensions": {
            "code": "UNAUTHORIZED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn token_without_scopes() {
    let (gateway, token) = setup(None).await;

    let response = gateway
        .query(QUERY)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;

    // The result is compared against a snapshot.
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "public": "public",
        "hasReadScope": null,
        "hasReadAndWriteScope": null,
        "hasReadOrWriteScope": null
      },
      "errors": [
        {
          "message": "Not authorized: insufficient scopes",
          "locations": [
            {
              "line": 3,
              "column": 5
            }
          ],
          "path": [
            "hasReadScope"
          ],
          "extensions": {
            "code": "UNAUTHORIZED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn token_with_insufficient_scopes() {
    let (gateway, token) = setup(Some(ADMIN_SCOPE)).await;

    let response = gateway
        .query(QUERY)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;

    // The result is compared against a snapshot.
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "public": "public",
        "hasReadScope": null,
        "hasReadAndWriteScope": null,
        "hasReadOrWriteScope": null
      },
      "errors": [
        {
          "message": "Not authorized: insufficient scopes",
          "locations": [
            {
              "line": 3,
              "column": 5
            }
          ],
          "path": [
            "hasReadScope"
          ],
          "extensions": {
            "code": "UNAUTHORIZED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn token_with_read_scope() {
    let (gateway, token) = setup(Some(READ_SCOPE)).await;

    let response = gateway
        .query(QUERY)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;

    // The result is compared against a snapshot.
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "public": "public",
        "hasReadScope": "Has read scope",
        "hasReadAndWriteScope": null,
        "hasReadOrWriteScope": "Has read or write scope"
      },
      "errors": [
        {
          "message": "Not authorized: insufficient scopes",
          "locations": [
            {
              "line": 4,
              "column": 5
            }
          ],
          "path": [
            "hasReadAndWriteScope"
          ],
          "extensions": {
            "code": "UNAUTHORIZED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn token_with_write_scope() {
    let (gateway, token) = setup(Some(WRITE_SCOPE)).await;

    let response = gateway
        .query(QUERY)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;

    // The result is compared against a snapshot.
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "public": "public",
        "hasReadScope": null,
        "hasReadAndWriteScope": null,
        "hasReadOrWriteScope": "Has read or write scope"
      },
      "errors": [
        {
          "message": "Not authorized: insufficient scopes",
          "locations": [
            {
              "line": 3,
              "column": 5
            }
          ],
          "path": [
            "hasReadScope"
          ],
          "extensions": {
            "code": "UNAUTHORIZED"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn token_with_read_and_write_scopes() {
    let (gateway, token) = setup(Some(&format!("{READ_SCOPE} {WRITE_SCOPE} {ADMIN_SCOPE}"))).await;

    let response = gateway
        .query(QUERY)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await;

    // The result is compared against a snapshot.
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "public": "public",
        "hasReadScope": "Has read scope",
        "hasReadAndWriteScope": "Has read and write scope",
        "hasReadOrWriteScope": "Has read or write scope"
      }
    }
    "#);
}
