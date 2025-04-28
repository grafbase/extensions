use crate::PgTestApi;
use grafbase_sdk::test::DynamicSchema;
use indoc::indoc;
use serde_json::json;

#[tokio::test]
async fn lookup_int_pk() {
    let mock_sdl = indoc! {r#"
        extend schema
            @link(url: "https://specs.grafbase.com/composite-schema/v1", import: ["@lookup", "@key"])

        type User @key(fields: "id") {
          id: Int!
          age: Int!
        }

        type Query {
          userFoobar: [User!]!
        }
    "#};

    let response = json!([
        {
            "id": 1,
            "age": 13
        },
        {
            "id": 3,
            "age": 30
        }
    ]);

    let subgraph = DynamicSchema::builder(mock_sdl)
        .with_resolver("Query", "userFoobar", response)
        .into_subgraph("mock")
        .unwrap();

    let api = PgTestApi::new_with_subgraphs("", vec![subgraph], |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Pentti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          userFoobar {
            id name age
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userFoobar": [
          {
            "id": 1,
            "name": "Musti",
            "age": 13
          },
          {
            "id": 3,
            "name": "Pentti",
            "age": 30
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn lookup_unique_string() {
    let mock_sdl = indoc! {r#"
        extend schema
            @link(url: "https://specs.grafbase.com/composite-schema/v1", import: ["@lookup", "@key"])

        type User @key(fields: "email") {
          email: String!
          age: Int!
        }

        type Query {
          userFoobar: [User!]!
        }
    "#};

    let response = json!([
        {
            "email": "foo@example.com",
            "age": 13
        },
        {
            "email": "bar@example.com",
            "age": 12
        }
    ]);

    let subgraph = DynamicSchema::builder(mock_sdl)
        .with_resolver("Query", "userFoobar", response)
        .into_subgraph("mock")
        .unwrap();

    let api = PgTestApi::new_with_subgraphs("", vec![subgraph], |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                email VARCHAR(255) UNIQUE NOT NULL,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (email, name) VALUES ('foo@example.com', 'Musti'), ('bar@example.com', 'Naukio'), ('baz@example.com', 'Pentti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          userFoobar {
            email name age
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userFoobar": [
          {
            "email": "foo@example.com",
            "name": "Musti",
            "age": 13
          },
          {
            "email": "bar@example.com",
            "name": "Naukio",
            "age": 12
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn with_one_to_many_join() {
    let mock_sdl = indoc! {r#"
        extend schema
            @link(url: "https://specs.grafbase.com/composite-schema/v1", import: ["@lookup", "@key"])

        type User @key(fields: "id") {
          id: Int!
          age: Int!
        }

        type Query {
          userFoobar: [User!]!
        }
    "#};

    let response = json!([
        {
            "id": 3,
            "age": 13
        },
        {
            "id": 1,
            "age": 12
        }
    ]);

    let subgraph = DynamicSchema::builder(mock_sdl)
        .with_resolver("Query", "userFoobar", response)
        .into_subgraph("mock")
        .unwrap();

    let api = PgTestApi::new_with_subgraphs("", vec![subgraph], |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY NOT NULL,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "User" (id)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Pentti')
        "#};

        api.execute_sql(insert).await;

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
          userFoobar {
            id
            name
            age
            blogs { edges { node { id } } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userFoobar": [
          {
            "id": 3,
            "name": "Pentti",
            "age": 13,
            "blogs": {
              "edges": []
            }
          },
          {
            "id": 1,
            "name": "Musti",
            "age": 12,
            "blogs": {
              "edges": [
                {
                  "node": {
                    "id": 1
                  }
                },
                {
                  "node": {
                    "id": 2
                  }
                }
              ]
            }
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn with_one_to_many_join_renamed() {
    let mock_sdl = indoc! {r#"
        extend schema
            @link(url: "https://specs.grafbase.com/composite-schema/v1", import: ["@lookup", "@key"])

        type User @key(fields: "id") {
          id: Int!
          age: Int!
        }

        type Query {
          userFoobar: [User!]!
        }
    "#};

    let response = json!([
        {
            "id": 3,
            "age": 13
        },
        {
            "id": 1,
            "age": 12
        }
    ]);

    let subgraph = DynamicSchema::builder(mock_sdl)
        .with_resolver("Query", "userFoobar", response)
        .into_subgraph("mock")
        .unwrap();

    let api = PgTestApi::new_with_subgraphs("", vec![subgraph], |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY NOT NULL,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "blogs" (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title VARCHAR(255) NOT NULL,
                CONSTRAINT Blog_User_fkey FOREIGN KEY (user_id) REFERENCES "users" (id)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "users" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Pentti')
        "#};

        api.execute_sql(insert).await;

        let insert_profiles = indoc! {r#"
            INSERT INTO "blogs" (id, user_id, title) VALUES
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
          userFoobar {
            id
            name
            age
            blogs { edges { node { id } } }
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userFoobar": [
          {
            "id": 3,
            "name": "Pentti",
            "age": 13,
            "blogs": {
              "edges": []
            }
          },
          {
            "id": 1,
            "name": "Musti",
            "age": 12,
            "blogs": {
              "edges": [
                {
                  "node": {
                    "id": 1
                  }
                },
                {
                  "node": {
                    "id": 2
                  }
                }
              ]
            }
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn with_composite_key() {
    let mock_sdl = indoc! {r#"
        extend schema
            @link(url: "https://specs.grafbase.com/composite-schema/v1", import: ["@lookup", "@key"])

        type User @key(fields: "name email") {
          name: String!
          email: String!
          age: Int!
        }

        type Query {
          userFoobar: [User!]!
        }
    "#};

    let response = json!([
        {
            "name": "bob",
            "email": "bob@example.com",
            "age": 25
        },
        {
            "name": "alice",
            "email": "alice@example.com",
            "age": 34
        }
    ]);

    let subgraph = DynamicSchema::builder(mock_sdl)
        .with_resolver("Query", "userFoobar", response)
        .into_subgraph("mock")
        .unwrap();

    let api = PgTestApi::new_with_subgraphs("", vec![subgraph], |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                address VARCHAR(255) NOT NULL,
                PRIMARY KEY (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (name, email, address) VALUES
              ('alice', 'alice@example.com', '123 Main St'),
              ('bob', 'bob@example.com', '456 Elm St')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          userFoobar {
            name
            email
            age
            address
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userFoobar": [
          {
            "name": "bob",
            "email": "bob@example.com",
            "age": 25,
            "address": "456 Elm St"
          },
          {
            "name": "alice",
            "email": "alice@example.com",
            "age": 34,
            "address": "123 Main St"
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn with_one_missing_from_the_middle() {
    let mock_sdl = indoc! {r#"
        extend schema
            @link(url: "https://specs.grafbase.com/composite-schema/v1", import: ["@lookup", "@key"])

        type User @key(fields: "id") {
          id: Int!
          age: Int!
        }

        type Query {
          userFoobar: [User!]!
        }
    "#};

    let response = json!([
        {
            "id": 3,
            "age": 13
        },
        {
            "id": 2,
            "age": 14
        },
        {
            "id": 1,
            "age": 12
        }
    ]);

    let subgraph = DynamicSchema::builder(mock_sdl)
        .with_resolver("Query", "userFoobar", response)
        .into_subgraph("mock")
        .unwrap();

    let api = PgTestApi::new_with_subgraphs("", vec![subgraph], |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY NOT NULL,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti'), (3, 'Pentti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          userFoobar {
            id
            name
            age
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userFoobar": [
          {
            "id": 3,
            "name": "Pentti",
            "age": 13
          },
          {
            "id": 2,
            "age": 14
          },
          {
            "id": 1,
            "name": "Musti",
            "age": 12
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn lookup_rename() {
    let mock_sdl = indoc! {r#"
        extend schema
            @link(url: "https://specs.grafbase.com/composite-schema/v1", import: ["@lookup", "@key"])

        type User @key(fields: "id") {
          id: Int!
          age: Int!
        }

        type Query {
          userFoobar: [User!]!
        }
    "#};

    let response = json!([
        {
            "id": 1,
            "age": 13
        },
        {
            "id": 3,
            "age": 30
        }
    ]);

    let subgraph = DynamicSchema::builder(mock_sdl)
        .with_resolver("Query", "userFoobar", response)
        .into_subgraph("mock")
        .unwrap();

    let api = PgTestApi::new_with_subgraphs("", vec![subgraph], |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "users" (id, name) VALUES (1, 'Musti'), (2, 'Naukio'), (3, 'Pentti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          userFoobar {
            id name age
          }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userFoobar": [
          {
            "id": 1,
            "name": "Musti",
            "age": 13
          },
          {
            "id": 3,
            "name": "Pentti",
            "age": 30
          }
        ]
      }
    }
    "#);
}
