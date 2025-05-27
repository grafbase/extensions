use std::str::FromStr;

use grafbase_sdk::host_io::kafka;

/// A Kafka producer configuration.
#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KafkaProducer {
    /// The name of the Kafka producer.
    pub name: String,
    /// The provider for the Kafka service.
    pub provider: String,
    /// The Kafka topic to produce messages to.
    pub topic: String,
    /// Optional producer configuration settings.
    #[serde(default)]
    pub config: KafkaProducerConfiguration,
}

/// Configuration settings for a Kafka producer.
#[derive(serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct KafkaProducerConfiguration {
    /// Optional compression algorithm to use for messages.
    pub compression: Option<KafkaProducerCompression>,
    /// List of partition IDs to produce to.
    pub partitions: Option<Vec<u32>>,
    /// Optional batching configuration for messages.
    pub batch: Option<KafkaProducerBatching>,
}

/// Configuration for batching messages in a Kafka producer.
#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KafkaProducerBatching {
    /// Maximum number of messages in a batch.
    pub max_size_bytes: u64,
    /// Maximum time to wait before sending a batch (in milliseconds).
    pub linger_ms: u64,
}

/// Compression algorithms supported by Kafka producers.
#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KafkaProducerCompression {
    /// GZIP compression.
    Gzip,
    /// Snappy compression.
    Snappy,
    /// LZ4 compression.
    Lz4,
    /// ZSTD compression.
    Zstd,
}

impl From<KafkaProducerCompression> for kafka::KafkaProducerCompression {
    fn from(value: KafkaProducerCompression) -> Self {
        match value {
            KafkaProducerCompression::Gzip => kafka::KafkaProducerCompression::Gzip,
            KafkaProducerCompression::Snappy => kafka::KafkaProducerCompression::Snappy,
            KafkaProducerCompression::Lz4 => kafka::KafkaProducerCompression::Lz4,
            KafkaProducerCompression::Zstd => kafka::KafkaProducerCompression::Zstd,
        }
    }
}

/// Represents the different kinds of Kafka directives that can be used.
#[derive(Debug)]
pub enum DirectiveKind {
    /// A publish directive for sending messages to Kafka.
    Publish,
}

impl FromStr for DirectiveKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kafkaPublish" => Ok(DirectiveKind::Publish),
            _ => Err(format!("Unknown directive: {}", s)),
        }
    }
}

/// Configuration for publishing messages to Kafka.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaPublish<'a> {
    /// The name of the Kafka producer to use for publishing.
    pub producer: &'a str,
    /// Optional key for the Kafka message.
    pub key: Option<&'a str>,
    /// The message body configuration.
    body: Option<Body>,
}

impl KafkaPublish<'_> {
    /// Returns the body content as a JSON value.
    pub fn body(&self) -> Option<&serde_json::Value> {
        self.body.as_ref().and_then(|body| {
            body.r#static
                .as_ref()
                .or_else(|| body.selection.as_ref().and_then(|s| s.input.as_ref()))
        })
    }
}

/// Represents the body of a Kafka message.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    /// Dynamic body content based on selection.
    pub selection: Option<BodyInput>,
    /// Static body content as a JSON value.
    pub r#static: Option<serde_json::Value>,
}

/// Input configuration for dynamic body content.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyInput {
    /// The input value as JSON.
    input: Option<serde_json::Value>,
}
