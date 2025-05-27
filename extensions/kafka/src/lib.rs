mod config;
mod directives;

use std::{collections::HashMap, str::FromStr, time::Duration};

use directives::{DirectiveKind, KafkaProducerBatching, KafkaPublish};
use grafbase_sdk::{
    FieldResolverExtension, Subscription,
    host_io::kafka::{self, KafkaAuthentication, KafkaBatchConfig, KafkaProducerConfig, KafkaTlsConfig},
    types::{
        Configuration, Error, FieldDefinitionDirective, FieldInputs, FieldOutputs, SchemaDirective, SubgraphHeaders,
    },
};

#[derive(FieldResolverExtension)]
struct Kafka {
    producers: HashMap<String, kafka::KafkaProducer>,
}

impl FieldResolverExtension for Kafka {
    fn new(schema_directives: Vec<SchemaDirective>, config: Configuration) -> Result<Self, Error> {
        let config: config::KafkaConfig = config.deserialize()?;
        let mut configs = HashMap::new();
        let mut producers = HashMap::new();

        for endpoint in config.endpoints {
            configs.insert(endpoint.name.clone(), endpoint);
        }

        for directive in schema_directives.into_iter() {
            if directive.name() != "kafkaProducer" {
                continue;
            }

            let directives::KafkaProducer {
                name,
                provider,
                topic,
                config,
            } = directive.arguments()?;

            let Some(endpoint) = configs.get(&provider) else {
                return Err(Error::new(format!("Kafka endpoint not found: {provider}")));
            };

            let mut producer_config = KafkaProducerConfig::default();

            if let Some(compression) = config.compression {
                producer_config.compression(compression.into());
            }

            if let Some(partitions) = config.partitions {
                let partitions = partitions.into_iter().map(|partition| partition as i32).collect();
                producer_config.partitions(partitions);
            }

            if let Some(KafkaProducerBatching {
                max_size_bytes,
                linger_ms,
            }) = config.batch
            {
                producer_config.batching(KafkaBatchConfig {
                    max_size_bytes,
                    linger: Duration::from_millis(linger_ms),
                });
            }

            if let Some(tls_config) = endpoint.tls.as_ref() {
                let tls_config = match tls_config {
                    config::TlsConfig::SystemCa => KafkaTlsConfig::system_ca(),
                    config::TlsConfig::CustomCa { ca_path } => KafkaTlsConfig::custom_ca(ca_path),
                };

                producer_config.tls(tls_config);
            }

            if let Some(auth_config) = endpoint.authentication.as_ref() {
                let auth_config = match auth_config {
                    config::AuthenticationConfig::SaslPlain { username, password } => {
                        KafkaAuthentication::sasl_plain(username, password)
                    }
                    config::AuthenticationConfig::SaslScram {
                        username,
                        password,
                        mechanism,
                    } => match mechanism {
                        config::SaslScramMechanism::Sha256 => {
                            KafkaAuthentication::sasl_scram_sha256(username, password)
                        }
                        config::SaslScramMechanism::Sha512 => {
                            KafkaAuthentication::sasl_scram_sha512(username, password)
                        }
                    },
                    config::AuthenticationConfig::Mtls { certificate, key } => {
                        KafkaAuthentication::mtls(certificate, key)
                    }
                };

                producer_config.authentication(auth_config);
            }

            let producer = kafka::producer(&name, &endpoint.bootstrap_servers, &topic, producer_config)?;
            producers.insert(name, producer);
        }

        Ok(Self { producers })
    }

    fn resolve_field(
        &mut self,
        _: SubgraphHeaders,
        _: &str,
        directive: FieldDefinitionDirective<'_>,
        inputs: FieldInputs,
    ) -> Result<FieldOutputs, Error> {
        let Ok(directive_kind) = DirectiveKind::from_str(directive.name()) else {
            return Err(format!("Invalid directive: {}", directive.name()).into());
        };

        match directive_kind {
            DirectiveKind::Publish => {
                let args = directive
                    .arguments()
                    .map_err(|e| format!("Error deserializing directive arguments: {e}"))?;

                self.publish(args, inputs)
            }
        }
    }

    fn resolve_subscription<'a>(
        &'a mut self,
        _headers: SubgraphHeaders,
        _subgraph_name: &str,
        _directive: FieldDefinitionDirective<'_>,
    ) -> Result<Box<dyn Subscription + 'a>, Error> {
        todo!("Subscriptions are not supported yet")
    }
}

impl Kafka {
    fn publish(&self, args: KafkaPublish<'_>, inputs: FieldInputs) -> Result<FieldOutputs, Error> {
        let Some(producer) = self.producers.get(args.producer) else {
            return Err(format!("Producer not found: {}", args.producer).into());
        };

        let body = args.body().unwrap_or(&serde_json::Value::Null);
        let body = serde_json::to_vec(body).unwrap();

        producer.produce(args.key, &body)?;

        Ok(FieldOutputs::new(inputs, true)?)
    }
}
