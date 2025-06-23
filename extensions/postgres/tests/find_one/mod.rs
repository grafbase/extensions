mod joins;
mod types;
mod views;

use indoc::indoc;

use crate::PgTestApi;

#[tokio::test]
async fn by_pk_no_rename() {
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
          user(lookup: { id: 1 }) { id name }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn by_pk_with_rename() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id_field INT PRIMARY KEY,
                name_field VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "users" (id_field, name_field) VALUES (1, 'Musti'), (2, 'Naukio')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { idField: 1 }) { idField nameField }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "idField": 1,
          "nameField": "Musti"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn by_compound_pk() {
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
              ('Musti', 'meow@meow.com'),
              ('Naukio', 'purr@meow.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          user(lookup: { nameEmail: { name: "Naukio", email: "purr@meow.com" } }) {
            name
            email
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Naukio",
          "email": "purr@meow.com"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn by_compound_unique_with_nullable_column() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NULL,
                CONSTRAINT "User_pkey" UNIQUE (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "users" (name, email) VALUES
              ('Musti', 'meow@meow.com'),
              ('Naukio', NULL),
              ('Naukio', 'purr@meow.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          user(lookup: { nameEmail: { name: "Naukio", email: null } }) {
            name
            email
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Naukio",
          "email": null
        }
      }
    }
    "#);
}

#[tokio::test]
async fn by_compound_unique_with_nullable_column_emitting_field() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NULL,
                CONSTRAINT "User_pkey" UNIQUE (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (name, email) VALUES
              ('Musti', 'meow@meow.com'),
              ('Naukio', NULL),
              ('Naukio', 'purr@meow.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          user(lookup: { nameEmail: { name: "Naukio" } }) {
            name
            email
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Naukio",
          "email": null
        }
      }
    }
    "#);
}

#[tokio::test]
async fn by_unique() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                email VARCHAR(255) NOT NULL UNIQUE
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, email) VALUES
              (1, 'meow@meow.com'),
              (2, 'purr@meow.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          user(lookup: { email: "purr@meow.com" }) {
            id
            email
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 2,
          "email": "purr@meow.com"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn by_id_when_having_another_unique() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                email VARCHAR(255) NOT NULL UNIQUE
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, email) VALUES
              (1, 'meow@meow.com'),
              (2, 'purr@meow.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 2 }) {
            id
            email
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 2,
          "email": "purr@meow.com"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn by_compound_unique() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                CONSTRAINT User_name_email_key UNIQUE (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name, email) VALUES
              (1, 'Musti', 'meow@meow.com'),
              (2, 'Naukio', 'purr@meow.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          user(lookup: { nameEmail: { name: "Naukio", email: "purr@meow.com" } }) {
            id
            name
            email
          }
        }
    "#};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 2,
          "name": "Naukio",
          "email": "purr@meow.com"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn mtls() {
    let api = PgTestApi::new_mtls("", |api| async move {
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
          user(lookup: { id: 1 }) { id name }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
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
async fn disabling_queries() {
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

    let config = indoc! {r#"
        enable_queries = false
    "#};

    let runner = api.runner_spawn_with_config(config).await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) { id name }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "errors": [
        {
          "message": "Query does not have a field named 'user'.",
          "locations": [
            {
              "line": 2,
              "column": 3
            }
          ],
          "extensions": {
            "code": "OPERATION_VALIDATION_ERROR"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn alias_column() {
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
          user(lookup: { id: 1 }) {
            id
            userName: name
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "userName": "Musti"
        }
      }
    }
    "#);
}
