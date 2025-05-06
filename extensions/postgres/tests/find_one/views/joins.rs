use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn one_to_one_join_parent_side() {
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
                description TEXT NOT NULL
            )
        "#};

        api.execute_sql(profile_table).await;

        // Create a view based on the Profile table
        let profile_view = indoc! {r#"
            CREATE VIEW "ViewProfile" AS SELECT id, user_id, description FROM "Profile";
        "#};

        api.execute_sql(profile_view).await;

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

    let config = indoc! {r#"
        [schemas.public.views.ViewProfile.columns.id]
        unique = true
        nullable = false

        [schemas.public.views.ViewProfile.columns.user_id]
        unique = true
        nullable = true

        [schemas.public.views.ViewProfile.relations.view_profile_to_users]
        referenced_table = "User"
        referencing_columns = ["user_id"]
        referenced_columns = ["id"]
    "#};

    let runner = api.runner_spawn_with_config(config).await;

    // Query through the User -> ViewProfile relationship
    let query = indoc! {r"
        query {
          user(lookup: { id: 2 }) {
            id
            name
            viewProfile { description }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    // Assert the response, expecting the viewProfile field
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 2,
          "name": "Naukio",
          "viewProfile": {
            "description": "purrpurrpurr"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_one_join_between_schemas() {
    let api = PgTestApi::new("", |api| async move {
        let private_schema = indoc! {r#"
            CREATE SCHEMA "private";
        "#};
        api.execute_sql(private_schema).await;

        let public_table = indoc! {r#"
            CREATE TABLE "public"."User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};
        api.execute_sql(public_table).await;

        let private_table = indoc! {r#"
            CREATE TABLE "private"."User" (
                id INT PRIMARY KEY,
                secret_name VARCHAR(255) NOT NULL,
                user_id INT NULL UNIQUE
            );
        "#};
        api.execute_sql(private_table).await;

        // Create view for Secret
        let secret_view = indoc! {r#"
            CREATE VIEW "private"."ViewUser" AS SELECT id, secret_name, user_id FROM "private"."User";
        "#};
        api.execute_sql(secret_view).await;

        let insert_public = indoc! {r#"
            INSERT INTO "public"."User" (id, name) VALUES
              (1, 'Musti'),
              (2, 'Naukio')
        "#};
        api.execute_sql(insert_public).await;

        let insert_private = indoc! {r#"
            INSERT INTO "private"."User" (id, user_id, secret_name) VALUES
              (1, 1, 'Naukio'),
              (2, 2, 'Musti')
        "#};
        api.execute_sql(insert_private).await;
    })
    .await;

    let config = indoc! {r#"
        [schemas.private.views.ViewUser.columns.id]
        unique = true
        nullable = false

        [schemas.private.views.ViewUser.columns.secret_name]
        nullable = false

        [schemas.private.views.ViewUser.columns.user_id]
        unique = true
        nullable = true

        [schemas.private.views.ViewUser.relations.view_user_to_public_user]
        referenced_schema = "public"
        referenced_table = "User"
        referencing_columns = ["user_id"]
        referenced_columns = ["id"]
    "#};

    let runner = api.runner_spawn_with_config(config).await;

    let query = indoc! {r#"
        query {
          publicUser(lookup: { id: 1 }) {
            id
            name
            viewUser { secretName } # Query the view relation
          }
        }
    "#};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    // Expect viewSecret field
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "publicUser": {
          "id": 1,
          "name": "Musti",
          "viewUser": {
            "secretName": "Naukio"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_many_join_between_schemas() {
    let api = PgTestApi::new("", |api| async move {
        let private_schema = indoc! {r#"
            CREATE SCHEMA "private";
        "#};

        api.execute_sql(private_schema).await;

        let public_table = indoc! {r#"
            CREATE TABLE "public"."User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(public_table).await;

        let private_table = indoc! {r#"
            CREATE TABLE "private"."User" (
                id INT PRIMARY KEY,
                secret_name VARCHAR(255) NOT NULL,
                user_id INT NULL
            );
        "#};

        api.execute_sql(private_table).await;

        // Create view based on the private User table
        let private_view = indoc! {r#"
            CREATE VIEW "private"."ViewPrivateUser" AS SELECT id, secret_name, user_id FROM "private"."User";
        "#};

        api.execute_sql(private_view).await;

        let insert_public = indoc! {r#"
            INSERT INTO "public"."User" (id, name) VALUES
              (1, 'Musti'),
              (2, 'Naukio')
        "#};

        api.execute_sql(insert_public).await;

        let insert_private = indoc! {r#"
            INSERT INTO "private"."User" (id, user_id, secret_name) VALUES
              (1, 1, 'Naukio'),
              (2, 1, 'Musti'),
              (3, 2, 'Pertti'),
              (4, 2, 'Matti')
        "#};

        api.execute_sql(insert_private).await;
    })
    .await;

    let config = indoc! {r#"
        [schemas.private.views.ViewPrivateUser.columns.id]
        unique = true
        nullable = false

        [schemas.private.views.ViewPrivateUser.columns.secret_name]
        nullable = false

        [schemas.private.views.ViewPrivateUser.columns.user_id]
        nullable = true

        [schemas.private.views.ViewPrivateUser.relations.view_private_user_to_public_user]
        referenced_schema = "public"
        referenced_table = "User"
        referencing_columns = ["user_id"]
        referenced_columns = ["id"]
    "#};

    let runner = api.runner_spawn_with_config(config).await;

    let query = indoc! {r"
        query {
          publicUser(lookup: { id: 1 }) {
            id
            name
            viewPrivateUsers { edges { node { secretName } } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "publicUser": {
          "id": 1,
          "name": "Musti",
          "viewPrivateUsers": {
            "edges": [
              {
                "node": {
                  "secretName": "Naukio"
                }
              },
              {
                "node": {
                  "secretName": "Musti"
                }
              }
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_one_join_child_side() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        // Create a view based on the User table
        let user_view = indoc! {r#"
            CREATE VIEW "ViewUser" AS SELECT id, name FROM "User";
        "#};

        api.execute_sql(user_view).await;

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

    let config = indoc! {r#"
        [schemas.public.views.ViewUser.columns.id]
        unique = true
        nullable = false

        [schemas.public.views.ViewUser.columns.name]
        nullable = false

        [schemas.public.views.ViewUser.relations.view_user_to_profile]
        referenced_table = "Profile"
        referencing_columns = ["id"]
        referenced_columns = ["user_id"]
    "#};

    let runner = api.runner_spawn_with_config(config).await;

    let query = indoc! {r"
        query {
          profile(lookup: { id: 2 }) {
            description
            viewUser {
              id
              name
            }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "profile": {
          "description": "purrpurrpurr",
          "viewUser": {
            "id": 2,
            "name": "Naukio"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_one_join_parent_side_compound_fk() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                CONSTRAINT User_name_email_pk PRIMARY KEY (name, email)
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Profile" (
                user_name VARCHAR(255) NULL,
                user_email VARCHAR(255) NULL,
                description TEXT NOT NULL,
                CONSTRAINT Profile_name_email_key UNIQUE (user_name, user_email)
            )
        "#};

        api.execute_sql(profile_table).await;

        // Create view based on the Profile table
        let profile_view = indoc! {r#"
            CREATE VIEW "ViewProfile" AS SELECT user_name, user_email, description FROM "Profile";
        "#};

        api.execute_sql(profile_view).await;

        let insert_users = indoc! {r#"
            INSERT INTO "User" (name, email) VALUES
              ('Musti', 'meow1@hotmail.com'),
              ('Musti', 'meow2@hotmail.com')
        "#};

        api.execute_sql(insert_users).await;

        let insert_profiles = indoc! {r#"
            INSERT INTO "Profile" (user_name, user_email, description) VALUES
              ('Musti', 'meow1@hotmail.com', 'meowmeowmeow'),
              ('Musti', 'meow2@hotmail.com', 'purrpurrpurr')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let config = indoc! {r#"
        [schemas.public.views.ViewProfile]
        unique_keys = [["user_name", "user_email"]]

        [schemas.public.views.ViewProfile.relations.view_profile_to_user]
        referenced_table = "User"
        referencing_columns = ["user_name", "user_email"]
        referenced_columns = ["name", "email"]
    "#};

    let runner = api.runner_spawn_with_config(config).await;

    let query = indoc! {r#"
        query {
          user(lookup: { nameEmail: { name: "Musti", email: "meow2@hotmail.com" } }) {
            name
            email
            viewProfile { description }
          }
        }
    "#};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Musti",
          "email": "meow2@hotmail.com",
          "viewProfile": {
            "description": "purrpurrpurr"
          }
        }
      }
    }
    "#);
}
