mod joins;

use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn by_pk_no_rename() {
    let api = PgTestApi::new("", |api| async move {
        let create_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT UNIQUE NOT NULL
            )
        "#};

        api.execute_sql(create_table).await;

        let create_view = indoc! {r#"
            CREATE VIEW "filtered_users" AS
            SELECT id FROM "User" WHERE id < 3;
        "#};

        api.execute_sql(create_view).await;

        let insert_users = indoc! {r#"
            INSERT INTO "User" (id) VALUES (1), (2), (3), (4), (5);
        "#};

        api.execute_sql(insert_users).await;
    })
    .await;

    let config = indoc! {r#"
        [schemas.public.views.filtered_users.columns.id]
        unique = true
        nullable = false
    "#};

    let runner = api.runner_spawn_with_config(config).await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 5 }) { id }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 5
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          filteredUser(lookup: { id: 5 }) { id }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "filteredUser": null
      }
    }
    "#);

    let query = indoc! {r"
        query {
          filteredUser(lookup: { id: 1 }) { id }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "filteredUser": {
          "id": 1
        }
      }
    }
    "#);
}

#[tokio::test]
async fn by_composite_unique() {
    let api = PgTestApi::new("", |api| async move {
        let create_table = indoc! {r#"
            CREATE TABLE "User" (
                email VARCHAR(255) NOT NULL,
                full_name VARCHAR(255) NOT NULL,
                PRIMARY KEY (email, full_name)
            )
        "#};

        api.execute_sql(create_table).await;

        let create_view = indoc! {r#"
            CREATE VIEW "filtered_users" AS
            SELECT email, full_name FROM "User"
        "#};

        api.execute_sql(create_view).await;

        let insert_users = indoc! {r#"
            INSERT INTO "User" (email, full_name) VALUES
                ('user1@example.com', 'User One'),
                ('user2@example.com', 'User Two'),
                ('user3@example.com', 'User Three'),
                ('user4@example.com', 'User Four'),
                ('user5@example.com', 'User Five');
        "#};

        api.execute_sql(insert_users).await;
    })
    .await;

    let config = indoc! {r#"
        [schemas.public.views.filtered_users]
        unique_keys = [["email", "full_name"]]
    "#};

    let runner = api.runner_spawn_with_config(config).await;

    let query = indoc! {r#"
        query {
          filteredUser(lookup: {
            emailFullName: {
              email: "user1@example.com",
              fullName: "User One"
            }
          }) {
            email
            fullName
          }
        }
    "#};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "filteredUser": {
          "email": "user1@example.com",
          "fullName": "User One"
        }
      }
    }
    "#);
}
