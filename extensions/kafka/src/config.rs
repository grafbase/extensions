use std::path::PathBuf;

/// Configuration for multiple named Kafka connections.
#[derive(serde::Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct KafkaConfig {
    /// Named Kafka connection configurations.
    #[serde(rename = "endpoint")]
    pub endpoints: Vec<Endpoint>,
}

/// Configuration for connecting to a single Kafka cluster.
#[derive(serde::Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Endpoint {
    /// A unique name for the endpoint.
    #[serde(default = "default_endpoint_name")]
    pub name: String,
    /// List of bootstrap servers in the format "host:port".
    pub bootstrap_servers: Vec<String>,
    /// Optional TLS configuration for secure connections.
    #[serde(default)]
    pub tls: Option<TlsConfig>,
    /// Optional authentication configuration.
    #[serde(default)]
    pub authentication: Option<AuthenticationConfig>,
}

fn default_endpoint_name() -> String {
    "default".to_string()
}

/// TLS configuration options for secure connections.
#[derive(serde::Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum TlsConfig {
    /// Use the system's certificate authority store.
    SystemCa,
    /// Use a custom certificate authority from a file.
    CustomCa {
        /// Path to the custom CA certificate file.
        ca_path: PathBuf,
    },
}

/// Authentication configuration options for connecting to Kafka.
#[derive(serde::Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum AuthenticationConfig {
    /// SASL PLAIN authentication using username and password.
    SaslPlain {
        /// Username for authentication.
        username: String,
        /// Password for authentication.
        password: String,
    },
    /// SASL SCRAM authentication with configurable mechanism.
    SaslScram {
        /// Username for authentication.
        username: String,
        /// Password for authentication.
        password: String,
        /// SCRAM mechanism to use (SHA-256 or SHA-512).
        mechanism: SaslScramMechanism,
    },
    /// Mutual TLS authentication using client certificates.
    Mtls {
        /// Path to the client certificate file.
        certificate: PathBuf,
        /// Path to the client private key file.
        key: PathBuf,
    },
}

/// SASL SCRAM mechanism variants.
#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SaslScramMechanism {
    /// SCRAM-SHA-256 mechanism.
    Sha256,
    /// SCRAM-SHA-512 mechanism.
    Sha512,
}
