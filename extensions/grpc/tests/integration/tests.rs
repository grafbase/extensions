use grafbase_sdk::test::{DynamicSchema, TestConfig, TestRunner};
use std::{net::SocketAddr, str::FromStr as _};
use tokio_stream::StreamExt;

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
            @protoMessages(definitions: [
              {{
                name: "Point"
                fields: [
                  {{ name: "latitude", type: "int32", number: 1 }}
                  {{ name: "longitude", type: "int32", number: 2 }}
                ]
              }},
              {{
                name: "Feature"
                fields: [
                  {{ name: "name", type: "string", number: 1 }}
                  {{ name: "location", type: "Point", number: 2 }}
                ]
              }}
              {{
                name: "Rectangle"
                fields: [
                  {{ name: "lo", type: "Point", number: 1 }}
                  {{ name: "hi", type: "Point", number: 2 }}
                ]
              }}
            ])
            @protoServices(definitions: [
              {{
                name: "routeguide.RouteGuide"
                methods: [
                {{ name: "GetFeature", inputType: "Point", outputType: "Feature" }}
                {{ name: "ListFeatures", inputType: "Rectangle", outputType: "Feature", serverStreaming: true }}
                ]
              }}
            ])


        type Query {{
            getFeature(input: PointInput!): Feature @grpcMethod(service: "routeguide.RouteGuide", method: "GetFeature")
        }}

        type Subscription {{
            listFeatures(input: RectangleInput!): Feature @grpcMethod(service: "routeguide.RouteGuide", method: "ListFeatures")
        }}

        input RectangleInput {{
            lo: PointInput!
            hi: PointInput!
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
        name = "routeguide.RouteGuide"
        address = "http://{grpc_server_addr}"
    "#
    );

    let config = TestConfig::builder()
        .with_subgraph(subgraph)
        .enable_networking()
        .enable_stdout()
        .enable_stderr()
        .build(config)
        .unwrap();

    let runner = TestRunner::new(config).await.unwrap();

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
        .unwrap()
        .into_inner();

    insta::assert_debug_snapshot!(response, @r#"
    Feature {
        name: "Berkshire Valley Management Area Trail, Jefferson, NJ, USA",
        location: Some(
            Point {
                latitude: 409146138,
                longitude: -746188906,
            },
        ),
    }
    "#);

    let result: serde_json::Value = runner
        .graphql_query(
            r#"
            query {
                getFeature(input: { latitude: 409146138, longitude: -746188906 }) {
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

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "getFeature": {
          "name": "Berkshire Valley Management Area Trail, Jefferson, NJ, USA",
          "location": {
            "latitude": 409146138,
            "longitude": -746188906
          }
        }
      }
    }
    "#);

    let sub = runner
        .graphql_subscription::<serde_json::Value>(
            r#"
        subscription {
          listFeatures(input: {
            lo: {
                latitude: 400000000,
                longitude: -750000000,
            },
            hi: {
                latitude: 420000000,
                longitude: -730000000,
            }
          }) {
            name
            location {
                latitude
                longitude
            }
          }
        }
    "#,
        )
        .unwrap();

    let stream = sub.subscribe().await.unwrap();

    let features: Vec<serde_json::Value> = stream.take(12).collect().await;

    insta::assert_debug_snapshot!(&features);
}
