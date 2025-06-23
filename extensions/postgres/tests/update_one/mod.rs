use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn string_set_with_returning() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { name: { set: "Naukio" } }) {
            returning {
              id
              name
            }
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1,
            "name": "Naukio"
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            name
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "name": "Naukio"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn enum_set_with_returning() {
    let api = PgTestApi::new("", |api| async move {
        let r#type = indoc! {r"
            CREATE TYPE street_light AS ENUM ('red', 'yellow', 'green');
        "};

        api.execute_sql(r#type).await;

        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name street_light NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'red')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { name: { set: GREEN } }) {
            returning {
              id
              name
            }
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1,
            "name": "GREEN"
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            name
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "name": "GREEN"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn enum_array_set_with_returning() {
    let api = PgTestApi::new("", |api| async move {
        let r#type = indoc! {r"
            CREATE TYPE street_light AS ENUM ('red', 'yellow', 'green');
        "};

        api.execute_sql(r#type).await;

        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name street_light[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, ARRAY['red', 'yellow', 'green']::street_light[])
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { name: { set: [GREEN, YELLOW] } }) {
            returning {
              id
              name
            }
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1,
            "name": [
              "GREEN",
              "YELLOW"
            ]
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            name
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "name": [
            "GREEN",
            "YELLOW"
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn string_set_no_returning() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, name) VALUES (1, 'Musti')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { name: { set: "Naukio" } }) {
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            name
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "name": "Naukio"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int2_increment() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val INT2 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 1)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { increment: 68 } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": 69
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int2_decrement() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val INT2 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 70)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { decrement: 1 } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": 69
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int2_multiply() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val INT2 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 6)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { multiply: 8 } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": 48
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int2_divide() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val INT2 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 138)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { divide: 2 } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": 69
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int4_increment() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val INT4 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 1)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { increment: 68 } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": 69
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int8_increment() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val INT8 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 1)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { increment: "68" } }) {
            returning { id }
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": "69"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn float_increment() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val FLOAT4 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 1.0)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { increment: 68.0 } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": 69.0
        }
      }
    }
    "#);
}

#[tokio::test]
async fn double_increment() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val FLOAT8 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 1.0)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { increment: 68.0 } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": 69.0
        }
      }
    }
    "#);
}

#[tokio::test]
async fn numeric_increment() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val NUMERIC NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 1.0)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { increment: "68.0" } }) {
            returning { id }
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": "69.0"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn money_increment() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val MONEY NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, 1.0)
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { increment: "68.0" } }) {
            returning { id }
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": "$69.00"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn array_set() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val INT2[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, '{1, 2}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { set: [3, 4] } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": [
            3,
            4
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn array_append() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val INT[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, '{1}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { append: [2, 3] } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": [
            1,
            2,
            3
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn array_prepend() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val INT[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, '{1}')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { prepend: [2, 3] } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": [
            2,
            3,
            1
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn jsonb_append() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val JSONB NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, '[1]')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { append: [2, 3] } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": [
            1,
            2,
            3
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn jsonb_prepend() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val JSONB NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, '[1]')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { prepend: [2, 3] } }) {
            returning { id }
            rowCount
          }
        }
    "};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": [
            2,
            3,
            1
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn jsonb_delete_key_from_object() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val JSONB NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, '{ "foo": 1, "bar": 2 }')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { deleteKey: "foo" } }) {
            returning { id }
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": {
            "bar": 2
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn jsonb_delete_key_from_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val JSONB NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, '["foo", "bar"]')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { deleteKey: "foo" } }) {
            returning { id }
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": [
            "bar"
          ]
        }
      }
    }
    "#);
}

#[tokio::test]
async fn jsonb_delete_at_path() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY,
                val JSONB NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let insert = indoc! {r#"
            INSERT INTO "User" (id, val) VALUES (1, '["a", { "b": 1 }]')
        "#};

        api.execute_sql(insert).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userUpdate(lookup: { id: 1 }, input: { val: { deleteAtPath: ["1", "b"] } }) {
            returning { id }
            rowCount
          }
        }
    "#};

    let mutation_response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(mutation_response, @r#"
    {
      "data": {
        "userUpdate": {
          "returning": {
            "id": 1
          },
          "rowCount": 1
        }
      }
    }
    "#);

    let query = indoc! {r"
        query {
          user(lookup: { id: 1 }) {
            id
            val
          }
        }
    "};

    let query_response = runner.query(query).send().await;

    insta::assert_json_snapshot!(query_response, @r#"
    {
      "data": {
        "user": {
          "id": 1,
          "val": [
            "a",
            {}
          ]
        }
      }
    }
    "#);
}
