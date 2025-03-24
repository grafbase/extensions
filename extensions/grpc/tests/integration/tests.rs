use std::{net::SocketAddr, str::FromStr as _};

use grafbase_sdk::test::{DynamicSchema, LogLevel, TestConfig, TestRunner};

use crate::integration::test_servers::routeguide::routeguide::Point;

#[tokio::test]
async fn basic() {
    let extension_path = std::env::current_dir().unwrap().join("build");
    let path_str = format!("file://{}", extension_path.display());

    // Create a subgraph with a single field
    let subgraph = DynamicSchema::builder(format!(
        r#"
        extend schema
            @link(url: "{path_str}", import: ["@grpcMethod", "@protoMessages", "@protoServices"])
            @protoMessages(messages: [
              {{
                name: "Point"
                fields: [
                  {{ name: "latitude", type: "int32", number: 1 }}
                  {{ name: "longitude", type: "int32", number: 2 }}
                ]
              }}
            ])
            @protoServices(services: [
              {{
                name: "routeguide.RouteGuide"
                methods: [
                  {{ name: "GetFeature", inputType: "Point", outputType: "Feature" }}
                ]
              }}
            ])


        type Query {{
            getFeature(point: PointInput): Feature @grpcMethod(service: "routeguide.RouteGuide", method: "GetFeature", input: "point")
        }}

        input PointInput {{
            latitude: Int!
            longitude: Int!
        }}

        type Feature {{
            name: String
            location: Point
        }}

        type Point {{
            latitude: Int!
            longitude: Int!
        }}
    "#,
    ))
    .into_extension_only_subgraph("test", &extension_path)
    .unwrap();

    let grpc_server_addr =
        SocketAddr::from_str(&format!("[::1]:{}", (rand::random::<u16>() % 16_384) + 49_152)).unwrap();

    let config = format!(
        r#"
        [[extensions.grpc.config.services]]
        name = "routeguide"
        address = "http://{grpc_server_addr}"
    "#
    );

    let config = TestConfig::builder()
        .with_subgraph(subgraph)
        .enable_networking()
        .enable_stdout()
        .enable_stderr()
        // .log_level(LogLevel::WasiDebug)
        .build(config)
        .unwrap();

    let runner = TestRunner::new(config).await.unwrap();

    let result: serde_json::Value = runner
        .graphql_query(
            r#"
            query {
                getFeature(point: { latitude: 409146138, longitude: -746188906 }) {
                    name
                    location {
                        latitude
                        longitude
                    }
                }
            }"#,
        )
        .send()
        .await
        .unwrap();

    // The result is compared against a snapshot.
    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "getFeature": {
          "name": "hi",
          "location": {
            "latitude": 1,
            "longitude": -1
          }
        }
      }
    }
    "#);

    panic!("stopping here");

    // Start the test GRPC server.
    tokio::spawn(async move {
        super::test_servers::routeguide::run_server(grpc_server_addr)
            .await
            .unwrap()
    });

    // Leave the server some time to start.
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // First check with a native client querying the server directly.
    let mut routeguide_client = super::test_servers::routeguide::client(grpc_server_addr).await;

    let response = routeguide_client
        .get_feature(Point {
            latitude: 409_146_138,
            longitude: -746_188_906,
        })
        .await
        .unwrap();

    panic!("Response: {response:#?}");
}
