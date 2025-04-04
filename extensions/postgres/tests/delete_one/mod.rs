use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn single_pk() {
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

    let mutation = indoc! {r"
        mutation {
          userDelete(lookup: { id: 1 }) {
            returning { id name }
            rowCount
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
        "userDelete": {
          "returning": {
            "id": 1,
            "name": "Musti"
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) { id }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": null
      }
    }
    "#);
}

#[tokio::test]
async fn single_pk_not_returning() {
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

    let mutation = indoc! {r"
        mutation {
          userDelete(lookup: { id: 1 }) {
            rowCount
          }
        }
    "};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) { id }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDelete": {
          "rowCount": 1
        }
      }
    }
    "#);

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": null
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
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userDelete(lookup: { id: 3 }) { returning { id name } rowCount }
        }
    "};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) { id name }
        }
    "};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDelete": {
          "returning": null,
          "rowCount": 0
        }
      }
    }
    "#);

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "name": "Musti"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn single_unique() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL UNIQUE
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

    let mutation = indoc! {r#"
        mutation {
          userDelete(lookup: { name: "Musti" }) { returning { id name } }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    let query = indoc! {r#"
        query {
          user(lookup: { name: "Musti" }) { id name }
        }
    "#};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDelete": {
          "returning": {
            "id": 1,
            "name": "Musti"
          }
        }
      }
    }
    "#);

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": null
      }
    }
    "#);
}

#[tokio::test]
async fn composite_pk() {
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
            INSERT INTO "User" (name, email) VALUES ('Musti', 'meow@example.com'), ('Musti', 'purr@example.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDelete(lookup: { nameEmail: { name: "Musti", email: "purr@example.com" } }) {
            returning { name email }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    let query = indoc! {r#"
        query {
          user(lookup: { nameEmail: { name: "Musti", email: "meow@example.com" } }) { name email }
        }
    "#};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDelete": {
          "returning": {
            "name": "Musti",
            "email": "purr@example.com"
          }
        }
      }
    }
    "#);

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "name": "Musti",
          "email": "meow@example.com"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn composite_key_with_nulls() {
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
            INSERT INTO "User" (name, email) VALUES ('Musti', 'meow@example.com'), ('Musti', NULL)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDelete(lookup: { nameEmail: { name: "Musti" } }) {
            returning { name email }
          }
        }
    "#};

    let mutation_response = runner
        .graphql_query::<serde_json::Value>(mutation)
        .send()
        .await
        .unwrap();

    let query = indoc! {r#"
        query {
          user(lookup: { nameEmail: { name: "Musti" }}) { name email }
        }
    "#};

    let query_response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userDelete": {
          "returning": {
            "name": "Musti",
            "email": null
          }
        }
      }
    }
    "#);

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": null
      }
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

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, ARRAY['red', 'green']::street_light[])
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userDelete(lookup: { id: 1 }) {
            returning { id val }
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
        "userDelete": {
          "returning": {
            "id": 1,
            "val": [
              "RED",
              "GREEN"
            ]
          }
        }
      }
    }
    "#);
}
