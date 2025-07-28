use grafbase_sdk::test::{GraphqlSubgraph, TestGateway};

#[tokio::test]
async fn exlucded_tags() {
    let gateway = TestGateway::builder()
        .toml_config(
            r#"
            [graph]
            introspection = true
            contracts.default_key = "{\"excludedTags\": [\"internal\"]}"
            "#,
        )
        .subgraph(
            GraphqlSubgraph::with_schema(
                r#"
                extend schema @link(url: "<self>", import: ["@tag"])

                type Query {
                    public: ID! @tag(name: "public")
                    private: ID! @tag(name: "internal")
                }
            "#,
            )
            .with_resolver("Query", "public", "public")
            .with_resolver("Query", "private", "private"),
        )
        .build()
        .await
        .unwrap();

    let sdl = gateway.introspect().send().await;

    insta::assert_snapshot!(sdl, @r"
    type Query {
      public: ID!
    }
    ");

    let response = gateway.query(r#"query { public }"#).send().await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "public": "public"
      }
    }
    "#);

    let response = gateway.query(r#"query { private }"#).send().await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Query does not have a field named 'private'.",
          "locations": [
            {
              "line": 1,
              "column": 9
            }
          ],
          "extensions": {
            "code": "OPERATION_VALIDATION_ERROR"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn included_and_exlucded_tags() {
    let gateway = TestGateway::builder()
        .toml_config(
            r#"
            [graph]
            introspection = true
            contracts.default_key = "{\"includedTags\": [\"public\"], \"excludedTags\": [\"secret\"]}"
            "#,
        )
        .subgraph(
            r#"
            extend schema @link(url: "<self>", import: ["@tag"])

            type Query @tag(name: "public") {
                user: User
            }

            type User {
                id: ID! @tag(name: "public") @tag(name: "basic")
                name: String! @tag(name: "secret") @tag(name: "internal")
                email: String! @tag(name: "public") @tag(name: "secret")
            }
            "#,
        )
        .build()
        .await
        .unwrap();

    let sdl = gateway.introspect().send().await;

    insta::assert_snapshot!(sdl, @r#"
    type Query {
      user: User
    }

    type User {
      id: ID!
    }
    "#);
}

#[tokio::test]
async fn included_tags() {
    let gateway = TestGateway::builder()
        .toml_config(
            r#"
            [graph]
            introspection = true
            contracts.default_key = "{\"includedTags\": [\"public\"]}"
            "#,
        )
        .subgraph(
            r#"
            extend schema @link(url: "<self>", import: ["@tag"])
            type Query {
                user: User @tag(name: "public")
                product: Product @tag(name: "internal")
            }

            type User {
                id: ID! @tag(name: "public")
                name: String!
                email: String!
            }

            type Product {
                id: ID!
                name: String!
                price: Float!
            }
            "#,
        )
        .build()
        .await
        .unwrap();

    let sdl = gateway.introspect().send().await;

    insta::assert_snapshot!(sdl, @r#"
    type Query {
      user: User
    }

    type User {
      id: ID!
    }
    "#);
}

#[tokio::test]
async fn complex_unreachable_scenario() {
    let gateway = TestGateway::builder()
        .toml_config(
            r#"
            [graph]
            introspection = true
            contracts.default_key = "{\"excludedTags\": [\"internal\"]}"

            [extensions.tag.config]
            hide_unreachable_types = true
            "#,
        )
        .subgraph(
            r#"
            extend schema @link(url: "<self>", import: ["@tag"])

            type Query {
                publicData: PublicType
                blockedData: BlockedType @tag(name: "internal")
            }

            type PublicType {
                id: ID!
                reachableField: ReachableType
            }

            type BlockedType {
                id: ID!
                data: String!
            }

            type ReachableType {
                value: String!
                nested: NestedReachable
            }

            type NestedReachable {
                data: String!
            }

            type UnreachableType {
                field: String!
                connection: AnotherUnreachable
            }

            type AnotherUnreachable {
                value: Int!
            }

            interface UnreachableInterface {
                id: ID!
            }

            union UnreachableUnion = UnreachableType | AnotherUnreachable

            enum UnreachableEnum {
                VALUE1
                VALUE2
            }

            input UnreachableInput {
                field: String
            }

            scalar UnreachableScalar
            "#,
        )
        .build()
        .await
        .unwrap();

    let sdl = gateway.introspect().send().await;
    insta::assert_snapshot!(sdl, @r#"
    type NestedReachable {
      data: String!
    }

    type PublicType {
      id: ID!
      reachableField: ReachableType
    }

    type Query {
      publicData: PublicType
    }

    type ReachableType {
      nested: NestedReachable
      value: String!
    }
    "#);
}
