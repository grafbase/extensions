use grafbase_sdk::test::TestGateway;
use std::{net::SocketAddr, str::FromStr as _};

use crate::integration::test_servers::routeguide::routeguide::Point;

#[tokio::test]
async fn basic() {
    let grpc_server_addr =
        SocketAddr::from_str(&format!("[::1]:{}", (rand::random::<u16>() % 16_384) + 49_152)).unwrap();

    let gateway = TestGateway::builder()
        .subgraph(
            r#"
            extend schema
                @link(url: "<self>", import: ["@grpcMethod", "@protoMessages", "@protoServices"])
                @protoMessages(definitions: [
                  {
                    name: "Point"
                    fields: [
                      { name: "latitude", type: "int32", number: 1 }
                      { name: "longitude", type: "int32", number: 2 }
                    ]
                  },
                  {
                    name: "Feature"
                    fields: [
                      { name: "name", type: "string", number: 1 }
                      { name: "location", type: "Point", number: 2 }
                    ]
                  }
                  {
                    name: "Rectangle"
                    fields: [
                      { name: "lo", type: "Point", number: 1 }
                      { name: "hi", type: "Point", number: 2 }
                    ]
                  }
                ])
                @protoServices(definitions: [
                  {
                    name: "routeguide.RouteGuide"
                    methods: [
                    { name: "GetFeature", inputType: "Point", outputType: "Feature" }
                    { name: "ListFeatures", inputType: "Rectangle", outputType: "Feature", serverStreaming: true }
                    ]
                  }
                ])


            type Query {
                getFeature(input: PointInput!): Feature @grpcMethod(service: "routeguide.RouteGuide", method: "GetFeature")
            }

            type Subscription {
                listFeatures(input: RectangleInput!): Feature @grpcMethod(service: "routeguide.RouteGuide", method: "ListFeatures")
            }

            input RectangleInput {
                lo: PointInput!
                hi: PointInput!
            }

            input PointInput {
                latitude: Int!
                longitude: Int!
            }

            type Feature {
                name: String
                location: Point
            }

            type Point {
                latitude: Int!
                longitude: Int!
            }
        "#,
        )
        .toml_config(format!(
            r#"
            [[extensions.grpc.config.services]]
            name = "routeguide.RouteGuide"
            address = "http://{grpc_server_addr}"
            "#
        ))
        .enable_networking()
        .enable_stdout()
        .enable_stderr()
        .build()
        .await.unwrap();

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

    let response = gateway
        .query(
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
        .await;

    insta::assert_json_snapshot!(response, @r#"
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

    let sub = gateway
        .query(
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
        .ws_stream()
        .await;

    let features = sub.take(12).await;

    insta::assert_json_snapshot!(&features, @r#"
    [
      {
        "data": {
          "listFeatures": {
            "name": "Patriots Path, Mendham, NJ 07945, USA",
            "location": {
              "latitude": 407838351,
              "longitude": -746143763
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "101 New Jersey 10, Whippany, NJ 07981, USA",
            "location": {
              "latitude": 408122808,
              "longitude": -743999179
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "U.S. 6, Shohola, PA 18458, USA",
            "location": {
              "latitude": 413628156,
              "longitude": -749015468
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "5 Conners Road, Kingston, NY 12401, USA",
            "location": {
              "latitude": 419999544,
              "longitude": -740371136
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "Mid Hudson Psychiatric Center, New Hampton, NY 10958, USA",
            "location": {
              "latitude": 414008389,
              "longitude": -743951297
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "287 Flugertown Road, Livingston Manor, NY 12758, USA",
            "location": {
              "latitude": 419611318,
              "longitude": -746524769
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "4001 Tremley Point Road, Linden, NJ 07036, USA",
            "location": {
              "latitude": 406109563,
              "longitude": -742186778
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "352 South Mountain Road, Wallkill, NY 12589, USA",
            "location": {
              "latitude": 416802456,
              "longitude": -742370183
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "Bailey Turn Road, Harriman, NY 10926, USA",
            "location": {
              "latitude": 412950425,
              "longitude": -741077389
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "193-199 Wawayanda Road, Hewitt, NJ 07421, USA",
            "location": {
              "latitude": 412144655,
              "longitude": -743949739
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "406-496 Ward Avenue, Pine Bush, NY 12566, USA",
            "location": {
              "latitude": 415736605,
              "longitude": -742847522
            }
          }
        }
      },
      {
        "data": {
          "listFeatures": {
            "name": "162 Merrill Road, Highland Mills, NY 10930, USA",
            "location": {
              "latitude": 413843930,
              "longitude": -740501726
            }
          }
        }
      }
    ]
    "#);
}
