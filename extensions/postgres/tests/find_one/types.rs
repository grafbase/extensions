use indoc::indoc;

use crate::PgTestApi;

#[tokio::test]
async fn char() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val CHAR(5) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "Musti"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn name() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val NAME NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "Musti"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn text() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val TEXT NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "Musti"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn xml() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val XML NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '<html></html>')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "<html></html>"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn cidr() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val CIDR NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '0.0.0.0/0')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "0.0.0.0/0"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn macaddr8() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val MACADDR8 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '08:00:2b:01:02:03:04:05')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "08:00:2b:01:02:03:04:05"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn macaddr() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val MACADDR NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '08:00:2b:01:02:03')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "08:00:2b:01:02:03"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn bpchar() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val BPCHAR(5) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "Musti"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn varchar() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val VARCHAR(5) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "Musti"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn bit() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val BIT(3) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, B'010')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "010"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn varbit() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val VARBIT(3) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, B'010')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": "010"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn xml_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val XML[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '{<html></html>, <head></head>}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": [
            "<html></html>",
            "<head></head>"
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn cidr_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val CIDR[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '{0.0.0.0/0, 192.168.0.0/32}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": [
            "0.0.0.0/0",
            "192.168.0.0/32"
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn macaddr8_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val MACADDR8[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '{08:00:2b:01:02:03:04:05, 08002b:0102030405}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": [
            "08:00:2b:01:02:03:04:05",
            "08:00:2b:01:02:03:04:05"
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn macaddr_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val MACADDR8[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '{08:00:2b:01:02:03:04:05, 08002b:0102030405}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": [
            "08:00:2b:01:02:03:04:05",
            "08:00:2b:01:02:03:04:05"
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn char_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val char(6)[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '{Musti, Naukio}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": [
            "Musti ",
            "Naukio"
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn name_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val NAME[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '{Musti, Naukio}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": [
            "Musti",
            "Naukio"
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn text_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "A" (
                id INT PRIMARY KEY,
                val TEXT[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "A" (id, val) VALUES (1, '{Musti, Naukio}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          a(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "a": {
          "id": 1,
          "val": [
            "Musti",
            "Naukio"
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn r#enum() {
    let api = PgTestApi::new("", |api| async move {
        let r#type = indoc! {r"
            CREATE TYPE street_light AS ENUM ('red', 'yellow', 'green');
        "};

        api.execute_sql(r#type).await;

        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val street_light NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 'red');
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": "RED"
        }
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
            INSERT INTO "User" (id, val) VALUES (1, ARRAY['red', 'yellow']::street_light[]);
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) { id val }
        }
    "};

    let response = runner.graphql_query::<serde_json::Value>(query).send().await.unwrap();

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": [
            "RED",
            "YELLOW"
          ]
        }
      }
    }
    "#);
}
