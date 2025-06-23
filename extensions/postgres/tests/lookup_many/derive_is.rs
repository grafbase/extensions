use crate::PgTestApi;
use grafbase_sdk::test::GraphqlSubgraph;
use indoc::indoc;
use serde_json::json;

#[tokio::test]
async fn join_one_column() {
    let mock_sdl = indoc! {r#"
        extend schema
            @link(url: "https://specs.grafbase.com/composite-schemas/v1", import: ["@lookup", "@key", "@derive", "@is"])

        type Post @key(fields: "id") {
          id: Int!
          title: String!
          authorId: Int!
          author: User! @derive @is(field: "{ id: authorId }")
        }

        type User @key(fields: "id") {
          id: Int!
        }

        type Query {
          postFoobar: [Post!]!
        }
    "#};

    let response = json!([
        {
            "id": 1,
            "title": "foo",
            "authorId": 1
        },
        {
            "id": 2,
            "title": "bar",
            "authorId": 3
        }
    ]);

    let subgraph = GraphqlSubgraph::with_schema(mock_sdl)
        .with_resolver("Query", "postFoobar", response)
        .with_name("mock")
        .build();

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
          postFoobar {
            id title author { id name }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "postFoobar": [
          {
            "id": 1,
            "title": "foo",
            "author": {
              "id": 1,
              "name": "Musti"
            }
          },
          {
            "id": 2,
            "title": "bar",
            "author": {
              "id": 3,
              "name": "Pentti"
            }
          }
        ]
      }
    }
    "#);
}

#[tokio::test]
async fn join_one_composite() {
    let mock_sdl = indoc! {r#"
        extend schema
            @link(url: "https://specs.grafbase.com/composite-schemas/v1", import: ["@lookup", "@key", "@derive", "@is"])

        type Post @key(fields: "id") {
          id: Int!
          title: String!
          authorEmail: String!
          authorName: String!
          author: User! @derive @is(field: "{ email: authorEmail name: authorName }")
        }

        type User @key(fields: "email name") {
          email: String!
          name: String!
        }

        type Query {
          postFoobar: [Post!]!
        }
    "#};

    let response = json!([
        {
            "id": 1,
            "title": "foo",
            "authorEmail": "musti@example.com",
            "authorName": "Musti",
        },
        {
            "id": 2,
            "title": "bar",
            "authorEmail": "pentti@example.com",
            "authorName": "Pentti",
        }
    ]);

    let subgraph = GraphqlSubgraph::with_schema(mock_sdl)
        .with_resolver("Query", "postFoobar", response)
        .with_name("mock")
        .build();

    let api = PgTestApi::new_with_subgraphs("", vec![subgraph], |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                PRIMARY KEY (name, email)
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "users" (name, email) VALUES
                ('Musti', 'musti@example.com'),
                ('Naukio', 'naukio@example.com'),
                ('Pentti', 'pentti@example.com')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          postFoobar {
            id title author { name email }
          }
        }
    "};

    let response = runner.query(query).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "postFoobar": [
          {
            "id": 1,
            "title": "foo",
            "author": {
              "name": "Musti",
              "email": "musti@example.com"
            }
          },
          {
            "id": 2,
            "title": "bar",
            "author": {
              "name": "Pentti",
              "email": "pentti@example.com"
            }
          }
        ]
      }
    }
    "#);
}
