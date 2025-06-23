mod types;

use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn pk_explicit_int() {
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
          userCreate(input: { id: 1 }) {
            returning {
              id
            }
            rowCount
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);
}

#[tokio::test]
async fn pk_explicit_int_aliased() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY,
                full_name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { id: 1, fullName: "John Doe" }) {
            returning {
              aliased: id
              fullName
            }
            rowCount
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "aliased": 1,
            "fullName": "John Doe"
          },
          "rowCount": 1
        }
      }
    }
    "#);
}

#[tokio::test]
async fn pk_explicit_int_no_returning() {
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
          userCreate(input: { id: 1 }) {
            rowCount
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "rowCount": 1
        }
      }
    }
    "#);
}

#[tokio::test]
async fn renamed() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id_field INT PRIMARY KEY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { idField: 1 }) {
            returning { idField }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "idField": 1
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn serial_id() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id_field SERIAL PRIMARY KEY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: {}) {
            returning { idField }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "idField": 1
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn disabling_mutations() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        enable_mutations = false
    "#};

    let runner = api.runner_spawn_with_config(config).await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { id: 1 }) {
            returning {
              id
            }
            rowCount
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Mutations are not defined on this schema.",
          "extensions": {
            "code": "OPERATION_VALIDATION_ERROR"
          }
        }
      ]
    }
    "#);
}
