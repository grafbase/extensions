mod cursors;
mod filters;

use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn xxx_page_info_no_nesting() {
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
          users(first: 2, orderBy: [{ name: ASC }]) {
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
              "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
            },
            {
              "node": {
                "name": "Naukio"
              },
              "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgIUFAgBByQBPAEBAQcoAiQB"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB",
            "endCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgIUFAgBByQBPAEBAQcoAiQB"
          }
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
              "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
            }
          ],
          "pageInfo": {
            "hasNextPage": true,
            "hasPreviousPage": false,
            "startCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB",
            "endCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
          }
        }
      }
    }"#);
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
              "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAKRGVzY2VuZGluZwADFycfAwEDEicCFBQIAQckAT0BAQEHKAIkAQ"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": true,
            "startCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAKRGVzY2VuZGluZwADFycfAwEDEicCFBQIAQckAT0BAQEHKAIkAQ",
            "endCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAKRGVzY2VuZGluZwADFycfAwEDEicCFBQIAQckAT0BAQEHKAIkAQ"
          }
        }
      }
    }"#);
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
          users(first: 10, filter: { id: { eq: 1 } }) {
            edges {
              node {
                blogs(first: 10) {
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
                      "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
                    },
                    {
                      "node": {
                        "id": 2,
                        "title": "Sayonara..."
                      },
                      "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgIUFAgBByQBPAEBAQcoAiQB"
                    }
                  ],
                  "pageInfo": {
                    "hasNextPage": false,
                    "hasPreviousPage": false,
                    "startCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB",
                    "endCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgIUFAgBByQBPAEBAQcoAiQB"
                  }
                }
              },
              "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB",
            "endCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
          }
        }
      }
    }"#);
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
                      "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
                    }
                  ],
                  "pageInfo": {
                    "hasNextPage": true,
                    "hasPreviousPage": false,
                    "startCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB",
                    "endCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
                  }
                }
              },
              "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB",
            "endCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
          }
        }
      }
    }"#);
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
                      "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAKRGVzY2VuZGluZwADFycfAwEDEicCFBQIAQckAT0BAQEHKAIkAQ"
                    }
                  ],
                  "pageInfo": {
                    "hasNextPage": false,
                    "hasPreviousPage": true,
                    "startCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAKRGVzY2VuZGluZwADFycfAwEDEicCFBQIAQckAT0BAQEHKAIkAQ",
                    "endCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAKRGVzY2VuZGluZwADFycfAwEDEicCFBQIAQckAT0BAQEHKAIkAQ"
                  }
                }
              },
              "cursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
            }
          ],
          "pageInfo": {
            "hasNextPage": false,
            "hasPreviousPage": false,
            "startCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB",
            "endCursor": "ZmllbGRzAG5hbWUAAmlkAHZhbHVlAGRpcmVjdGlvbgAJQXNjZW5kaW5nAAMWJh4DAQMRJgEUFAgBByQBPAEBAQcoAiQB"
          }
        }
      }
    }"#);
}
