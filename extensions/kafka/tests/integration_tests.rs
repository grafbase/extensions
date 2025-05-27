use std::{sync::Arc, time::Duration};

use futures_util::{Stream, StreamExt, TryStreamExt, stream::select_all};
use grafbase_sdk::test::{DynamicSchema, ExtensionOnlySubgraph, LogLevel, TestConfig, TestRunner};
use indoc::{formatdoc, indoc};
use rskafka::{
    client::{
        consumer::{StartOffset, StreamConsumerBuilder},
        partition::UnknownTopicHandling,
    },
    record::RecordAndOffset,
};

const TEST_TOPIC: &str = "test-topic-1";

fn subgraph() -> ExtensionOnlySubgraph {
    let extension_path = std::env::current_dir().unwrap().join("build");
    let path_str = format!("file://{}", extension_path.display());

    let schema = formatdoc! {r#"
        extend schema
          @link(url: "https://specs.apollo.dev/federation/v2.0", import: ["@key", "@shareable"])
          @link(
            url: "{path_str}",
            import: [
              "@kafkaProducer",
              "@kafkaPublish",
            ]
          )
          @kafkaProducer(
            name: "user-producer-plain",
            topic: "{TEST_TOPIC}",
          )
          @kafkaProducer(
            name: "user-producer-sasl-plain",
            provider: "sasl-plain",
            topic: "{TEST_TOPIC}",
          )
          @kafkaProducer(
            name: "user-producer-sasl-scram",
            provider: "sasl-scram",
            topic: "{TEST_TOPIC}",
          )
          @kafkaProducer(
            name: "user-producer-tls-no-auth",
            provider: "ssl-plain",
            topic: "{TEST_TOPIC}",
          )
          @kafkaProducer(
            name: "user-producer-mtls",
            provider: "mtls",
            topic: "{TEST_TOPIC}",
          )
          @kafkaProducer(
            name: "user-producer-batched",
            topic: "{TEST_TOPIC}",
            config: {{
              compression: GZIP,
              batch: {{
                maxSizeBytes: 100,
                lingerMs: 1000,
              }},
            }}
          )
          @kafkaProducer(
            name: "user-producer-single-partition",
            topic: "{TEST_TOPIC}",
            config: {{
              partitions: [0],
            }}
          )

        type Query {{
          hello: String!
        }}

        type Mutation {{
          publishUserEvent(id: Int!, input: UserEventInput!): Boolean! @kafkaPublish(
            producer: "user-producer-plain",
            key: "publish.user.{{{{args.id}}}}.events"
          )

          publishUserEventSaslPlain(id: Int!, input: UserEventInput!): Boolean! @kafkaPublish(
            producer: "user-producer-sasl-plain",
            key: "publish.user.sasl-plain.{{{{args.id}}}}.events"
          )

          publishUserEventSaslScram(id: Int!, input: UserEventInput!): Boolean! @kafkaPublish(
            producer: "user-producer-sasl-scram",
            key: "publish.user.sasl-scram.{{{{args.id}}}}.events"
          )

          publishUserEventTlsNoAuth(id: Int!, input: UserEventInput!): Boolean! @kafkaPublish(
            producer: "user-producer-tls-no-auth",
            key: "publish.user.tls-no-auth.{{{{args.id}}}}.events"
          )

          publishUserEventMtls(id: Int!, input: UserEventInput!): Boolean! @kafkaPublish(
            producer: "user-producer-mtls",
            key: "publish.user.mtls.{{{{args.id}}}}.events"
          )

          publishUserEventSinglePartition(id: Int!, input: UserEventInput!): Boolean! @kafkaPublish(
            producer: "user-producer-single-partition",
            key: "publish.user.single-partition.{{{{args.id}}}}.events",
          )
        }}

        input UserEventInput {{
          email: String!
          name: String!
        }}
    "#};

    DynamicSchema::builder(schema)
        .into_extension_only_subgraph("test", &extension_path)
        .unwrap()
}

fn config() -> &'static str {
    indoc! {r#"
        [[extensions.kafka.config.endpoint]]
        bootstrap_servers = ["localhost:9092"]

        [[extensions.kafka.config.endpoint]]
        name = "sasl-plain"
        bootstrap_servers = ["localhost:9093"]

        [extensions.kafka.config.endpoint.authentication]
        type = "sasl_plain"
        username = "testuser"
        password = "testuser-secret"

        [[extensions.kafka.config.endpoint]]
        name = "sasl-scram"
        bootstrap_servers = ["localhost:9094"]

        [extensions.kafka.config.endpoint.authentication]
        type = "sasl_scram"
        username = "testuser"
        password = "testuser-secret"
        mechanism = "sha512"

        [[extensions.kafka.config.endpoint]]
        name = "ssl-plain"
        bootstrap_servers = ["localhost:9095"]

        [extensions.kafka.config.endpoint.tls]
        type = "custom_ca"
        ca_path = "../../docker/kafka/config/ssl/ca-cert"

        [[extensions.kafka.config.endpoint]]
        name = "mtls"
        bootstrap_servers = ["localhost:9096"]

        [extensions.kafka.config.endpoint.tls]
        type = "custom_ca"
        ca_path = "../../docker/kafka/config/ssl/ca-cert"

        [extensions.kafka.config.endpoint.authentication]
        type = "mtls"
        certificate = "../../docker/kafka/config/ssl/client.crt"
        key = "../../docker/kafka/config/ssl/client.key"
    "#}
}

async fn consumer(topic: &str) -> impl Stream<Item = Result<(RecordAndOffset, i64), String>> {
    let client = rskafka::client::ClientBuilder::new(vec!["localhost:9092".to_string()])
        .build()
        .await
        .unwrap();

    let mut streams = Vec::new();

    for partition in 0..=3 {
        let partition_client = client
            .partition_client(topic, partition, UnknownTopicHandling::Error)
            .await
            .unwrap();

        let consumer = StreamConsumerBuilder::new(Arc::new(partition_client), StartOffset::Latest)
            .with_max_wait_ms(10000)
            .build()
            .map_err(|e| e.to_string());

        streams.push(consumer);
    }

    select_all(streams)
}

async fn partition_consumer(topic: &str, partition: i32) -> impl Stream<Item = Result<(RecordAndOffset, i64), String>> {
    let client = rskafka::client::ClientBuilder::new(vec!["localhost:9092".to_string()])
        .build()
        .await
        .unwrap();

    let partition_client = client
        .partition_client(topic, partition, UnknownTopicHandling::Error)
        .await
        .unwrap();

    StreamConsumerBuilder::new(Arc::new(partition_client), StartOffset::Latest)
        .with_max_wait_ms(10000)
        .build()
        .map_err(|e| e.to_string())
}

#[tokio::test]
async fn produce_no_batch() {
    let config = TestConfig::builder()
        .with_subgraph(subgraph())
        .enable_networking()
        .enable_stderr()
        .enable_stdout()
        .log_level(LogLevel::Error)
        .build(config())
        .unwrap();

    let runner = TestRunner::new(config).await.unwrap();

    let (sender, mut recv) = tokio::sync::mpsc::channel(10);

    tokio::spawn(async move {
        let mut consumer = consumer(TEST_TOPIC).await;
        let expected_key = "publish.user.1.events".as_bytes();

        while let Some(Ok((record, _))) = consumer.next().await {
            if record.record.key.as_deref() != Some(expected_key) {
                continue;
            }

            sender.send(record.record).await.unwrap();
        }
    });

    tokio::time::sleep(Duration::from_millis(500)).await;

    let mutation = indoc! {r#"
        mutation {
          publishUserEvent(id: 1, input: { email: "test@example.com", name: "Test User" })
        }
    "#};

    let result: serde_json::Value = runner.graphql_query(mutation).send().await.unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "publishUserEvent": true
      }
    }
    "#);

    let record = recv.recv().await.unwrap();
    let key = record.key.unwrap();
    let key = String::from_utf8_lossy(&key);

    assert_eq!("publish.user.1.events", key);

    let event = record.value.unwrap();
    let event: serde_json::Value = serde_json::from_slice(&event).unwrap();

    insta::assert_json_snapshot!(&event, @r#"
    {
      "email": "test@example.com",
      "name": "Test User"
    }
    "#);
}

#[tokio::test]
async fn produce_single_partition() {
    let config = TestConfig::builder()
        .with_subgraph(subgraph())
        .enable_networking()
        .enable_stderr()
        .enable_stdout()
        .log_level(LogLevel::Error)
        .build(config())
        .unwrap();

    let runner = TestRunner::new(config).await.unwrap();

    let (sender, mut recv) = tokio::sync::mpsc::channel(10);

    tokio::spawn(async move {
        let mut consumer = partition_consumer(TEST_TOPIC, 0).await;
        let expected_key = "publish.user.single-partition.1.events".as_bytes();

        while let Some(Ok((record, _))) = consumer.next().await {
            if record.record.key.as_deref() != Some(expected_key) {
                continue;
            }

            sender.send(record.record).await.unwrap();
        }
    });

    tokio::time::sleep(Duration::from_millis(500)).await;

    let mutation = indoc! {r#"
        mutation {
          publishUserEventSinglePartition(id: 1, input: { email: "test@example.com", name: "Test User" })
        }
    "#};

    let result: serde_json::Value = runner.graphql_query(mutation).send().await.unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "publishUserEventSinglePartition": true
      }
    }
    "#);

    let record = recv.recv().await.unwrap();
    let key = record.key.unwrap();
    let key = String::from_utf8_lossy(&key);

    assert_eq!("publish.user.single-partition.1.events", key);

    let event = record.value.unwrap();
    let event: serde_json::Value = serde_json::from_slice(&event).unwrap();

    insta::assert_json_snapshot!(&event, @r#"
    {
      "email": "test@example.com",
      "name": "Test User"
    }
    "#);
}

#[tokio::test]
async fn produce_batch() {
    let config = TestConfig::builder()
        .with_subgraph(subgraph())
        .enable_networking()
        .enable_stderr()
        .enable_stdout()
        .log_level(LogLevel::Error)
        .build(config())
        .unwrap();

    let runner = TestRunner::new(config).await.unwrap();

    let (sender, mut recv) = tokio::sync::mpsc::channel(10);

    tokio::spawn(async move {
        let mut consumer = consumer(TEST_TOPIC).await;
        let expected_key = "publish.user.1.events".as_bytes();

        while let Some(Ok((record, _))) = consumer.next().await {
            if record.record.key.as_deref() != Some(expected_key) {
                continue;
            }

            sender.send(record.record).await.unwrap();
        }
    });

    tokio::time::sleep(Duration::from_millis(500)).await;

    let mutation = indoc! {r#"
        mutation {
          publishUserEvent(id: 1, input: { email: "test@example.com", name: "Test User" })
        }
    "#};

    let result: serde_json::Value = runner.graphql_query(mutation).send().await.unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "publishUserEvent": true
      }
    }
    "#);

    let mutation = indoc! {r#"
        mutation {
          publishUserEvent(id: 1, input: { email: "test2@example.com", name: "Test User 2" })
        }
    "#};

    let result: serde_json::Value = runner.graphql_query(mutation).send().await.unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "publishUserEvent": true
      }
    }
    "#);

    let mutation = indoc! {r#"
        mutation {
          publishUserEvent(id: 1, input: { email: "test3@example.com", name: "Test User 3" })
        }
    "#};

    let result: serde_json::Value = runner.graphql_query(mutation).send().await.unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "publishUserEvent": true
      }
    }
    "#);

    let mut events = Vec::new();

    while let Some(record) = recv.recv().await {
        let event = record.value.unwrap();
        let event: serde_json::Value = serde_json::from_slice(&event).unwrap();

        events.push(event);

        if events.len() == 2 {
            break;
        }
    }

    let events = serde_json::Value::Array(events);

    insta::assert_json_snapshot!(&events, @r#"
    [
      {
        "email": "test@example.com",
        "name": "Test User"
      },
      {
        "email": "test2@example.com",
        "name": "Test User 2"
      }
    ]
    "#);
}

#[tokio::test]
async fn connect_sasl_plain() {
    let config = TestConfig::builder()
        .with_subgraph(subgraph())
        .enable_networking()
        .enable_stderr()
        .enable_stdout()
        .log_level(LogLevel::Error)
        .build(config())
        .unwrap();

    let runner = TestRunner::new(config).await.unwrap();

    let mutation = indoc! {r#"
        mutation {
          publishUserEventSaslPlain(id: 1, input: { email: "test@example.com", name: "Test User" })
        }
    "#};

    let result: serde_json::Value = runner.graphql_query(mutation).send().await.unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "publishUserEventSaslPlain": true
      }
    }
    "#);
}

#[tokio::test]
async fn connect_sasl_scram() {
    let config = TestConfig::builder()
        .with_subgraph(subgraph())
        .enable_networking()
        .enable_stderr()
        .enable_stdout()
        .log_level(LogLevel::Error)
        .build(config())
        .unwrap();

    let runner = TestRunner::new(config).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let mutation = indoc! {r#"
        mutation {
            publishUserEventSaslScram(id: 1, input: { email: "test@example.com", name: "Test User" })
        }
    "#};

    let result: serde_json::Value = runner.graphql_query(mutation).send().await.unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "publishUserEventSaslScram": true
      }
    }
    "#);
}

#[tokio::test]
async fn connect_tls_no_auth() {
    let config = TestConfig::builder()
        .with_subgraph(subgraph())
        .enable_networking()
        .enable_stderr()
        .enable_stdout()
        .log_level(LogLevel::Error)
        .build(config())
        .unwrap();

    let runner = TestRunner::new(config).await.unwrap();

    let mutation = indoc! {r#"
        mutation {
            publishUserEventTlsNoAuth(id: 1, input: { email: "test@example.com", name: "Test User" })
        }
    "#};

    let result: serde_json::Value = runner.graphql_query(mutation).send().await.unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "publishUserEventTlsNoAuth": true
      }
    }
    "#);
}

#[tokio::test]
async fn connect_mtls() {
    let config = TestConfig::builder()
        .with_subgraph(subgraph())
        .enable_networking()
        .enable_stderr()
        .enable_stdout()
        .log_level(LogLevel::Error)
        .build(config())
        .unwrap();

    let runner = TestRunner::new(config).await.unwrap();

    let mutation = indoc! {r#"
        mutation {
            publishUserEventMtls(id: 1, input: { email: "test@example.com", name: "Test User" })
        }
    "#};

    let result: serde_json::Value = runner.graphql_query(mutation).send().await.unwrap();

    insta::assert_json_snapshot!(result, @r#"
    {
      "data": {
        "publishUserEventMtls": true
      }
    }
    "#);
}
