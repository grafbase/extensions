use grafbase_sdk::test::TestGateway;
use wiremock::matchers;

#[derive(serde::Deserialize, serde::Serialize)]
struct Response {
    data: Option<serde_json::Value>,
    errors: Vec<Error>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Error {
    message: String,
    extensions: Option<serde_json::Value>,
}

#[tokio::test]
async fn test_basic_responses() {
    let mock_server = wiremock::MockServer::start().await;
    let mock_server_url = mock_server.address();

    let test_rsa_private_key = include_str!("./test_rsa_key.p8");
    let test_rsa_public_key = include_str!("./test_rsa_key.pub");

    let config = format! {r#"
        [extensions.snowflake.config]
        account = "cywwwdp-qv94952"
        user = "tomhoule"

        snowflake_api_url_override = "http://{mock_server_url}"

        warehouse = "COMPUTE_WH"
        database = "SNOWFLAKE_SAMPLE_DATA"
        schema = "TPCH_SF1"
        # role = ""

        [extensions.snowflake.config.authentication.key_pair_jwt]
        public_key = """
        {test_rsa_public_key}
        """
        private_key = """
        {test_rsa_private_key}
        """
    "#};

    let gateway = TestGateway::builder()
        .subgraph(
            r#"
            extend schema
              @link(url: "https://specs.apollo.dev/federation/v2.7")
              @link(url: "<self>", import: ["@snowflakeQuery"])

            scalar JSON

            type Query {
              hi(params: [JSON!]!): [[JSON!]!] @snowflakeQuery(sql: "SELECT ?", bindings: "{{ args.params }}")
              users(params: [JSON!]!): [[JSON!]!]
                @snowflakeQuery(sql: "SELECT * FROM CUSTOMER LIMIT ?;", bindings: "{{ args.params }}")
            }

            "#,
        )
        .log_level(grafbase_sdk::test::LogLevel::Debug)
        .toml_config(&config)
        .build()
        .await
        .unwrap();

    wiremock::Mock::given(matchers::method("POST"))
        .and(matchers::path("/api/v2/statements"))
        .and(matchers::body_partial_json(serde_json::json!({
            "statement": "SELECT ?",
        })))
        .respond_with(wiremock::ResponseTemplate::new(200)
        .set_body_raw(
        r#"
{
"resultSetMetaData" : {
 "numRows" : 1,
 "format" : "jsonv2",
 "partitionInfo" : [ {
   "rowCount" : 1,
   "uncompressedSize" : 8
 } ],
 "rowType" : [ {
   "name" : "?",
   "database" : "",
   "schema" : "",
   "table" : "",
   "nullable" : false,
   "length" : null,
   "type" : "fixed",
   "scale" : 0,
   "precision" : 4,
   "byteLength" : null,
   "collation" : null
 } ]
},
"data" : [ ["9999"] ],
"code" : "090001",
"statementStatusUrl" : "/api/v2/statements/01bad80f-0000-4392-0000-3c790002f19e?requestId=7c765ba6-f6e3-4407-bb44-206bf63ddd96",
"requestId" : "7c765ba6-f6e3-4407-bb44-206bf63ddd96",
"sqlState" : "00000",
"statementHandle" : "01bad80f-0000-4392-0000-3c790002f19e",
"message" : "Statement executed successfully.",
"createdOn" : 1741333409656
}
"#, "application/json"
    )).mount(&mock_server).await;

    let response = gateway.query(r#"query { hi(params: [9999]) }"#).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "hi": [
          [
            "9999"
          ]
        ]
      }
    }
    "#);

    wiremock::Mock::given(matchers::method("POST"))
        .and(matchers::path("/api/v2/statements"))
        .and(matchers::body_json(serde_json::json!({
            "statement": "SELECT * FROM CUSTOMER LIMIT ?;",
            "bindings": {
                "1":  {
                    "type": "TEXT",
                    "value": "abcd",
                },
            },
            "database": "SNOWFLAKE_SAMPLE_DATA",
            "schema": "TPCH_SF1",
            "warehouse": "COMPUTE_WH",
            "role": null,
        })))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_raw(
            r#"
{
"code" : "002010",
"message" : "SQL compilation error:\nInvalid row count '?' in limit clause",
"sqlState" : "2201W",
"statementHandle" : "01bad820-0000-43c9-0000-3c790003200e"
}
"#,
            "application/json",
        ))
        .mount(&mock_server)
        .await;

    let response = gateway.query(r#"query { users(params: ["abcd"]) }"#).send().await;

    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "users": null
      },
      "errors": [
        {
          "message": "No data returned from Snowflake query. SQL State: 2201W, Code: 002010. Message: SQL compilation error:\nInvalid row count '?' in limit clause",
          "locations": [
            {
              "line": 1,
              "column": 9
            }
          ],
          "path": [
            "users"
          ],
          "extensions": {
            "code": "EXTENSION_ERROR"
          }
        }
      ]
    }
    "#);
}
