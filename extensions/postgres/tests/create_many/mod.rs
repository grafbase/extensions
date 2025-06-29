use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn two_identity_by_default() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY GENERATED BY DEFAULT AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreateMany(input: [{ id: 7 }, { id: 8 }]) {
            returning { id }
            rowCount
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreateMany": {
          "returning": [
            {
              "id": 7
            },
            {
              "id": 8
            }
          ],
          "rowCount": 2
        }
      }
    }
    "#);
}

#[tokio::test]
async fn two_identity_always() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreateMany(input: [{}, {}]) {
            returning { id }
            rowCount
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreateMany": {
          "returning": [
            {
              "id": 1
            },
            {
              "id": 2
            }
          ],
          "rowCount": 2
        }
      }
    }
    "#);
}

#[tokio::test]
async fn two_pk_ids() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreateMany(input: [{ id: 1 }, { id: 2 }]) {
            returning { id }
            rowCount
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreateMany": {
          "returning": [
            {
              "id": 1
            },
            {
              "id": 2
            }
          ],
          "rowCount": 2
        }
      }
    }
    "#);
}

#[tokio::test]
async fn two_pk_ids_no_returning() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreateMany(input: [{ id: 1 }, { id: 2 }]) {
            rowCount
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreateMany": {
          "rowCount": 2
        }
      }
    }
    "#);
}

#[tokio::test]
async fn wrong_keys() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(5) NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreateMany(input: [{ id: 1 }, { id: 2, name: "Musti" }]) {
            returning { id }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": null,
      "errors": [
        {
          "message": "All insert items must have the same columns.",
          "locations": [
            {
              "line": 2,
              "column": 3
            }
          ],
          "path": [
            "userCreateMany"
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
async fn enum_array() {
    let api = PgTestApi::new("", |api| async move {
        let r#type = indoc! {r"
            CREATE TYPE street_light AS ENUM ('red', 'yellow', 'green');
        "};

        api.execute_sql(r#type).await;

        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val street_light[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreateMany(input: [
            { val: [YELLOW, GREEN] },
            { val: [RED] }
          ]) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreateMany": {
          "returning": [
            {
              "val": [
                "YELLOW",
                "GREEN"
              ]
            },
            {
              "val": [
                "RED"
              ]
            }
          ]
        }
      }
    }
    "#);
}
