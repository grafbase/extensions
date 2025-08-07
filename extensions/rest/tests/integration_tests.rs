use grafbase_sdk::test::TestGateway;
use indoc::{formatdoc, indoc};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

#[derive(serde::Deserialize, serde::Serialize)]
struct Response {
    data: Option<serde_json::Value>,
    errors: Vec<Error>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Error {
    message: String,
    extensions: Option<serde_json::Value>,
}

fn subgraph(rest_endpoint: &str) -> String {
    subgraph_sdl(
        rest_endpoint,
        r#"
        type Query {
          users: [User!]! @rest(
            endpoint: "endpoint",
            http: { GET: "/users" },
            selection: "[.[] | { id, name, age }]"
          )

          user(id: Int!): User @rest(
            endpoint: "endpoint",
            http: { GET: "/users/{{ args.id }}" }
          )
        }

        type Mutation {
          createUser(input: UserInput!): User! @rest(
            endpoint: "endpoint",
            http: { POST: "/users" },
            selection: "{ id, name, age }"
          )

          createStaticUser: User! @rest(
            endpoint: "endpoint",
            http: {
                POST: "/users"
                body: { static: { name: "John Doe", age: 30 } }
            },
            selection: "{ id, name, age }"
          )

          updateUser(id: Int!, input: UserInput!): User! @rest(
            endpoint: "endpoint",
            http: { PUT: "/users/{{ args.id }}" },
            selection: "{ id, name, age }"
          )

          deleteUser(id: Int!): User! @rest(
            endpoint: "endpoint",
            http: { DELETE: "/users/{{ args.id }}" }
            selection: "{ id, name, age }"
          )
        }

        type User {
          id: ID!
          name: String!
          age: Int
        }

        input UserInput {
          name: String!
          age: Int!
        }
    "#,
    )
}

fn subgraph_sdl(url: &str, sdl: &str) -> String {
    [
        &format!(
            r#"
        extend schema
          @link(url: "https://specs.apollo.dev/federation/v2.0", import: ["@key", "@shareable"])
          @link(url: "<self>", import: ["@restEndpoint", "@rest"])

        extend schema @restEndpoint(
          name: "endpoint",
          baseURL: "{url}"
        )
        "#
        ),
        sdl,
    ]
    .join("\n")
}

async fn mock_server(listen_path: &str, template: ResponseTemplate, headers: &[(&str, &str)]) -> MockServer {
    let mock_server = MockServer::builder().start().await;

    let mut mock = Mock::given(method("GET")).and(path(listen_path));

    for (key, value) in headers {
        mock = mock.and(header(*key, *value));
    }

    mock.respond_with(template).mount(&mock_server).await;

    mock_server
}

#[tokio::test]
async fn get_many() {
    let response_body = json!([
        {
            "id": "1",
            "name": "John Doe",
            "age": 30,
            "nonimportant": 2,
        },
        {
            "id": "2",
            "name": "Jane Doe",
            "age": 25,
            "nonimportant": 3,
        }
    ]);

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/users", template, &[]).await;
    let subgraph = subgraph(&mock_server.uri());

    let gateway = TestGateway::builder().subgraph(subgraph).build().await.unwrap();

    let query = indoc! {r#"
        query {
          users {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": [
          {
            "id": "1",
            "name": "John Doe",
            "age": 30
          },
          {
            "id": "2",
            "name": "Jane Doe",
            "age": 25
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn with_required_headers() {
    let response_body = json!([
        {
            "id": "1",
            "name": "John Doe",
            "age": 30,
            "nonimportant": 2,
        },
        {
            "id": "2",
            "name": "Jane Doe",
            "age": 25,
            "nonimportant": 3,
        }
    ]);

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/users", template, &[("Authorization", "Bearer token")]).await;
    let subgraph = subgraph(&mock_server.uri());

    let config = indoc! {r#"
        [[subgraphs.test.headers]]
        rule = "forward"
        name = "Authorization"
    "#};

    let gateway = TestGateway::builder()
        .subgraph(("test", subgraph))
        .toml_config(config)
        .build()
        .await
        .unwrap();

    let query = indoc! {r#"
        query {
          users {
            id
            name
            age
          }
        }
    "#};

    let response = gateway
        .query(query)
        .header("Authorization", "Bearer token")
        .send()
        .await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": [
          {
            "id": "1",
            "name": "John Doe",
            "age": 30
          },
          {
            "id": "2",
            "name": "Jane Doe",
            "age": 25
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn static_headers() {
    let response_body = json!({
        "id": "1",
        "name": "John Doe",
        "age": 30,
    });

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/me", template, &[("X-Custom", "tea")]).await;
    let subgraph = format!(
        r#"
        extend schema
          @link(url: "https://specs.apollo.dev/federation/v2.0", import: ["@key", "@shareable"])
          @link(url: "<self>", import: ["@restEndpoint", "@rest"])
          @restEndpoint(
          name: "endpoint",
          baseURL: "{uri}"
          headers: [
            {{
                name: "X-Custom"
                value: "tea"
            }}
          ]
        )

        type Query {{
            me: User @rest(endpoint: "endpoint", http: {{ GET: "/me" }})
        }}

        type User {{
          id: ID!
          name: String!
          age: Int
        }}
        "#,
        uri = mock_server.uri()
    );

    let gateway = TestGateway::builder().subgraph(subgraph).build().await.unwrap();

    let query = indoc! {r#"
        query {
          me {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "me": {
          "id": "1",
          "name": "John Doe",
          "age": 30
        }
      }
    }
    "#);
}

#[tokio::test]
async fn headers_from_config() {
    let response_body = json!({
        "id": "1",
        "name": "John Doe",
        "age": 30,
    });

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/me", template, &[("X-Custom", "tea")]).await;
    let subgraph = format!(
        r#"
        extend schema
          @link(url: "https://specs.apollo.dev/federation/v2.0", import: ["@key", "@shareable"])
          @link(url: "<self>", import: ["@restEndpoint", "@rest"])
          @restEndpoint(
          name: "endpoint",
          baseURL: "{uri}"
          headers: [
            {{
                name: "X-Custom"
                value: "{{{{config.header}}}}"
            }}
          ]
        )

        type Query {{
            me: User @rest(endpoint: "endpoint", http: {{ GET: "/me" }})
        }}

        type User {{
          id: ID!
          name: String!
          age: Int
        }}
        "#,
        uri = mock_server.uri()
    );

    let gateway = TestGateway::builder()
        .subgraph(("test", subgraph))
        .toml_config(
            r#"
            [extensions.rest.config.subgraphs.test]
            header = "tea"
            "#,
        )
        .build()
        .await
        .unwrap();

    let query = indoc! {r#"
        query {
          me {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "me": {
          "id": "1",
          "name": "John Doe",
          "age": 30
        }
      }
    }
    "#);
}

#[tokio::test]
async fn get_one() {
    let response_body = json!({
        "id": "1",
        "name": "John Doe",
        "age": 30,
    });

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/users/1", template, &[]).await;

    let gateway = TestGateway::builder()
        .subgraph(subgraph(&mock_server.uri()))
        .build()
        .await
        .unwrap();

    let query = indoc! {r#"
        query {
          user(id: 1) {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": "1",
          "name": "John Doe",
          "age": 30
        }
      }
    }
    "#);
}

#[tokio::test]
async fn get_one_missing() {
    let response_body = json!(null);

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/users/1", template, &[]).await;

    let gateway = TestGateway::builder()
        .subgraph(subgraph(&mock_server.uri()))
        .build()
        .await
        .unwrap();

    let query = indoc! {r#"
        query {
          user(id: 1) {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": null
      }
    }
    "#);
}

#[tokio::test]
async fn get_one_nested_null() {
    let response_body = json!({
        "id": "1",
        "name": "John Doe",
        "age": null,
    });

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/users/1", template, &[]).await;

    let gateway = TestGateway::builder()
        .subgraph(subgraph(&mock_server.uri()))
        .build()
        .await
        .unwrap();

    let query = indoc! {r#"
        query {
          user(id: 1) {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": "1",
          "name": "John Doe",
          "age": null
        }
      }
    }
    "#);
}

#[tokio::test]
async fn get_some_fields() {
    let response_body = json!([
        {
            "id": "1",
            "name": "John Doe",
            "age": 30
        },
        {
            "id": "2",
            "name": "Jane Doe",
            "age": 25
        }
    ]);

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/users", template, &[]).await;

    let gateway = TestGateway::builder()
        .subgraph(subgraph(&mock_server.uri()))
        .build()
        .await
        .unwrap();

    let query = indoc! {r#"
        query {
          users {
            id
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": [
          {
            "id": "1"
          },
          {
            "id": "2"
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn faulty_response() {
    let response_body = json!([
        {
            "foo": "1",
            "bar": "John Doe",
            "lol": 30
        }
    ]);

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/users", template, &[]).await;

    let gateway = TestGateway::builder()
        .subgraph(subgraph(&mock_server.uri()))
        .build()
        .await
        .unwrap();

    let query = indoc! {r#"
        query {
          users {
            id
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": null,
      "errors": [
        {
          "message": "Invalid response from subgraph",
          "locations": [
            {
              "line": 3,
              "column": 5
            }
          ],
          "path": [
            "users",
            0,
            "id"
          ],
          "extensions": {
            "code": "SUBGRAPH_INVALID_RESPONSE_ERROR"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn internal_server_error() {
    let template = ResponseTemplate::new(500);
    let mock_server = mock_server("/users", template, &[]).await;
    let subgraph = subgraph(&mock_server.uri());

    let gateway = TestGateway::builder().subgraph(subgraph).build().await.unwrap();

    let query = indoc! {r#"
        query {
          users {
            id
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": null,
      "errors": [
        {
          "message": "HTTP request failed with status: 500 Internal Server Error",
          "locations": [
            {
              "line": 2,
              "column": 3
            }
          ],
          "path": [
            "users"
          ],
          "extensions": {
            "code": "EXTENSION_ERROR"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn with_bad_jq() {
    let response_body = json!([
        {
            "id": "1",
            "name": "John Doe",
            "age": 30,
            "nonimportant": 2,
        },
        {
            "id": "2",
            "name": "Jane Doe",
            "age": 25,
            "nonimportant": 3,
        }
    ]);

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/users", template, &[]).await;
    let rest_endpoint = mock_server.uri();

    let schema = formatdoc! {r#"
        extend schema
          @link(url: "https://specs.apollo.dev/federation/v2.0", import: ["@key", "@shareable"])
          @link(url: "<self>", import: ["@restEndpoint", "@rest"])

        @restEndpoint(
          name: "endpoint",
          baseURL: "{rest_endpoint}"
        )

        type Query {{
          users: [User!]! @rest(
            endpoint: "endpoint",
            http: {{ GET: "/users" }},
            selection: "\\||\\"
          )
        }}

        type User {{
          id: ID!
          name: String!
          age: Int!
        }}
    "#};

    let gateway = TestGateway::builder().subgraph(schema).build().await.unwrap();

    let query = indoc! {r#"
        query {
          users {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": null,
      "errors": [
        {
          "message": "Failed to filter with selection: The selection is not valid jq syntax: `\\||\\`",
          "locations": [
            {
              "line": 2,
              "column": 3
            }
          ],
          "path": [
            "users"
          ],
          "extensions": {
            "code": "EXTENSION_ERROR"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn with_path_in_the_endpoint() {
    let response_body = json!([
        {
            "id": "1",
            "name": "John Doe",
            "age": 30,
            "nonimportant": 2,
        },
        {
            "id": "2",
            "name": "Jane Doe",
            "age": 25,
            "nonimportant": 3,
        }
    ]);

    let template = ResponseTemplate::new(200).set_body_json(response_body);
    let mock_server = mock_server("/admin/users", template, &[]).await;
    let rest_endpoint = mock_server.uri();

    let schema = formatdoc! {r#"
        extend schema
          @link(url: "https://specs.apollo.dev/federation/v2.0", import: ["@key", "@shareable"])
          @link(url: "<self>", import: ["@restEndpoint", "@rest"])

        @restEndpoint(
          name: "endpoint",
          baseURL: "{rest_endpoint}/admin"
        )

        type Query {{
          users: [User!]! @rest(
            endpoint: "endpoint",
            http: {{ GET: "/users" }},
            selection: "[.[] | {{ id, name, age }}]"
          )
        }}

        type User {{
          id: ID!
          name: String!
          age: Int!
        }}
    "#};

    let gateway = TestGateway::builder().subgraph(schema).build().await.unwrap();

    let query = indoc! {r#"
        query {
          users {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": [
          {
            "id": "1",
            "name": "John Doe",
            "age": 30
          },
          {
            "id": "2",
            "name": "Jane Doe",
            "age": 25
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn update_user() {
    let request_body = json!({
        "name": "John Doe",
        "age": 30,
    });

    let response_body = json!({
        "id": "1",
        "name": "John Doe",
        "age": 30,
    });

    let template = ResponseTemplate::new(201).set_body_json(response_body);
    let mock_server = MockServer::builder().start().await;

    Mock::given(method("PUT"))
        .and(path("/users/1"))
        .and(body_json(request_body))
        .and(header("Content-Type", "application/json"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let gateway = TestGateway::builder()
        .subgraph(subgraph(&mock_server.uri()))
        .build()
        .await
        .unwrap();

    let mutation = indoc! {r#"
        mutation {
          updateUser(id: 1, input: { name: "John Doe", age: 30 }) {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "updateUser": {
          "id": "1",
          "name": "John Doe",
          "age": 30
        }
      }
    }
    "#);
}

#[tokio::test]
async fn delete_user() {
    let response_body = json!({
        "id": "1",
        "name": "John Doe",
        "age": 30,
    });

    let template = ResponseTemplate::new(201).set_body_json(response_body);
    let mock_server = MockServer::builder().start().await;

    Mock::given(method("DELETE"))
        .and(path("/users/1"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let gateway = TestGateway::builder()
        .subgraph(subgraph(&mock_server.uri()))
        .build()
        .await
        .unwrap();

    let mutation = indoc! {r#"
        mutation {
          deleteUser(id: 1) {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "deleteUser": {
          "id": "1",
          "name": "John Doe",
          "age": 30
        }
      }
    }
    "#);
}

#[tokio::test]
async fn dynamic_post() {
    let request_body = json!({
        "name": "John Doe",
        "age": 30,
    });

    let response_body = json!({
        "id": "1",
        "name": "John Doe",
        "age": 30,
    });

    let template = ResponseTemplate::new(201).set_body_json(response_body);
    let mock_server = MockServer::builder().start().await;

    Mock::given(method("POST"))
        .and(path("/users"))
        .and(body_json(request_body))
        .and(header("Content-Type", "application/json"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let gateway = TestGateway::builder()
        .subgraph(subgraph(&mock_server.uri()))
        .build()
        .await
        .unwrap();

    let mutation = indoc! {r#"
        mutation {
          createUser(input: { name: "John Doe", age: 30 }) {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "createUser": {
          "id": "1",
          "name": "John Doe",
          "age": 30
        }
      }
    }
    "#);
}

#[tokio::test]
async fn static_post() {
    let request_body = json!({
        "name": "John Doe",
        "age": 30,
    });

    let response_body = json!({
        "id": "1",
        "name": "John Doe",
        "age": 30,
    });

    let template = ResponseTemplate::new(201).set_body_json(response_body);
    let mock_server = MockServer::builder().start().await;

    Mock::given(method("POST"))
        .and(path("/users"))
        .and(body_json(request_body))
        .and(header("Content-Type", "application/json"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let gateway = TestGateway::builder()
        .subgraph(subgraph(&mock_server.uri()))
        .build()
        .await
        .unwrap();

    let mutation = indoc! {r#"
        mutation {
          createStaticUser {
            id
            name
            age
          }
        }
    "#};

    let response = gateway.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "createStaticUser": {
          "id": "1",
          "name": "John Doe",
          "age": 30
        }
      }
    }
    "#);
}

#[tokio::test]
async fn arguments_in_selection() {
    let mock_server = MockServer::builder().start().await;

    Mock::given(method("GET"))
        .and(path("/my-summary"))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "text": "Summary for John Doe",
        })))
        .mount(&mock_server)
        .await;

    let gateway = TestGateway::builder()
        .subgraph(subgraph_sdl(
            &mock_server.uri(),
            r#"
            type Query {
              test(myContext: Context!): Summary
                @rest(
                  endpoint: "endpoint"
                  http: {
                    GET: "/my-summary"
                  }
                  selection: "{ text: {{ args.myContext.name }} }"
                )
            }

            input Context {
                name: String!
            }

            type Summary {
                text: String!
            }
            "#,
        ))
        .stream_stdout_stderr()
        .build()
        .await
        .unwrap();

    let response = gateway
        .query(
            r#"
            query {
              test(myContext: { name: "John Doe" }) {
                text
              }
            }
            "#,
        )
        .send()
        .await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "test": {
          "text": "John Doe"
        }
      }
    }
    "#);
}
