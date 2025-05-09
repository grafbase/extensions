use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn page_info_no_nesting() {
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
          users(first: 10) {
            edges { node { name } cursor }
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
                "name": "Musti"
              },
              "cursor": "WzFd"
            },
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
            "startCursor": "WzFd",
            "endCursor": "WzJd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn load_only_page_info() {
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
          users(first: 10) {
            pageInfo { hasNextPage hasPreviousPage startCursor endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "WzFd",
            "endCursor": "WzJd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn load_only_has_next_page() {
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
          users(first: 10) {
            pageInfo { hasNextPage }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "pageInfo": {
            "hasNextPage": false
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn load_only_has_previous_page() {
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
          users(first: 10) {
            pageInfo { hasPreviousPage }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "pageInfo": {
            "hasPreviousPage": false
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn load_only_start_cursor() {
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
          users(first: 10) {
            pageInfo { startCursor }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "pageInfo": {
            "startCursor": "WzFd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn load_only_end_cursor() {
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
          users(first: 10) {
            pageInfo { endCursor }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "pageInfo": {
            "endCursor": "WzJd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn load_only_cursors() {
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
          users(first: 10) {
            edges { cursor }
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
              "cursor": "WzFd"
            },
            {
              "cursor": "WzJd"
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn page_info_first_has_more_data() {
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

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

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
            "hasNextPage": true,
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
async fn page_info_last_has_more_data() {
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

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

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
            "hasPreviousPage": true,
            "startCursor": "WzJd",
            "endCursor": "WzJd"
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
          users(first: 10, filter: { id: { eq: 1 } }, orderBy: [{ name: ASC }]) {
            edges {
              node {
                blogs(first: 10, orderBy: [{ title: ASC }]) {
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
                      "cursor": "WyJIZWxsbywgd29ybGQhIiwgMV0="
                    },
                    {
                      "node": {
                        "id": 2,
                        "title": "Sayonara..."
                      },
                      "cursor": "WyJTYXlvbmFyYS4uLiIsIDJd"
                    }
                  ],
                  "pageInfo": {
                    "hasNextPage": false,
                    "hasPreviousPage": false,
                    "startCursor": "WyJIZWxsbywgd29ybGQhIiwgMV0=",
                    "endCursor": "WyJTYXlvbmFyYS4uLiIsIDJd"
                  }
                }
              },
              "cursor": "WyJNdXN0aSIsIDFd"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "WyJNdXN0aSIsIDFd",
            "endCursor": "WyJNdXN0aSIsIDFd"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn nested_page_info_with_first() {
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
          users(first: 10, filter: { id: { eq: 1 } }) {
            edges {
              node {
                blogs(first: 1) {
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
                    }
                  ],
                  "pageInfo": {
                    "hasNextPage": true,
                    "hasPreviousPage": false,
                    "startCursor": "WzFd",
                    "endCursor": "WzFd"
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

#[tokio::test]
async fn nested_page_info_with_last() {
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
          users(first: 10, filter: { id: { eq: 1 } }) {
            edges {
              node {
                blogs(last: 1) {
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
                        "id": 2,
                        "title": "Sayonara..."
                      },
                      "cursor": "WzJd"
                    }
                  ],
                  "pageInfo": {
                    "hasNextPage": false,
                    "hasPreviousPage": true,
                    "startCursor": "WzJd",
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
