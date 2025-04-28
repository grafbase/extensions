use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn eq() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { name: { eq: "Musti" } }) {
            returning { id name }
            rowCount
          }
        }
    "#};

    let response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 1,
              "name": "Musti"
            },
            {
              "id": 3,
              "name": "Musti"
            }
          ],
          "rowCount": 2
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
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
                    "id": 2,
                    "name": "Naukio"
                  }
                }
              ]
            }
          }
        }
    "#);
}

#[tokio::test]
async fn eq_not_returning() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { name: { eq: "Musti" } }) {
            rowCount
          }
        }
    "#};

    let response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "rowCount": 2
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
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
                "id": 2,
                "name": "Naukio"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn missing() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { name: { eq: "Pertti" } }) {
            returning { id name }
            rowCount
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [],
          "rowCount": 0
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti"
              }
            },
            {
              "node": {
                "id": 2,
                "name": "Naukio"
              }
            },
            {
              "node": {
                "id": 3,
                "name": "Musti"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn eq_null() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, null), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userDeleteMany(filter: { name: { eq: null } }) {
            returning { id name }
          }
        }
    "};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "name": null
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti"
              }
            },
            {
              "node": {
                "id": 3,
                "name": "Musti"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn ne_null() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, null), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userDeleteMany(filter: { name: { ne: null } }) {
            returning { id name }
          }
        }
    "};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 1,
              "name": "Musti"
            },
            {
              "id": 3,
              "name": "Musti"
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 2,
                "name": null
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn eq_two_fields() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                age INT NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name, age) VALUES (1, 'Musti', 11), (2, 'Naukio', 11), (3, 'Musti', 12)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { ALL: [ { name: { eq: "Musti" } }, { age: { eq: 12 } } ] }) {
            returning { id name age }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 3,
              "name": "Musti",
              "age": 12
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name age } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti",
                "age": 11
              }
            },
            {
              "node": {
                "id": 2,
                "name": "Naukio",
                "age": 11
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn eq_rename() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name_game VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name_game) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { nameGame: { eq: "Musti" } }) {
            returning { id nameGame }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 1,
              "nameGame": "Musti"
            },
            {
              "id": 3,
              "nameGame": "Musti"
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id nameGame } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 2,
                "nameGame": "Naukio"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn ne() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { name: { ne: "Musti" } }) {
            returning { id name }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "name": "Naukio"
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti"
              }
            },
            {
              "node": {
                "id": 3,
                "name": "Musti"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn gt() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { id: { gt: 1 } }) {
            returning { id name }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "name": "Naukio"
            },
            {
              "id": 3,
              "name": "Musti"
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn lt() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { id: { lt: 3 } }) {
            returning { id name }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 1,
              "name": "Musti"
            },
            {
              "id": 2,
              "name": "Naukio"
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 3,
                "name": "Musti"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn gte() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { id: { gte: 2 } }) {
            returning { id name }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "name": "Naukio"
            },
            {
              "id": 3,
              "name": "Musti"
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn lte() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { id: { lte: 2 } }) {
            returning { id name }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 1,
              "name": "Musti"
            },
            {
              "id": 2,
              "name": "Naukio"
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 3,
                "name": "Musti"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn r#in() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { id: { in: [1, 3] } }) {
            returning { id name }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 1,
              "name": "Musti"
            },
            {
              "id": 3,
              "name": "Musti"
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 2,
                "name": "Naukio"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn nin() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { id: { nin: [1, 3] } }) {
            returning { id name }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "name": "Naukio"
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti"
              }
            },
            {
              "node": {
                "id": 3,
                "name": "Musti"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn all() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                age INT NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name, age) VALUES (1, 'Musti', 11), (2, 'Naukio', 11), (3, 'Musti', 12)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { ALL: [
            { name: { eq: "Musti" } },
            { age: { eq: 11 } }
          ]}) {
            returning { id name age }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 1,
              "name": "Musti",
              "age": 11
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name age } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 2,
                "name": "Naukio",
                "age": 11
              }
            },
            {
              "node": {
                "id": 3,
                "name": "Musti",
                "age": 12
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn any() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                age INT NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name, age) VALUES (1, 'Musti', 11), (2, 'Naukio', 11), (3, 'Musti', 12)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { ANY: [
            { name: { eq: "Naukio" } },
            { age: { eq: 12 } }
          ]}) {
            returning { id name age }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "name": "Naukio",
              "age": 11
            },
            {
              "id": 3,
              "name": "Musti",
              "age": 12
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name age } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti",
                "age": 11
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn none() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                age INT NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name, age) VALUES (1, 'Musti', 11), (2, 'Naukio', 12), (3, 'Pentti', 13)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { NONE: [
            { name: { eq: "Musti" } },
            { age: { eq: 13 } }
          ]}) {
            returning { id name age }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "name": "Naukio",
              "age": 12
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name age } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti",
                "age": 11
              }
            },
            {
              "node": {
                "id": 3,
                "name": "Pentti",
                "age": 13
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn not() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { name: { not: { eq: "Musti" } } }) {
            returning { id name }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "name": "Naukio"
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) { edges { node { id name } } }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti"
              }
            },
            {
              "node": {
                "id": 3,
                "name": "Musti"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn array_eq() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                numbers INT[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, numbers) VALUES (1, '{1, 2}'), (2, '{3, 4}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { numbers: { eq: [3, 4] } }) {
            returning { id numbers }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "numbers": [
                3,
                4
              ]
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) {
            edges { node { id numbers } }
          }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "numbers": [
                  1,
                  2
                ]
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn array_ne() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                numbers INT[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, numbers) VALUES (1, '{1, 2}'), (2, '{3, 4}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { numbers: { ne: [3, 4] } }) {
            returning { id numbers }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 1,
              "numbers": [
                1,
                2
              ]
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) {
            edges { node { id numbers } }
          }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 2,
                "numbers": [
                  3,
                  4
                ]
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn array_gt() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                numbers INT[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, numbers) VALUES (1, '{1, 2}'), (2, '{3, 4}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { numbers: { gt: [1, 2] } }) {
            returning { id numbers }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "numbers": [
                3,
                4
              ]
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) {
            edges { node { id numbers } }
          }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "numbers": [
                  1,
                  2
                ]
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn array_contains() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                numbers INT[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, numbers) VALUES (1, '{1, 2}'), (2, '{3, 4}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { numbers: { contains: [1, 2, 2, 1] } }) {
            returning { id numbers }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 1,
              "numbers": [
                1,
                2
              ]
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) {
            edges { node { id numbers } }
          }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 2,
                "numbers": [
                  3,
                  4
                ]
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn array_contained() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                numbers INT[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, numbers) VALUES (1, '{1, 2}'), (2, '{3, 4}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { numbers: { contained: [3, 6, 4, 7] } }) {
            returning { id numbers }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 2,
              "numbers": [
                3,
                4
              ]
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) {
            edges { node { id numbers } }
          }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "numbers": [
                  1,
                  2
                ]
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn array_overlaps() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                numbers INT[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, numbers) VALUES (1, '{1, 2}'), (2, '{3, 4}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDeleteMany(filter: { numbers: { overlaps: [1, 5, 5, 6] } }) {
            returning { id numbers }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDeleteMany": {
          "returning": [
            {
              "id": 1,
              "numbers": [
                1,
                2
              ]
            }
          ]
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(first: 10) {
            edges { node { id numbers } }
          }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 2,
                "numbers": [
                  3,
                  4
                ]
              }
            }
          ]
        }
      }
    }
    "#);
}
