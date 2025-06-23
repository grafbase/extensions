use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn string_set_with_returning() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { name: { set: "Naukio" } }) {
            returning {
              id
              name
            }
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1,
            "name": "Naukio"
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            name
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "name": "Naukio"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn with_returning() {
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
                (2, 'Naukio', 11),
                (3, 'Pertti', 12)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdateMany(filter: { age: { eq: 11 } }, input: { age: { set: 10 } }) {
            returning {
              id
              name
              age
            }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdateMany": {
          "returning": [
            {
              "id": 1,
              "name": "Musti",
              "age": 10
            },
            {
              "id": 2,
              "name": "Naukio",
              "age": 10
            }
          ],
          "rowCount": 2
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(orderBy: [{ id: ASC }]) {
            edges { node { id name age } }
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti",
                "age": 10
              }
            },
            {
              "node": {
                "id": 2,
                "name": "Naukio",
                "age": 10
              }
            },
            {
              "node": {
                "id": 3,
                "name": "Pertti",
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
async fn no_returning() {
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
                (2, 'Naukio', 11),
                (3, 'Pertti', 12)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdateMany(filter: { age: { eq: 11 } }, input: { age: { set: 10 } }) {
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdateMany": {
          "rowCount": 2
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          users(orderBy: [{ id: ASC }]) {
            edges { node { id name age } }
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "users": {
          "edges": [
            {
              "node": {
                "id": 1,
                "name": "Musti",
                "age": 10
              }
            },
            {
              "node": {
                "id": 2,
                "name": "Naukio",
                "age": 10
              }
            },
            {
              "node": {
                "id": 3,
                "name": "Pertti",
                "age": 12
              }
            }
          ]
        }
      }
    }
    "#);
}
