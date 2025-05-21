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

    let query = indoc! {r"
        query {
          user(lookup: { id: 2 }) {
            id
            name
            profile { description }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 2,
          "name": "Naukio",
          "profile": {
            "description": "purrpurrpurr"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn aliased_one_to_one_join_parent_side() {
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

    let query = indoc! {r"
        query {
          user(lookup: { id: 2 }) {
            id
            name
            aliased: profile { description }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 2,
          "name": "Naukio",
          "aliased": {
            "description": "purrpurrpurr"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_one_join_parent_side_null() {
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
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 2 }) {
            id
            name
            profile { description }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 2,
          "name": "Naukio",
          "profile": null
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
            CREATE TABLE "private"."Secret" (
                id INT PRIMARY KEY,
                secret_name VARCHAR(255) NOT NULL,
                user_id INT NULL UNIQUE,
                CONSTRAINT User_User_fkey FOREIGN KEY (user_id) REFERENCES "public"."User" (id)
            );
        "#};

        api.execute_sql(private_table).await;

        let insert_public = indoc! {r#"
            INSERT INTO "public"."User" (id, name) VALUES
              (1, 'Musti'),
              (2, 'Naukio')
        "#};

        api.execute_sql(insert_public).await;

        let insert_private = indoc! {r#"
            INSERT INTO "private"."Secret" (id, user_id, secret_name) VALUES
              (1, 1, 'Naukio'),
              (2, 2, 'Musti')
        "#};

        api.execute_sql(insert_private).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            name
            secret { secretName }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "name": "Musti",
          "secret": {
            "secretName": "Naukio"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_one_join_between_schemas_using_duplicate_table_names() {
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
                user_id INT NULL UNIQUE,
                CONSTRAINT User_User_fkey FOREIGN KEY (user_id) REFERENCES "public"."User" (id)
            );
        "#};

        api.execute_sql(private_table).await;

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

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          publicUser(lookup: { id: 1 }) {
            id
            name
            privateUser { secretName }
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
          "privateUser": {
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
                user_id INT NULL,
                CONSTRAINT User_User_fkey FOREIGN KEY (user_id) REFERENCES "public"."User" (id)
            );
        "#};

        api.execute_sql(private_table).await;

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

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          publicUser(lookup: { id: 1 }) {
            id
            name
            privateUsers(first: 1000) { edges { node { secretName } } }
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
          "privateUsers": {
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
                CONSTRAINT Profile_name_email_key UNIQUE (user_name, user_email),
                CONSTRAINT Profile_User_fkey FOREIGN KEY (user_name, user_email) REFERENCES "User" (name, email)
            )
        "#};

        api.execute_sql(profile_table).await;

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

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          user(lookup: { nameEmail: { name: "Musti", email: "meow2@hotmail.com" } }) {
            name
            email
            profile { description }
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
          "profile": {
            "description": "purrpurrpurr"
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

    let query = indoc! {r"
        query {
          profile(lookup: { id: 2 }) {
            description
            user { id name }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "profile": {
          "description": "purrpurrpurr",
          "user": {
            "id": 2,
            "name": "Naukio"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_one_to_one_join() {
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

        let extra_table = indoc! {r#"
            CREATE TABLE "Extra" (
                id INT PRIMARY KEY,
                profile_id INT NULL UNIQUE,
                number int NOT NULL,
                CONSTRAINT Extra_Profile_fkey FOREIGN KEY (profile_id) REFERENCES "Profile" (id)
            )
        "#};

        api.execute_sql(extra_table).await;

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

        let insert_extras = indoc! {r#"
            INSERT INTO "Extra" (id, profile_id, number) VALUES
              (1, 1, 420),
              (2, 2, 666)
        "#};

        api.execute_sql(insert_extras).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 2 }) {
            id
            name
            profile { description extra { number } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 2,
          "name": "Naukio",
          "profile": {
            "description": "purrpurrpurr",
            "extra": {
              "number": 666
            }
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_many_join_child_side() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
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
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          blog(lookup: { id: 2 }) {
            title
            user { id name }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "blog": {
          "title": "Sayonara...",
          "user": {
            "id": 1,
            "name": "Musti"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn one_to_many_join_parent_side() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
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
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            name
            blogs(first: 10000) { edges { node { id title } } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
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
    }
    "#);
}

#[tokio::test]
async fn nested_one_to_many_joins_parent_side() {
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

        let post_table = indoc! {r#"
            CREATE TABLE "Post" (
                id INT PRIMARY KEY,
                blog_id INT NOT NULL,
                content TEXT NOT NULL,
                CONSTRAINT Post_Blog_fkey FOREIGN KEY (blog_id) REFERENCES "Blog" (id)
            )
        "#};

        api.execute_sql(post_table).await;

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

        let insert_posts = indoc! {r#"
            INSERT INTO "Post" (id, blog_id, content) VALUES
              (1, 1, 'meowmeow'),
              (2, 2, 'uwuwuwuwu'),
              (3, 3, 'Meow meow?')
        "#};

        api.execute_sql(insert_posts).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            name
            blogs(first: 1000) {
              edges {
                node {
                  id
                  title
                  posts(first: 1000) {
                    edges {
                      node {
                        id
                        content
                      }
                    }
                  }
                }
              }
            }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Musti",
          "blogs": {
            "edges": [
              {
                "node": {
                  "id": 1,
                  "title": "Hello, world!",
                  "posts": {
                    "edges": [
                      {
                        "node": {
                          "id": 1,
                          "content": "meowmeow"
                        }
                      }
                    ]
                  }
                }
              },
              {
                "node": {
                  "id": 2,
                  "title": "Sayonara...",
                  "posts": {
                    "edges": [
                      {
                        "node": {
                          "id": 2,
                          "content": "uwuwuwuwu"
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
    }
    "#);
}

#[tokio::test]
async fn one_to_many_join_parent_side_with_first() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
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
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            name
            blogs(first: 1) { edges { node { id title } } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Musti",
          "blogs": {
            "edges": [
              {
                "node": {
                  "id": 1,
                  "title": "Hello, world!"
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
async fn one_to_many_join_parent_side_with_first_aliased() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
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
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            name
            aliased: blogs(first: 1) { edges { node { id aliased: title } } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Musti",
          "aliased": {
            "edges": [
              {
                "node": {
                  "id": 1,
                  "aliased": "Hello, world!"
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
async fn one_to_many_join_parent_side_with_last() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
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
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            name
            blogs(last: 1) { edges { node { id title } } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Musti",
          "blogs": {
            "edges": [
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
    }
    "#);
}

#[tokio::test]
async fn one_to_many_join_parent_side_with_single_column_descending_order() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
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
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            name
            blogs(first: 10, orderBy: [{ id: DESC }]) { edges { node { id title } } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Musti",
          "blogs": {
            "edges": [
              {
                "node": {
                  "id": 2,
                  "title": "Sayonara..."
                }
              },
              {
                "node": {
                  "id": 1,
                  "title": "Hello, world!"
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
async fn one_to_many_join_parent_side_with_compound_column_ordering_with_last() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                description VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
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
            INSERT INTO "Blog" (id, user_id, description, title) VALUES
              (1, 1, 'a', 'a'),
              (2, 1, 'a', 'b'),
              (3, 1, 'b', 'c')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            name
            blogs(last: 2, orderBy: [{ description: DESC }, { title: DESC }]) {
              edges {
                node {
                  id
                  description
                  title
                }
              }
            }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Musti",
          "blogs": {
            "edges": [
              {
                "node": {
                  "id": 2,
                  "description": "a",
                  "title": "b"
                }
              },
              {
                "node": {
                  "id": 1,
                  "description": "a",
                  "title": "a"
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
async fn one_to_many_join_parent_side_with_single_column_descending_order_with_last() {
    let api = PgTestApi::new("", |api| async move {
        let user_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );
        "#};

        api.execute_sql(user_table).await;

        let profile_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
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
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            name
            blogs(last: 1, orderBy: [{ id: DESC }]) { edges { node { id title } } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "name": "Musti",
          "blogs": {
            "edges": [
              {
                "node": {
                  "id": 1,
                  "title": "Hello, world!"
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
async fn two_one_to_many_joins_parent_side() {
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

        let cat_table = indoc! {r#"
            CREATE TABLE "Cat" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                name VARCHAR(255) NOT NULL,
                CONSTRAINT Cat_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
            )
        "#};

        api.execute_sql(cat_table).await;

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

        let insert_cats = indoc! {r#"
            INSERT INTO "Cat" (id, user_id, name) VALUES
              (1, 1, 'Musti'),
              (2, 1, 'Naukio'),
              (3, 2, 'Pertti')
        "#};

        api.execute_sql(insert_cats).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            name
            blogs(first: 1000) { edges { node { id title } } }
            cats(first: 100) { edges { node { id name } } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
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
          },
          "cats": {
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
    }
    "#);
}

#[tokio::test]
async fn one_to_one_with_one_to_many_joins_parent_side() {
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

        let insert_blogs = indoc! {r#"
            INSERT INTO "Blog" (id, user_id, title) VALUES
              (1, 1, 'Hello, world!'),
              (2, 1, 'Sayonara...'),
              (3, 2, 'Meow meow?')
        "#};

        api.execute_sql(insert_blogs).await;

        let insert_profiles = indoc! {r#"
            INSERT INTO "Profile" (id, user_id, description) VALUES
              (1, 1, 'meow'),
              (2, 2, 'uwu')
        "#};

        api.execute_sql(insert_profiles).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            name
            blogs(first: 10) { edges { node { id title } } }
            profile { description }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
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
          },
          "profile": {
            "description": "meow"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn two_foreign_keys_to_same_table() {
    let api = PgTestApi::new("", |api| async move {
        let setup = [
            r#"CREATE TABLE public.colors (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                name VARCHAR(255) NOT NULL UNIQUE,
                rgb INT NOT NULL
            );"#,
            r#"CREATE TABLE public.users (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                name VARCHAR(255) NOT NULL UNIQUE,
                favorite_color_id INT NOT NULL REFERENCES colors(id),
                least_favorite_color_id INT REFERENCES colors(id)
            );"#,
            r#"INSERT INTO public.colors (name, rgb) VALUES
              ('rebeccapurple',  0x663399),
              ('tomato',         0xff6347),
              ('skyblue',        0x87ceeb),
              ('maroon',         0x800000);"#,
            r#"INSERT INTO public.users (name, favorite_color_id, least_favorite_color_id) VALUES
              ('Guignol', 1, 2),
              ('Gnafron', 2, NULL),
              ('Flageolet', 3, 2),
              ('Canezou', 3, 4);
             "#,
        ];

        for statement in setup {
            api.execute_sql(statement).await;
        }
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r#"
        query {
          gnafron: user(lookup: { name: "Gnafron" }) {
            name
            colorByFavoriteColorId { id name rgb }
            colorByLeastFavoriteColorId { id name rgb }
          }
          guignol: user(lookup: { name: "Guignol" }) {
            name
            colorByFavoriteColorId { id name rgb }
            colorByLeastFavoriteColorId { id name rgb }
          }
          color(lookup: { name: "tomato" }) {
              id
              usersByFavoriteColorId(first: 10) { edges { node { id name favoriteColorId leastFavoriteColorId } } }
              usersByLeastFavoriteColorId(first: 10) { edges { node { id name favoriteColorId leastFavoriteColorId } } }
          }
        }
    "#};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "gnafron": {
              "name": "Gnafron",
              "colorByFavoriteColorId": {
                "id": 2,
                "name": "tomato",
                "rgb": 16737095
              },
              "colorByLeastFavoriteColorId": null
            },
            "guignol": {
              "name": "Guignol",
              "colorByFavoriteColorId": {
                "id": 1,
                "name": "rebeccapurple",
                "rgb": 6697881
              },
              "colorByLeastFavoriteColorId": {
                "id": 2,
                "name": "tomato",
                "rgb": 16737095
              }
            },
            "color": {
              "id": 2,
              "usersByFavoriteColorId": {
                "edges": [
                  {
                    "node": {
                      "id": 2,
                      "name": "Gnafron",
                      "favoriteColorId": 2,
                      "leastFavoriteColorId": null
                    }
                  }
                ]
              },
              "usersByLeastFavoriteColorId": {
                "edges": [
                  {
                    "node": {
                      "id": 1,
                      "name": "Guignol",
                      "favoriteColorId": 1,
                      "leastFavoriteColorId": 2
                    }
                  },
                  {
                    "node": {
                      "id": 3,
                      "name": "Flageolet",
                      "favoriteColorId": 3,
                      "leastFavoriteColorId": 2
                    }
                  }
                ]
              }
            }
          }
        }
    "#);
}
