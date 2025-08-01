use std::time::Duration;

use async_nats::{
    jetstream::{kv, stream::Config},
    ConnectOptions,
};
use futures::StreamExt;
use grafbase_sdk::test::{TestGateway, TestGatewayBuilder};
use indoc::indoc;
use serde_json::json;

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

async fn nats_client() -> async_nats::Client {
    let opts = ConnectOptions::new().user_and_password("grafbase".to_string(), "grafbase".to_string());
    let addrs = vec!["nats://localhost:4222"];

    async_nats::connect_with_options(addrs, opts).await.unwrap()
}

fn gateway_builder() -> TestGatewayBuilder {
    TestGateway::builder()
        .subgraph(
            r#"
            extend schema
              @link(url: "https://specs.apollo.dev/federation/v2.0", import: ["@key", "@shareable"])
              @link(
                url: "<self>",
                import: [
                  "@natsPublish",
                  "@natsSubscription",
                  "@natsRequest",
                  "@natsKeyValue",
                  "NatsStreamDeliverPolicy",
                ]
              )

            type Query {
              hello: String!

              requestReply(input: RequestReplyInput!): RequestReplyResult! @natsRequest(
                subject: "help.please",
                timeoutMs: 500,
              )

              timeoutReply(input: RequestReplyInput!): RequestReplyResult! @natsRequest(
                subject: "timeout.please",
                timeoutMs: 500,
              )

              getUser(id: Int!): User @natsKeyValue(
                bucket: "users",
                key: "user.{{ args.id }}",
                action: GET,
                selection: "{ id, email, name }",
              )

              getOtherUser(id: Int!): User @natsKeyValue(
                bucket: "otherUsers",
                key: "user.{{ args.id }}",
                action: GET,
                selection: "{ id, email, name }",
              )
            }

            type Mutation {
              publishUserEvent(id: Int!, input: UserEventInput!): Boolean! @natsPublish(
                subject: "publish.user.{{args.id}}.events"
              )

              kvPutUser(id: Int!, input: UserEventInput!): String! @natsKeyValue(
                bucket: "putUsers",
                key: "user.{{ args.id }}",
                action: PUT,
              )

              kvCreateUser(id: Int!, input: UserEventInput!): String! @natsKeyValue(
                bucket: "createUsers",
                key: "user.{{ args.id }}",
                action: CREATE,
              )

              kvDeleteUser(id: Int!): Boolean! @natsKeyValue(
                bucket: "deleteUsers",
                key: "user.{{ args.id }}",
                action: DELETE,
              )
            }

            type Subscription {
              userEvents(id: Int!): UserEvent! @natsSubscription(
                subject: "subscription.user.{{args.id}}.events",
                selection: "{ email, name }",
              )

              highPriorityBankEvents(limit: Int!): BankEvent! @natsSubscription(
                subject: "subscription.bank",
                selection: "select(.money > {{args.limit}}) | { id, account, money }",
              )

              persistenceEvents(id: Int!): UserEvent! @natsSubscription(
                subject: "persistence.user.{{args.id}}.events",
                selection: "{ email, name}",
                streamConfig: {
                  streamName: "testStream",
                  consumerName: "testConsumer",
                  durableName: "testConsumer",
                  description: "Test Description",
                },
              )

              nonexistingEvents(id: Int!): UserEvent! @natsSubscription(
                subject: "persistence.user.{{args.id}}.events",
                selection: "{ email, name }",
                streamConfig: {
                  streamName: "nonExistingStream",
                  consumerName: "testConsumer",
                  durableName: "testConsumer",
                  description: "Test Description",
                },
              )
            }

            input RequestReplyInput {
              message: String!
            }

            type RequestReplyResult {
              message: String!
            }

            input UserEventInput {
              email: String!
              name: String!
            }

            type UserEvent {
              email: String!
              name: String!
            }

            type User {
              id: Int!
              email: String!
              name: String!
            }

            type BankEvent {
              id: Int!
              account: String!
              money: Int!
            }
            "#,
        )
        .toml_config(
            r#"
            [extensions.nats]
            stdout = true
            stderr = true

            [[extensions.nats.config.endpoint]]
            servers = ["nats://localhost:4222"]

            [extensions.nats.config.endpoint.authentication]
            username = "grafbase"
            password = "grafbase"
            "#,
        )
}

#[tokio::test]
async fn test_subscribe() {
    let nats = nats_client().await;
    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        subscription {
          userEvents(id: 1) {
            email
            name
          }
        }
    "#};

    let subscription1 = gateway.query(query).ws_stream().await;

    let query = indoc! {r#"
        subscription {
          userEvents(id: 2) {
            email
            name
          }
        }
    "#};

    let subscription2 = gateway.query(query).ws_stream().await;

    tokio::spawn(async move {
        for _ in 0.. {
            let event1 = json!({ "email": "user1@example.com", "name": "User One" });
            let event2 = json!({ "email": "user2@example.com", "name": "User Two" });

            let event1 = serde_json::to_vec(&event1).unwrap();
            let event2 = serde_json::to_vec(&event2).unwrap();

            nats.publish("subscription.user.1.events", event1.into()).await.unwrap();
            nats.publish("subscription.user.2.events", event2.into()).await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    let events = tokio::time::timeout(Duration::from_secs(10), subscription1.take(2))
        .await
        .unwrap();

    insta::assert_json_snapshot!(&events, @r#"
    [
      {
        "data": {
          "userEvents": {
            "email": "user1@example.com",
            "name": "User One"
          }
        }
      },
      {
        "data": {
          "userEvents": {
            "email": "user1@example.com",
            "name": "User One"
          }
        }
      }
    ]
    "#);

    let events = tokio::time::timeout(Duration::from_secs(10), subscription2.take(2))
        .await
        .unwrap();

    insta::assert_json_snapshot!(&events, @r#"
    [
      {
        "data": {
          "userEvents": {
            "email": "user2@example.com",
            "name": "User Two"
          }
        }
      },
      {
        "data": {
          "userEvents": {
            "email": "user2@example.com",
            "name": "User Two"
          }
        }
      }
    ]
    "#);
}

#[tokio::test]
async fn test_subscribe_with_filter() {
    let nats = nats_client().await;
    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        subscription {
          highPriorityBankEvents(limit: 1000) {
            id
            account
            money
          }
        }
    "#};

    {
        let subscription = gateway.query(query).ws_stream().await;

        let nats = nats_client().await;

        tokio::spawn(async move {
            for i in 1000..=1002 {
                let event = json!({ "id": 1, "account": "User One", "money": i });
                let event = serde_json::to_vec(&event).unwrap();

                nats.publish("subscription.bank", event.into()).await.unwrap();
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });

        let events = tokio::time::timeout(Duration::from_secs(30), subscription.take(2))
            .await
            .unwrap();

        insta::assert_json_snapshot!(&events, @r#"
        [
          {
            "data": {
              "highPriorityBankEvents": {
                "id": 1,
                "account": "User One",
                "money": 1001
              }
            }
          },
          {
            "data": {
              "highPriorityBankEvents": {
                "id": 1,
                "account": "User One",
                "money": 1002
              }
            }
          }
        ]
        "#);
    }

    let subscription = gateway.query(query).ws_stream().await;

    tokio::spawn(async move {
        for i in 1000..=1002 {
            let event = json!({ "id": 1, "account": "User One", "money": i });
            let event = serde_json::to_vec(&event).unwrap();

            nats.publish("subscription.bank", event.into()).await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    let events = tokio::time::timeout(Duration::from_secs(30), subscription.take(2))
        .await
        .unwrap();

    insta::assert_json_snapshot!(&events, @r#"
    [
      {
        "data": {
          "highPriorityBankEvents": {
            "id": 1,
            "account": "User One",
            "money": 1001
          }
        }
      },
      {
        "data": {
          "highPriorityBankEvents": {
            "id": 1,
            "account": "User One",
            "money": 1002
          }
        }
      }
    ]
    "#);
}

#[tokio::test]
async fn test_publish() {
    let nats = nats_client().await;
    let mut subscriber = nats.subscribe("publish.user.>").await.unwrap();
    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        mutation {
          publishUserEvent(id: 1, input: { email: "alice@example.com", name: "Alice" })
        }
    "#};

    let response = gateway.query(query).send().await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "publishUserEvent": true
      }
    }
    "#);

    let event = subscriber.next().await.unwrap();
    assert_eq!(event.subject.as_str(), "publish.user.1.events");

    let event: serde_json::Value = serde_json::from_slice(event.payload.as_ref()).unwrap();
    insta::assert_json_snapshot!(&event, @r#"
    {
      "email": "alice@example.com",
      "name": "Alice"
    }
    "#);
}

#[tokio::test]
async fn test_existing_stream() {
    let nats = nats_client().await;
    let context = async_nats::jetstream::new(nats);

    let _ = context.delete_stream("testStream").await;

    context
        .create_stream(Config {
            name: String::from("testStream"),
            subjects: vec![String::from("persistence.user.1.events")],
            ..Default::default()
        })
        .await
        .unwrap();

    let gateway = gateway_builder().build().await.unwrap();

    tokio::spawn(async move {
        for _ in 1.. {
            let event = json!({ "email": "user1@example.com", "name": "User One" });
            let event = serde_json::to_vec(&event).unwrap();

            context
                .publish("persistence.user.1.events", event.into())
                .await
                .unwrap();

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    let query = indoc! {r#"
        subscription {
          persistenceEvents(id: 1) {
            email
            name
          }
        }
    "#};

    let subscription = gateway.query(query).ws_stream().await;

    let events = tokio::time::timeout(Duration::from_secs(5), subscription.take(2))
        .await
        .unwrap();

    insta::assert_json_snapshot!(&events, @r#"
    [
      {
        "data": {
          "persistenceEvents": {
            "email": "user1@example.com",
            "name": "User One"
          }
        }
      },
      {
        "data": {
          "persistenceEvents": {
            "email": "user1@example.com",
            "name": "User One"
          }
        }
      }
    ]
    "#);
}

#[tokio::test]
async fn test_non_existing_stream() {
    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        subscription {
          nonexistingEvents(id: 1) {
            email
            name
          }
        }
    "#};

    let subscription = gateway.query(query).ws_stream().await;

    let events = tokio::time::timeout(Duration::from_secs(5), subscription.take(2))
        .await
        .unwrap();

    insta::assert_json_snapshot!(&events, @r#"
    [
      {
        "data": null,
        "errors": [
          {
            "message": "Failed to subscribe to subject 'persistence.user.1.events': jetstream error: stream not found (code 404, error code 10059)",
            "locations": [
              {
                "line": 2,
                "column": 3
              }
            ],
            "path": [
              "nonexistingEvents"
            ],
            "extensions": {
              "code": "EXTENSION_ERROR"
            }
          }
        ]
      }
    ]
    "#);
}

#[tokio::test]
async fn request_reply() {
    tokio::spawn(async move {
        let nats = nats_client().await;
        let mut subscription = nats.subscribe("help.please").await.unwrap();
        let reply = json!({ "message": "OK, I CAN HELP!!!" });

        while let Some(message) = subscription.next().await {
            let reply_subject = message.reply.unwrap();

            nats.publish(reply_subject, serde_json::to_vec(&reply).unwrap().into())
                .await
                .unwrap();
        }
    });

    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        query {
          requestReply(input: { message: "Help, please!" }) {
            message
          }
        }
    "#};

    let response = gateway.query(query).send().await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "requestReply": {
          "message": "OK, I CAN HELP!!!"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn request_reply_timeout() {
    tokio::spawn(async move {
        let nats = nats_client().await;
        let mut subscription = nats.subscribe("timeout.please").await.unwrap();
        let reply = json!({ "message": "OK, I CAN HELP!!!" });

        while (subscription.next().await).is_some() {
            nats.publish("other.subject", serde_json::to_vec(&reply).unwrap().into())
                .await
                .unwrap();
        }
    });

    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        query {
          timeoutReply(input: { message: "Help, please!" }) {
            message
          }
        }
    "#};

    let response = gateway.query(query).send().await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": null,
      "errors": [
        {
          "message": "Failed to request message: deadline has elapsed",
          "locations": [
            {
              "line": 2,
              "column": 3
            }
          ],
          "path": [
            "timeoutReply"
          ],
          "extensions": {
            "code": "EXTENSION_ERROR"
          }
        }
      ]
    }
    "#);
}

#[tokio::test]
async fn kv_get_missing() {
    let nats = nats_client().await;
    let jet = async_nats::jetstream::new(nats);

    let _ = jet.delete_key_value("users").await;

    let kv_config = kv::Config {
        bucket: String::from("users"),
        ..Default::default()
    };

    jet.create_key_value(kv_config).await.unwrap();

    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        query {
          getUser(id: 1) {
            id
            email
            name
          }
        }
    "#};

    let response = gateway.query(query).send().await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "getUser": null
      }
    }
    "#);
}

#[tokio::test]
async fn kv_get_existing() {
    let nats = nats_client().await;
    let jet = async_nats::jetstream::new(nats);

    let _ = jet.delete_key_value("otherUsers").await;

    let kv_config = kv::Config {
        bucket: String::from("otherUsers"),
        ..Default::default()
    };

    let bucket = jet.create_key_value(kv_config).await.unwrap();
    let data = json!({
        "id": 1,
        "email": "user1@example.com",
        "name": "User One"
    });

    bucket
        .create("user.1", serde_json::to_vec(&data).unwrap().into())
        .await
        .unwrap();

    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        query {
          getOtherUser(id: 1) {
            id
            email
            name
          }
        }
    "#};

    let response = gateway.query(query).send().await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "getOtherUser": {
          "id": 1,
          "email": "user1@example.com",
          "name": "User One"
        }
      }
    }
    "#);
}

#[tokio::test]
async fn kv_put() {
    let nats = nats_client().await;
    let jet = async_nats::jetstream::new(nats);

    let _ = jet.delete_key_value("putUsers").await;

    let kv_config = kv::Config {
        bucket: String::from("putUsers"),
        ..Default::default()
    };

    let bucket = jet.create_key_value(kv_config).await.unwrap();

    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        mutation {
          kvPutUser(id: 1,input: { email: "user1@example.com", name: "User One" })
        }
    "#};

    let response = gateway.query(query).send().await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "kvPutUser": "1"
      }
    }
    "#);

    let user = bucket.get("user.1").await.unwrap().unwrap();
    let user: serde_json::Value = serde_json::from_slice(user.as_ref()).unwrap();

    insta::assert_json_snapshot!(user, @r#"
    {
      "email": "user1@example.com",
      "name": "User One"
    }
    "#);
}

#[tokio::test]
async fn kv_create() {
    let nats = nats_client().await;
    let jet = async_nats::jetstream::new(nats);

    let _ = jet.delete_key_value("createUsers").await;

    let kv_config = kv::Config {
        bucket: String::from("createUsers"),
        ..Default::default()
    };

    let bucket = jet.create_key_value(kv_config).await.unwrap();

    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        mutation {
          kvCreateUser(id: 1,input: { email: "user1@example.com", name: "User One" })
        }
    "#};

    let response = gateway.query(query).send().await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "kvCreateUser": "1"
      }
    }
    "#);

    let user = bucket.get("user.1").await.unwrap().unwrap();
    let user: serde_json::Value = serde_json::from_slice(user.as_ref()).unwrap();

    insta::assert_json_snapshot!(user, @r#"
    {
      "email": "user1@example.com",
      "name": "User One"
    }
    "#);
}

#[tokio::test]
async fn kv_delete() {
    let nats = nats_client().await;
    let jet = async_nats::jetstream::new(nats);

    let _ = jet.delete_key_value("deleteUsers").await;

    let kv_config = kv::Config {
        bucket: String::from("deleteUsers"),
        ..Default::default()
    };

    let bucket = jet.create_key_value(kv_config).await.unwrap();

    let user = json!({
        "email": "user1@example.com",
        "name": "User One"
    });

    bucket
        .create("user.1", serde_json::to_vec(&user).unwrap().into())
        .await
        .unwrap();

    let gateway = gateway_builder().build().await.unwrap();

    let query = indoc! {r#"
        mutation {
          kvDeleteUser(id: 1)
        }
    "#};

    let response = gateway.query(query).send().await;
    insta::assert_json_snapshot!(response, @r#"
    {
      "data": {
        "kvDeleteUser": true
      }
    }
    "#);

    assert!(bucket.get("user.1").await.unwrap().is_none());
}
