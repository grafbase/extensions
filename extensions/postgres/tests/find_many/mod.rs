mod pagination;

use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn everything() {
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
          users {
            edges { node { id name } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn eq_pk() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "users" (id, name) VALUES (1, 'Musti'), (2, 'Naukio')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(first: 10, filter: { id: { eq: 1 } }) {
            edges { node { id name } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn first() {
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
            edges { node { id name } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn last() {
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
            edges { node { id name } }
          }
        }
    "};

    let response = runner.query(query).send().await;

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
async fn order_by() {
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
          users(first: 10, orderBy: [{ name: DESC }]) {
            edges { node { id name } }
          }
        }
    "};

    let response = runner.query(query).send().await;

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
            },
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
async fn order_by_without_selecting_id() {
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
          users(first: 10, orderBy: [{ name: DESC }]) {
            edges { node { name } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "name": "Naukio"
              }
            },
            {
              "node": {
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
async fn eq_pk_rename() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id_field INT PRIMARY KEY,
                name_field VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id_field, name_field) VALUES (1, 'Musti'), (2, 'Naukio')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(first: 10, filter: { idField: { eq: 1 } }) {
            edges { node { idField nameField } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "idField": 1,
                "nameField": "Musti"
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn string_eq() {
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

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { eq: "Musti" } }) {
            edges { node { id name } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn string_like() {
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

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { like: "%us%" } }) {
            edges { node { id name } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn bytea_eq() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val BYTEA NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, '\xdeadbeef'::bytea), (2, '\xbeefdead'::bytea)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query Pg {
          users(first: 10, filter: { val: { eq: "\\xdeadbeef" }}) { edges { node { id val }} }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "val": "\\xdeadbeef"
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

    let query = indoc! {r"
        query {
          users(first: 10, filter: { numbers: { eq: [3, 4] } }) {
            edges { node { id numbers } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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

    let query = indoc! {r"
        query {
          users(first: 10, filter: { numbers: { ne: [3, 4] } }) {
            edges { node { id numbers } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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

    let query = indoc! {r"
        query {
          users(first: 10, filter: { numbers: { gt: [1, 2] } }) {
            edges { node { id numbers } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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

    let query = indoc! {r"
        query {
          users(first: 10, filter: { numbers: { contains: [1, 2, 2, 1] } }) {
            edges { node { id numbers } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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

    let query = indoc! {r"
        query {
          users(first: 10, filter: { numbers: { contained: [3, 6, 4, 7] } }) {
            edges { node { id numbers } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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

    let query = indoc! {r"
        query {
          users(first: 10, filter: { numbers: { overlaps: [1, 5, 5, 6] } }) {
            edges { node { id numbers } }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn two_field_eq() {
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
            INSERT INTO "User" (id, name, age) VALUES (1, 'Musti', 11), (2, 'Musti', 12)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { ALL: [ { name: { eq: "Musti" } }, { age: { eq: 11 } } ] }) {
            edges { node { id name age } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn string_ne() {
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

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { ne: "Musti" } }) {
            edges { node { id name } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

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
async fn string_gt() {
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

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { gt: "Musti" } }) {
            edges { node { id name } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

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
async fn string_lt() {
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

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { lt: "Naukio" } }) {
            edges { node { id name } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn string_gte() {
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

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { gte: "Musti" } }) {
            edges { node { id name } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn string_lte() {
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

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { lte: "Naukio" } }) {
            edges { node { id name } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn string_in() {
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

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { in: ["Musti", "Naukio"] } }) {
            edges { node { id name } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn string_nin() {
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

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { nin: ["Musti", "Naukio"] } }) {
            edges { node { id name } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": []
        }
      }
    }
    "#);
}

#[tokio::test]
async fn inet_in() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name inet NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, '0.0.0.0'), (2, '127.0.0.1')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { in: ["0.0.0.0", "127.0.0.1"] } }) {
            edges { node { id name } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "0.0.0.0"
              }
            },
            {
              "node": {
                "id": 2,
                "name": "127.0.0.1"
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
            INSERT INTO "User" (id, name, age) VALUES (1, 'Musti', 11), (2, 'Musti', 12)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { ALL: [
            { name: { eq: "Musti" } },
            { age: { eq: 11 } }
          ]}) {
            edges { node { id name age } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
            INSERT INTO "User" (id, name, age) VALUES (1, 'Musti', 12), (2, 'Naukio', 11)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { ANY: [
            { name: { eq: "Musti" } },
            { age: { eq: 11 } }
          ]}) {
            edges { node { id name age } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti",
                "age": 12
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
            INSERT INTO "User" (id, name, age) VALUES
              (1, 'Musti', 11),
              (2, 'Naukio', 12),
              (3, 'Pentti', 13)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { NONE: [
            { name: { eq: "Musti" } },
            { age: { eq: 13 } }
          ]}) {
            edges { node { id name age } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 2,
                "name": "Naukio",
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
async fn not() {
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
            INSERT INTO "User" (id, name, age) VALUES
              (1, 'Musti', 11),
              (2, 'Naukio', 12),
              (3, 'Pentti', 13)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { name: { not: { eq: "Pentti" } } }) {
            edges { node { id name age } }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn one_to_one() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Profile" (
                id INT PRIMARY KEY,
                user_id INT NULL UNIQUE,
                description TEXT NOT NULL,
                CONSTRAINT Profile_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
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
            INSERT INTO "Profile" (id, user_id, description) VALUES
              (1, 1, 'meowmeowmeow'),
              (2, 2, 'purrpurrpurr')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          users(first: 10) {
            edges {
              node {
                id
                name
                profile { description }
              }
            }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti",
                "profile": {
                  "description": "meowmeowmeow"
                }
              }
            },
            {
              "node": {
                "id": 2,
                "name": "Naukio",
                "profile": {
                  "description": "purrpurrpurr"
                }
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_one_relation_filter() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Profile" (
                id INT PRIMARY KEY,
                user_id INT NULL UNIQUE,
                description TEXT NOT NULL,
                CONSTRAINT Profile_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
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
            INSERT INTO "Profile" (id, user_id, description) VALUES
              (1, 1, 'meowmeowmeow'),
              (2, 2, 'purrpurrpurr')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          users(first: 10, filter: { profile: { description: { eq: "purrpurrpurr" } } }) {
            edges {
              node {
                id
                name
                profile { description }
              }
            }
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 2,
                "name": "Naukio",
                "profile": {
                  "description": "purrpurrpurr"
                }
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_many_child_side() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let blog_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
            )
        "#};

        api.execute_sql(blog_table).await;

        let insert_users = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES
              (1, 'Musti'),
              (2, 'Naukio')
        "#};

        api.execute_sql(insert_users).await;

        let insert_blogs = indoc! {r#"
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?'),
              (4, 2, 'Purr purr!')
        "#};

        api.execute_sql(insert_blogs).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          blogs(first: 10) {
            edges {
              node {
                id
                title
                user { id name }
              }
            }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "blogs": {
          "edges": [
            {
              "node": {
                "id": 1,
                "title": "Hello, world!",
                "user": {
                  "id": 1,
                  "name": "Musti"
                }
              }
            },
            {
              "node": {
                "id": 2,
                "title": "Sayonara...",
                "user": {
                  "id": 1,
                  "name": "Musti"
                }
              }
            },
            {
              "node": {
                "id": 3,
                "title": "Meow meow?",
                "user": {
                  "id": 2,
                  "name": "Naukio"
                }
              }
            },
            {
              "node": {
                "id": 4,
                "title": "Purr purr!",
                "user": {
                  "id": 2,
                  "name": "Naukio"
                }
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_many_relation_filter_child_side() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let blog_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
            )
        "#};

        api.execute_sql(blog_table).await;

        let insert_users = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES
              (1, 'Musti'),
              (2, 'Naukio')
        "#};

        api.execute_sql(insert_users).await;

        let insert_blogs = indoc! {r#"
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_blogs).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          blogs(first: 10, filter: { user: { id: { eq: 1 } } }) {
            edges {
              node {
                id
                title
                user { id name }
              }
            }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "blogs": {
          "edges": [
            {
              "node": {
                "id": 1,
                "title": "Hello, world!",
                "user": {
                  "id": 1,
                  "name": "Musti"
                }
              }
            },
            {
              "node": {
                "id": 2,
                "title": "Sayonara...",
                "user": {
                  "id": 1,
                  "name": "Musti"
                }
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_many_parent_side() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let blog_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
            )
        "#};

        api.execute_sql(blog_table).await;

        let insert_users = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES
              (1, 'Musti'),
              (2, 'Naukio')
        "#};

        api.execute_sql(insert_users).await;

        let insert_blogs = indoc! {r#"
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?'),
              (4, 2, 'Purr purr!')
        "#};

        api.execute_sql(insert_blogs).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(first: 10) {
            edges {
              node {
                id
                name
                blogs(first: 10) { edges { node { id title } } }
              }
            }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti",
                "blogs": {
                  "edges": [
                    {
                      "node": {
                        "id": 1,
                        "title": "Hello, world!"
                      }
                    },
                    {
                      "node": {
                        "id": 2,
                        "title": "Sayonara..."
                      }
                    }
                  ]
                }
              }
            },
            {
              "node": {
                "id": 2,
                "name": "Naukio",
                "blogs": {
                  "edges": [
                    {
                      "node": {
                        "id": 3,
                        "title": "Meow meow?"
                      }
                    },
                    {
                      "node": {
                        "id": 4,
                        "title": "Purr purr!"
                      }
                    }
                  ]
                }
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_many_relation_filter_parent_side() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let blog_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
            )
        "#};

        api.execute_sql(blog_table).await;

        let insert_users = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES
              (1, 'Musti'),
              (2, 'Naukio')
        "#};

        api.execute_sql(insert_users).await;

        let insert_blogs = indoc! {r#"
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_blogs).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          users(first: 10, filter: { blogs: { contains: { id: { eq: 1 } } } }) {
            edges {
              node {
                id
                name
                blogs(first: 10) { edges { node { id title } } }
              }
            }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti",
                "blogs": {
                  "edges": [
                    {
                      "node": {
                        "id": 1,
                        "title": "Hello, world!"
                      }
                    },
                    {
                      "node": {
                        "id": 2,
                        "title": "Sayonara..."
                      }
                    }
                  ]
                }
              }
            }
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn first_as_parameter() {
    use serde_json::json;

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
        query Pg($first: Int) {
          users(first: $first) {
            edges { node { id name } }
          }
        }
    "};

    let variables = json!({
        "first": 1
    });

    let response = runner.query(query).variables(variables).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn last_as_parameter() {
    use serde_json::json;

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
        query Pg($last: Int) {
          users(last: $last) {
            edges { node { id name } }
          }
        }
    "};

    let variables = json!({
        "last": 1
    });

    let response = runner.query(query).variables(variables).send().await;

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
