use crate::PgTestApi;
use indoc::indoc;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Users {
    page_info: PageInfo,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
    has_next_page: bool,
    has_previous_page: bool,
    start_cursor: String,
    end_cursor: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseData {
    users: Users,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    data: ResponseData,
}

#[tokio::test]
async fn id_pk_implicit_order_with_after() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(first: 1) {
            edges { node { name } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<Response>(query).send().await.unwrap();
    let page_info = response.data.users.page_info;

    assert!(page_info.has_next_page);

    let query = indoc! {r#"
        query Pg($after: String) {
          users(first: 1, after: $after) {
            edges { node { name } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "#};

    let variables = serde_json::json!({
        "after": page_info.end_cursor,
    });

    let response = runner
        .graphql_query::<serde_json::Value>(query)
        .with_variables(variables)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Naukio"
              },
              "cursor": "WzJd"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "WzJd",
            "endCursor": "WzJd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn id_pk_implicit_order_with_before() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(last: 1) {
            edges { node { name } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<Response>(query).send().await.unwrap();
    let page_info = response.data.users.page_info;

    assert!(page_info.has_previous_page);

    let query = indoc! {r#"
        query Pg($before: String) {
          users(last: 1, before: $before) {
            edges { node { name } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "#};

    let variables = serde_json::json!({
        "before": page_info.start_cursor,
    });

    let response = runner
        .graphql_query::<serde_json::Value>(query)
        .with_variables(variables)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Musti"
              },
              "cursor": "WzFd"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "WzFd",
            "endCursor": "WzFd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn id_pk_explicit_order_with_after() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(first: 1, orderBy: [{ id: DESC }]) {
            edges { node { name } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<Response>(query).send().await.unwrap();
    let page_info = response.data.users.page_info;
    let cursor = page_info.end_cursor;

    assert!(page_info.has_next_page);

    let query = format!(
        r#"
        query {{
          users(first: 1, orderBy: [{{ id: DESC }}], after: "{cursor}") {{
            edges {{ node {{ name }} cursor }}
            pageInfo {{ hasNextPage hasPreviousPage startCursor endCursor }}
          }}
        }}
    "#
    );

    let response = runner.graphql_query::<serde_json::Value>(&query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Musti"
              },
              "cursor": "WzFd"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "WzFd",
            "endCursor": "WzFd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn id_pk_explicit_order_with_before() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(last: 1, orderBy: [{ id: DESC }]) {
            edges { node { name } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<Response>(query).send().await.unwrap();
    let page_info = response.data.users.page_info;
    let cursor = page_info.start_cursor;

    assert!(page_info.has_previous_page);

    let query = format!(
        r#"
        query {{
          users(last: 1, before: "{cursor}", orderBy: [{{ id: DESC }}]) {{
            edges {{ node {{ name }} cursor }}
            pageInfo {{ hasNextPage hasPreviousPage startCursor endCursor }}
          }}
        }}
    "#
    );

    let response = runner.graphql_query::<serde_json::Value>(&query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Naukio"
              },
              "cursor": "WzJd"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "WzJd",
            "endCursor": "WzJd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn compound_pk_implicit_order_with_after() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                CONSTRAINT "User_pkey" PRIMARY KEY (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (name, email) VALUES
                ('Musti', 'meow1@example.com'),
                ('Musti', 'meow2@example.com'),
                ('Naukio', 'meow3@example.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(first: 1) {
            edges { node { name email } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<Response>(query).send().await.unwrap();
    let page_info = response.data.users.page_info;
    let cursor = page_info.end_cursor;

    assert!(page_info.has_next_page);

    let query = format!(
        r#"
        query {{
          users(first: 1, after: "{cursor}") {{
            edges {{ node {{ name email }} cursor }}
            pageInfo {{ hasNextPage hasPreviousPage startCursor endCursor }}
          }}
        }}
    "#
    );

    let response = runner.graphql_query::<serde_json::Value>(&query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Musti",
                "email": "meow2@example.com"
              },
              "cursor": "WyJNdXN0aSIsICJtZW93MkBleGFtcGxlLmNvbSJd"
            }
          ],
          "pageInfo": {
            "hasNextPage": true,
            "hasPreviousPage": false,
            "startCursor": "WyJNdXN0aSIsICJtZW93MkBleGFtcGxlLmNvbSJd",
            "endCursor": "WyJNdXN0aSIsICJtZW93MkBleGFtcGxlLmNvbSJd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn compound_pk_implicit_order_with_before() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                CONSTRAINT "User_pkey" PRIMARY KEY (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (name, email) VALUES
                ('Musti', 'meow1@example.com'),
                ('Naukio', 'meow3@example.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(last: 1) {
            edges { node { name email } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<Response>(query).send().await.unwrap();
    let page_info = response.data.users.page_info;
    let cursor = page_info.start_cursor;

    assert!(page_info.has_previous_page);

    let query = format!(
        r#"
        query {{
          users(last: 1, before: "{cursor}") {{
            edges {{ node {{ name email }} cursor }}
            pageInfo {{ hasNextPage hasPreviousPage startCursor endCursor }}
          }}
        }}
    "#
    );

    let response = runner.graphql_query::<serde_json::Value>(&query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Musti",
                "email": "meow1@example.com"
              },
              "cursor": "WyJNdXN0aSIsICJtZW93MUBleGFtcGxlLmNvbSJd"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "WyJNdXN0aSIsICJtZW93MUBleGFtcGxlLmNvbSJd",
            "endCursor": "WyJNdXN0aSIsICJtZW93MUBleGFtcGxlLmNvbSJd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn compound_pk_explicit_order_with_after() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                CONSTRAINT "User_pkey" PRIMARY KEY (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (name, email) VALUES
                ('Musti', 'meow1@example.com'),
                ('Musti', 'meow2@example.com'),
                ('Naukio', 'meow3@example.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(first: 1, orderBy: [{ name: ASC }, { email: DESC }]) {
            edges { node { name email } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<Response>(query).send().await.unwrap();
    let page_info = response.data.users.page_info;
    let cursor = page_info.end_cursor;

    assert!(page_info.has_next_page);

    let query = format!(
        r#"
        query {{
          users(first: 1, orderBy: [{{ name: ASC }}, {{ email: DESC }}], after: "{cursor}") {{
            edges {{ node {{ name email }} cursor }}
            pageInfo {{ hasNextPage hasPreviousPage startCursor endCursor }}
          }}
        }}
    "#
    );

    let response = runner.graphql_query::<serde_json::Value>(&query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Musti",
                "email": "meow1@example.com"
              },
              "cursor": "WyJNdXN0aSIsICJtZW93MUBleGFtcGxlLmNvbSJd"
            }
          ],
          "pageInfo": {
            "hasNextPage": true,
            "hasPreviousPage": false,
            "startCursor": "WyJNdXN0aSIsICJtZW93MUBleGFtcGxlLmNvbSJd",
            "endCursor": "WyJNdXN0aSIsICJtZW93MUBleGFtcGxlLmNvbSJd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn compound_pk_explicit_order_with_before() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                CONSTRAINT "User_pkey" PRIMARY KEY (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (name, email) VALUES
                ('Musti', 'meow1@example.com'),
                ('Musti', 'meow2@example.com'),
                ('Naukio', 'meow3@example.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(last: 1, orderBy: [{ name: ASC }, { email: DESC }]) {
            edges { node { name email } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<Response>(query).send().await.unwrap();
    let page_info = response.data.users.page_info;
    let cursor = page_info.start_cursor;

    assert!(page_info.has_previous_page);

    let query = format!(
        r#"
        query {{
          users(last: 1, orderBy: [{{ name: ASC }}, {{ email: DESC }}], before: "{cursor}") {{
            edges {{ node {{ name email }} cursor }}
            pageInfo {{ hasNextPage hasPreviousPage startCursor endCursor }}
          }}
        }}
    "#
    );

    let response = runner.graphql_query::<serde_json::Value>(&query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Musti",
                "email": "meow1@example.com"
              },
              "cursor": "WyJNdXN0aSIsICJtZW93MUBleGFtcGxlLmNvbSJd"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": true,
            "startCursor": "WyJNdXN0aSIsICJtZW93MUBleGFtcGxlLmNvbSJd",
            "endCursor": "WyJNdXN0aSIsICJtZW93MUBleGFtcGxlLmNvbSJd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn compound_pk_implicit_order_with_nulls_and_after() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NULL,
                CONSTRAINT "User_key" UNIQUE (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (name, email) VALUES
                ('Musti', NULL),
                ('Naukio', NULL),
                ('Naukio', 'meow3@example.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(first: 1) {
            edges { node { name email } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<Response>(query).send().await.unwrap();
    let page_info = response.data.users.page_info;
    let cursor = page_info.end_cursor;

    assert!(page_info.has_next_page);

    let query = format!(
        r#"
        query {{
          users(first: 1, after: "{cursor}") {{
            edges {{ node {{ name email }} cursor }}
            pageInfo {{ hasNextPage hasPreviousPage startCursor endCursor }}
          }}
        }}
    "#
    );

    let response = runner.graphql_query::<serde_json::Value>(&query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Naukio",
                "email": null
              },
              "cursor": "WyJOYXVraW8iLCBudWxsXQ=="
            }
          ],
          "pageInfo": {
            "hasNextPage": true,
            "hasPreviousPage": false,
            "startCursor": "WyJOYXVraW8iLCBudWxsXQ==",
            "endCursor": "WyJOYXVraW8iLCBudWxsXQ=="
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn compound_pk_implicit_order_with_nulls_and_before() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NULL,
                CONSTRAINT "User_key" UNIQUE (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (name, email) VALUES
                ('Musti', NULL),
                ('Naukio', NULL),
                ('Naukio', 'meow3@example.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(last: 1) {
            edges { node { name email } cursor }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<Response>(query).send().await.unwrap();
    let page_info = response.data.users.page_info;
    let cursor = page_info.start_cursor;

    assert!(page_info.has_previous_page);

    let query = format!(
        r#"
        query {{
          users(last: 1, before: "{cursor}") {{
            edges {{ node {{ name email }} cursor }}
            pageInfo {{ hasNextPage hasPreviousPage startCursor endCursor }}
          }}
        }}
    "#
    );

    let response = runner.graphql_query::<serde_json::Value>(&query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Naukio",
                "email": "meow3@example.com"
              },
              "cursor": "WyJOYXVraW8iLCAibWVvdzNAZXhhbXBsZS5jb20iXQ=="
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": true,
            "startCursor": "WyJOYXVraW8iLCAibWVvdzNAZXhhbXBsZS5jb20iXQ==",
            "endCursor": "WyJOYXVraW8iLCAibWVvdzNAZXhhbXBsZS5jb20iXQ=="
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn nested_page_info_no_limit() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
            )
        "#};

        api.execute_sql(profile_table).await;

        let insert_users = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES
              (1, 'Musti'),
              (2, 'Naukio')
        "#};

        api.execute_sql(insert_users).await;

        let insert_profiles = indoc! {r#"
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(first: 1000, filter: { id: { eq: 1 } }) {
            edges {
              node {
                blogs(first: 1000) {
                  edges { node { id title } cursor }
                  pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
                }
              }
              cursor
            }
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "blogs": {
                  "edges": [
                    {
                      "node": {
                        "id": 1,
                        "title": "Hello, world!"
                      },
                      "cursor": "WzFd"
                    },
                    {
                      "node": {
                        "id": 2,
                        "title": "Sayonara..."
                      },
                      "cursor": "WzJd"
                    }
                  ],
                  "pageInfo": {
                    "hasNextPage": false,
                    "hasPreviousPage": false,
                    "startCursor": "WzFd",
                    "endCursor": "WzJd"
                  }
                }
              },
              "cursor": "WzFd"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "WzFd",
            "endCursor": "WzFd"
          }
        }
      }
    }
    "#);
}
