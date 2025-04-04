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

    let response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

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

    let response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

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

    let response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

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

    let response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

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
