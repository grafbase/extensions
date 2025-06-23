use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn char() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val CHAR(5) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "Musti" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "Musti"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn char_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val CHAR(6)[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["Musti", "Naukio"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "Musti ",
              "Naukio"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn name() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val NAME NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "Musti" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "Musti"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn name_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val NAME[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["Musti", "Naukio"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "Musti",
              "Naukio"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn text() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val TEXT NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "Musti" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "Musti"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn text_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val TEXT[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["Musti", "Naukio"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "Musti",
              "Naukio"
            ]
          }
        }
      }
    }
    "#);
}

// TODO: fix
#[tokio::test]
async fn xml() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val XML NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "<html></html>" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "<html></html>"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn xml_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val XML[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["<html></html>", "<head></head>"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "<html></html>",
              "<head></head>"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn cidr() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val CIDR NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "0.0.0.0/0" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "0.0.0.0/0"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn cidr_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val CIDR[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["0.0.0.0/0", "192.168.0.0/32"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "0.0.0.0/0",
              "192.168.0.0/32"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn macaddr8() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val MACADDR8 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {

        userCreate(input: { val: "08:00:2b:01:02:03:04:05" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "08:00:2b:01:02:03:04:05"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn macaddr8_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val MACADDR8[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["08:00:2b:01:02:03:04:05", "08002b:0102030405"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "08:00:2b:01:02:03:04:05",
              "08:00:2b:01:02:03:04:05"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn macaddr() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val MACADDR NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "08:00:2b:01:02:03" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "08:00:2b:01:02:03"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn macaddr_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val MACADDR[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["08:00:2b:01:02:03", "08:00:2b:01:02:04"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "08:00:2b:01:02:03",
              "08:00:2b:01:02:04"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn bpchar() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val BPCHAR(5) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "Musti" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "Musti"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn bpchar_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val BPCHAR(6)[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["Musti", "Naukio"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "Musti ",
              "Naukio"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn varchar() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val VARCHAR(5) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "Musti" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "Musti"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn varchar_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val VARCHAR(6)[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["Musti", "Naukio"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "Musti",
              "Naukio"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn bit() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val BIT(3) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "010" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "010"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn bit_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val BIT(3)[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["010", "101"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "010",
              "101"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn varbit() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val VARBIT(3) NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "010" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "010"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn varbit_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val VARBIT(3)[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["010", "101"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "010",
              "101"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int2() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val INT2 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: 420 }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": 420
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int2_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val INT2[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: [1, 2] }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              1,
              2
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int4() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val INT4 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: 420 }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": 420
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int4_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val INT4[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: [1, 2] }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              1,
              2
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int8() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val INT8 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "420" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "420"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn int8_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val INT8[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["1", "2"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "1",
              "2"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn oid() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val OID NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "420" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "420"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn oid_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val OID[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["1", "2"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "1",
              "2"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn json() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val JSON NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: { foo: 1 } }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": {
              "foo": 1
            }
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn json_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val JSON[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: [{ foo: 1 }, { bar: 2 }] }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              {
                "foo": 1
              },
              {
                "bar": 2
              }
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn jsonb() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val JSONB NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: { foo: 1 } }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": {
              "foo": 1
            }
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn jsonb_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val JSONB[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: [{ foo: 1 }, { bar: 2 }] }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              {
                "foo": 1
              },
              {
                "bar": 2
              }
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn money() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val MONEY NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "$1.23" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "$1.23"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn money_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val MONEY[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["$1.23", "$3.14"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "$1.23",
              "$3.14"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn numeric() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val NUMERIC NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "1.23" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "1.23"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn numeric_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val NUMERIC[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["1.23", "3.14"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              "1.23",
              "3.14"
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn float4() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val FLOAT4 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: 3.14 }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": 3.14
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn float4_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val FLOAT4[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: [3.14, 1.23] }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              3.14,
              1.23
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn float8() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val FLOAT8 NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: 3.14 }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": 3.14
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn float8_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val FLOAT8[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: [3.14, 1.23] }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": [
              3.14,
              1.23
            ]
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn time() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val TIME NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "16:20:00.666" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": "16:20:00.666"
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn time_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val TIME[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
            mutation {
              userCreate(input: { val: ["16:20:00", "04:20:00"] }) {
                returning { val }
              }
            }
        "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": [
                  "16:20:00",
                  "04:20:00"
                ]
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn timetz() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val TIMETZ NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
            mutation {
              userCreate(input: { val: "16:20:00Z" }) {
                returning { val }
              }
            }
        "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreate": {
          "returning": {
            "val": "16:20:00+00"
          }
        }
      }
    }
    "#);
}

#[tokio::test]
async fn timetz_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val TIMETZ[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
            mutation {
              userCreate(input: { val: ["16:20:00+00", "04:20:00Z"] }) {
                returning { val }
              }
            }
        "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": [
                  "16:20:00+00",
                  "04:20:00+00"
                ]
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn bool() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val BOOL NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
            mutation {
              userCreate(input: { val: true }) {
                returning { val }
              }
            }
        "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": true
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn bool_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val BOOL[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
            mutation {
              userCreate(input: { val: [true, false] }) {
                returning { val }
              }
            }
        "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": [
                  true,
                  false
                ]
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn bytea() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val BYTEA NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "\\xdeadbeef" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": "\\xdeadbeef"
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn bytea_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val BYTEA[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["\\xdeadbeef", "\\xdeadbeee"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": [
                  "\\xdeadbeef",
                  "\\xdeadbeee"
                ]
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn inet() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val INET NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
            mutation {
              userCreate(input: { val: "192.168.0.1" }) {
                returning { val }
              }
            }
        "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": "192.168.0.1"
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn inet_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val INET[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
            mutation {
              userCreate(input: { val: ["192.168.0.1", "10.0.0.1"] }) {
                returning { val }
              }
            }
        "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": [
                  "192.168.0.1",
                  "10.0.0.1"
                ]
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn date() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val DATE NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
            mutation {
              userCreate(input: { val: "1999-01-08" }) {
                returning { val }
              }
            }
        "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": "1999-01-08"
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn date_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val DATE[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["1999-01-08", "1999-01-09"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": [
                  "1999-01-08",
                  "1999-01-09"
                ]
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn timestamp() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val TIMESTAMP NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
            mutation {
              userCreate(input: { val: "2004-10-19T10:23:54" }) {
                returning { val }
              }
            }
        "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": "2004-10-19T10:23:54"
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn timestamp_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val TIMESTAMP[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["2004-10-19T10:23:54", "2004-10-19T10:23:55"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": [
                  "2004-10-19T10:23:54",
                  "2004-10-19T10:23:55"
                ]
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn timestamptz() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val TIMESTAMPTZ NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "2004-10-19T10:23:54+00:00" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": "2004-10-19T10:23:54+00:00"
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn timestamptz_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val TIMESTAMPTZ[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: ["2004-10-19T10:23:54+00:00", "2004-10-19T10:23:55+00:00"] }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": [
                  "2004-10-19T10:23:54+00:00",
                  "2004-10-19T10:23:55+00:00"
                ]
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn uuid() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val UUID NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
        mutation {
          userCreate(input: { val: "d89bd15d-ac64-4c71-895c-adba9c35a132" }) {
            returning { val }
          }
        }
    "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": "d89bd15d-ac64-4c71-895c-adba9c35a132"
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn uuid_array() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val UUID[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r#"
            mutation {
              userCreate(input: { val: ["d89bd15d-ac64-4c71-895c-adba9c35a132", "d89bd15d-ac64-4c71-895c-adba9c35a133"] }) {
                returning { val }
              }
            }
        "#};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": [
                  "d89bd15d-ac64-4c71-895c-adba9c35a132",
                  "d89bd15d-ac64-4c71-895c-adba9c35a133"
                ]
              }
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
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: YELLOW }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": "YELLOW"
              }
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
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreate(input: { val: [YELLOW, GREEN] }) {
            returning { val }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
        {
          "data": {
            "userCreate": {
              "returning": {
                "val": [
                  "YELLOW",
                  "GREEN"
                ]
              }
            }
          }
        }
    "#);
}

#[tokio::test]
async fn multiple_enum_arrays() {
    let api = PgTestApi::new("", |api| async move {
        let r#type = indoc! {r"
            CREATE TYPE street_light AS ENUM ('red', 'yellow', 'green');
        "};

        api.execute_sql(r#type).await;

        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                val1 street_light[] NOT NULL,
                val2 street_light[] NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let runner = api.runner_spawn().await;

    let mutation = indoc! {r"
        mutation {
          userCreateMany(input: [
            { val1: [RED], val2: [YELLOW] },
            { val1: [GREEN], val2: [RED, YELLOW] }
          ]) {
            returning { val1 val2 }
          }
        }
    "};

    let response = runner.query(mutation).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "userCreateMany": {
          "returning": [
            {
              "val1": [
                "RED"
              ],
              "val2": [
                "YELLOW"
              ]
            },
            {
              "val1": [
                "GREEN"
              ],
              "val2": [
                "RED",
                "YELLOW"
              ]
            }
          ]
        }
      }
    }
    "#);
}
