# Kafka extension

This is a Kafka extension for the Grafbase Gateway. It allows you to publish messages to and subscribe to Kafka topics with full support for authentication, TLS, and advanced producer/consumer configurations.

This extension expects JSON payloads. If you use a different format, fork the extension and modify it to fit your needs. For static formats such as Protobuf or Avro, we recommend customizing the extension.

## Installing

Add the following to your Grafbase Gateway configuration file:

```toml
[extensions.kafka]
version = "0.2"
```

Then run `grafbase extension install`. The extension will be installed in the `grafbase_extensions` directory. That directory must be present when the gateway is started.

## Building From Source

Build this extension manually and copy the artifacts to a location where the gateway can find them until we complete the Grafbase Extension Registry.

```bash
grafbase extension build
```

The `build` directory contains the resulting Wasm component and manifest file.

```bash
build/
├── extension.wasm
└── manifest.json
```

In your gateway configuration, you can now load the extension from the `build` directory.

```toml
[extensions.kafka]
path = "/path/to/build"
```

## Configuration

Configure the extension through the Grafbase Gateway configuration file:

```toml
[[extensions.kafka.config.endpoint]]
name = "default"
bootstrap_servers = ["localhost:9092"]
```

- `name`: The name of the endpoint. This identifies the endpoint in the GraphQL schema. Default is `default`. You can omit the name in the configuration and in the schema if using only one endpoint.
- `bootstrap_servers`: The list of Kafka bootstrap servers to connect to in the format "host:port".

The authentication and TLS configurations are optional, and we support multiple authentication methods:

### TLS Configuration

#### System CA

```toml
[[extensions.kafka.config.endpoint]]
name = "default"
bootstrap_servers = ["kafka.example.com:9092"]

[extensions.kafka.config.endpoint.tls]
system_ca = true
```

#### Custom CA

```toml
[[extensions.kafka.config.endpoint]]
name = "default"
bootstrap_servers = ["kafka.example.com:9092"]

[extensions.kafka.config.endpoint.tls]
custom_ca = { ca_path = "/etc/ssl/certs/kafka-ca.pem" }
```

### Authentication

#### SASL Plain Authentication

```toml
[[extensions.kafka.config.endpoint]]
name = "default"
bootstrap_servers = ["kafka.example.com:9092"]

[extensions.kafka.config.endpoint.authentication]
type = "sasl_plain"
username = "kafka_user"
password = "kafka_password"
```

- `username`: The username to use for SASL Plain authentication.
- `password`: The password to use for SASL Plain authentication.

#### SASL SCRAM Authentication

```toml
[[extensions.kafka.config.endpoint]]
name = "default"
bootstrap_servers = ["kafka.example.com:9092"]

[extensions.kafka.config.endpoint.authentication]
type = "sasl_scram"
username = "kafka_user"
password = "kafka_password"
mechanism = "sha512"
```

- `username`: The username to use for SASL SCRAM authentication.
- `password`: The password to use for SASL SCRAM authentication.
- `mechanism`: The SCRAM mechanism to use. Supported values: `sha256`, `sha512`.

#### Mutual TLS (mTLS) Authentication

```toml
[[extensions.kafka.config.endpoint]]
name = "default"
bootstrap_servers = ["kafka.example.com:9092"]

[extensions.kafka.config.endpoint.authentication]
type = "mtls"
certificate = "/etc/ssl/certs/kafka-client.pem"
key = "/etc/ssl/private/kafka-client.key"
```

- `client_certificate_path`: Path to the client certificate file.
- `client_key_path`: Path to the client private key file.

## Publish

To publish messages to a Kafka topic, you need to first configure a producer using the `@kafkaProducer` directive at the schema level, then use the `@kafkaPublish` directive on field definitions.

### Schema-level Producer Configuration

```graphql
extend schema
  @link(
    url: "https://grafbase.com/extensions/kafka/0.1.1"
    import: ["@kafkaProducer", "@kafkaPublish", "@kafkaSubscription"]
  )
  @kafkaProducer(name: "userEvents", topic: "user-events", provider: "default")
```

- `name`: The name of the Kafka producer. This is used to reference the producer in the `@kafkaPublish` directive. Must be unique within the schema.
- `provider`: The name of the kafka provider in the Grafbase configuration. Default is `default`.
- `topic`: The name of the Kafka topic to publish to.
- `config`: Configuration options for the Kafka producer.
  - `compression`: The compression algorithm to use for messages. Either `GZIP`, `SNAPPY`, `LZ4`, or `ZSTD`. Default is no compression.
  - `partitions`: A list of partition IDs to publish to. If not specified, messages partition will either be decided based on the key, or the partition will be chosen in round-robin order.
  - `batch`: Configuration options for batching messages.
    - `lingerMs`: The time in milliseconds to wait before sending a batch of messages.
    - `maxSizeBytes`: The maximum size in bytes of a batch of messages.

### Field-level Publish Directive

```graphql
directive @kafkaPublish(
  producer: String!
  key: UrlTemplate
  body: Body! = { selection: ".args.input" }
) on FIELD_DEFINITION
```

- `producer`: The name of the Kafka producer configured with `@kafkaProducer`. This must reference a producer defined at the schema level.
- `key`: The message key to use for partitioning. This supports templating using GraphQL arguments: `{{args.myArgument}}`.
- `body`: The body of the message to publish. If not set, takes the body from the field's `input` argument. Can also be set to a static JSON object.

### Example

```graphql
extend schema
  @link(
    url: "https://grafbase.com/extensions/kafka/0.1.1"
    import: ["@kafkaProducer", "@kafkaPublish", "@kafkaSubscription"]
  )
  @kafkaProducer(
    name: "userEventProducer"
    provider: "default"
    topic: "user-events"
    config: {
      compression: GZIP
      partitions: [0, 1, 2]
      batch: { lingerMs: 5, maxSizeBytes: 8192 }
    }
  )

type Mutation {
  publishUserEvent(id: String!, input: UserEventInput!): Boolean!
    @kafkaPublish(producer: "userEventProducer", key: "user-{{args.id}}")
}

input UserEventInput {
  email: String!
  name: String!
  eventType: String!
}
```

This example publishes an event to the `user-events` topic using the configured producer. The message key is set to `user-<id>` where `id` comes from the GraphQL argument. The payload comes from the `input` argument:

```graphql
mutation PublishUserEvent($id: String!, $email: String!, $name: String!) {
  publishUserEvent(
    id: $id
    input: { email: $email, name: $name, eventType: "profile_update" }
  )
}
```

By calling the mutation with id `123`, the following message will be published to the `user-events` topic with key `user-123`:

```json
{
  "email": "john@example.com",
  "name": "John Doe",
  "eventType": "profile_update"
}
```

## Subscribe

To subscribe to messages from a Kafka topic, use the `@kafkaSubscription` directive:

```graphql
directive @kafkaSubscription(
  provider: String! = "default"
  topic: UrlTemplate!
  selection: JsonTemplate
  keyFilter: UrlTemplate
  consumerConfig: KafkaConsumerConfiguration! = {}
) on FIELD_DEFINITION
```

- `provider`: The Kafka provider to use. Default is `default`.
- `topic`: The topic to subscribe to. This supports templating using GraphQL arguments: `{{args.myArgument}}`.
- `selection`: Selection to apply to the subscription payload. In [jq syntax](https://jqlang.org/manual/). This supports templating using GraphQL arguments: `{{args.myArgument}}`.
- `keyFilter`: A regular expression to filter messages by key.
- `consumerConfig`: Consumer configuration for fine-tuning consumption behavior.

The client is expected to know from which offset to start consuming messages. It can either choose to start from the beginning, from the latest, or from a specific offset.

More complex consumer scenarios should be achieved by implementing a proper Kafka consumer and communicating directly with the Kafka broker. Consider the GraphQL subscriptions to be more of a notification pipeline, where the client is responsible for handling the messages and processing them accordingly, and where it is not a big deal if the client skips some messages.

If you expect to build a full Kafka consumer with the extension, with at least, at most, or exactly once delivery guarantees, please [contact us](https://grafbase.com/contact/sales).

### Consumer Configuration

```graphql
input KafkaConsumerConfiguration {
  minBatchSize: Int
  maxBatchSize: Int
  maxWaitTimeMs: Int
  startOffset: KafkaConsumerStartOffset! = { preset: LATEST }
  partitions: [Int!]
}
```

- `minBatchSize`: Minimum number of messages to wait for before processing a batch.
- `maxBatchSize`: Maximum number of messages to process in a single batch.
- `maxWaitTimeMs`: Maximum time in milliseconds to wait for messages before returning an empty batch.
- `startOffset`: Starting offset position for consuming messages from the topic.
- `partitions`: Specific partition numbers to consume from. If not specified, consumes from all partitions.

The start offset configuration:

```graphql
input KafkaConsumerStartOffset @oneOf {
  preset: KafkaConsumerStartOffsetPreset
  offset: Int
}

enum KafkaConsumerStartOffsetPreset {
  LATEST
  EARLIEST
}
```

- `LATEST`: Start consuming from the latest offset available in the topic. This is the default behavior.
- `EARLIEST`: Start consuming from the earliest offset available in the topic.

### Example

#### Basic Subscription

```graphql
type Subscription {
  userEvents(userId: String!): UserEvent!
    @kafkaSubscription(topic: "user-events", keyFilter: "user-{{args.userId}}")
}

type UserEvent {
  email: String!
  name: String!
  eventType: String!
  timestamp: String!
}
```

This example subscribes to the `user-events` topic and filters messages by key using a regular expression pattern. Only messages with keys matching `user-<userId>` will be delivered to the client.

#### Advanced Consumer Configuration

```graphql
type Subscription {
  orderUpdates: OrderUpdate!
    @kafkaSubscription(
      topic: "order-updates"
      consumerConfig: {
        startOffset: { preset: EARLIEST }
        maxBatchSize: 100
        maxWaitTimeMs: 5000
        partitions: [0, 1, 2]
      }
    )
}

type OrderUpdate {
  orderId: String!
  status: String!
  updatedAt: String!
  customerId: String!
}
```

This example subscribes to the `order-updates` topic with advanced consumer configuration. It starts consuming from the earliest available offset, processes up to 100 messages in a batch, waits up to 5 seconds for messages, and only consumes from partitions 0, 1, and 2.

#### Using Selection

```graphql
type Subscription {
  highValueOrders: Order!
    @kafkaSubscription(topic: "orders", selection: "select(.amount > 1000)")
}

type Order {
  id: String!
  amount: Float!
  customerId: String!
  timestamp: String!
}
```

This example subscribes to the `orders` topic but filters the incoming messages using a JQ-style selection to only deliver orders with an amount greater than 1000. This enables server-side filtering of messages before sending them to the client.

The selection also supports dynamic parameters:

```graphql
type Subscription {
  ordersAboveThreshold(minimumAmount: Float!): Order!
    @kafkaSubscription(
      topic: "orders"
      selection: "select(.amount > {{args.minimumAmount}})"
    )
}
```

This example allows clients to set their own filtering criteria by providing a `minimumAmount` argument that gets used in the [jq-style](https://jqlang.org/manual/) selection filter.

## Multiple Providers

You can configure multiple Kafka clusters and reference them in your GraphQL schema:

```toml
[[extensions.kafka.config.endpoint]]
name = "production"
bootstrap_servers = ["prod-kafka-1:9092", "prod-kafka-2:9092"]

[extensions.kafka.config.endpoint.tls]
system_ca = true

[extensions.kafka.config.endpoint.authentication]
sasl_scram = { username = "prod_user", password = "prod_password", mechanism = "sha512" }

[[extensions.kafka.config.endpoint]]
name = "analytics"
bootstrap_servers = ["analytics-kafka:9092"]

[extensions.kafka.config.endpoint.authentication]
sasl_plain = { username = "analytics_user", password = "analytics_password" }
```

Then reference the specific provider in your GraphQL schema:

```graphql
schema
  @kafkaProducer(name: "prodEvents", provider: "production", topic: "events")
  @kafkaProducer(
    name: "analyticsEvents"
    provider: "analytics"
    topic: "analytics"
  ) {
  query: Query
  mutation: Mutation
  subscription: Subscription
}

type Mutation {
  logEvent(input: EventInput!): Boolean! @kafkaPublish(producer: "prodEvents")
  trackAnalytics(data: AnalyticsInput!): Boolean!
    @kafkaPublish(producer: "analyticsEvents")
}

type Subscription {
  productionEvents: Event!
    @kafkaSubscription(provider: "production", topic: "events")
  analyticsStream: Analytics!
    @kafkaSubscription(provider: "analytics", topic: "analytics")
}
```

This setup allows you to publish and subscribe to different Kafka clusters based on your application's needs, such as separating production traffic from analytics data.
