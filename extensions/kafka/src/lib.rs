mod config;
mod directives;
mod subscription;

use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr, time::Duration};

use directives::{DirectiveKind, KafkaConsumerStartOffset, KafkaProducerBatching, KafkaPublish, KafkaSubscription};
use grafbase_sdk::{
    FieldResolverExtension, Subscription,
    host_io::kafka::{self, KafkaBatchConfig, KafkaConsumerConfig, KafkaProducerConfig},
    jq_selection::JqSelection,
    types::{
        Configuration, Error, FieldDefinitionDirective, FieldInputs, FieldOutputs, SchemaDirective, SubgraphHeaders,
    },
};
use regex::Regex;
use subscription::FilteredSubscription;

#[derive(FieldResolverExtension)]
struct Kafka {
    producers: HashMap<String, kafka::KafkaProducer>,
    endpoints: HashMap<String, config::Endpoint>,
    jq_selection: Rc<RefCell<JqSelection>>,
}

impl FieldResolverExtension for Kafka {
    fn new(schema_directives: Vec<SchemaDirective>, config: Configuration) -> Result<Self, Error> {
        let config: config::KafkaConfig = config.deserialize()?;
        let mut endpoints = HashMap::new();
        let mut producers = HashMap::new();

        for endpoint in config.endpoints {
            endpoints.insert(endpoint.name.clone(), endpoint);
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

            let Some(endpoint) = endpoints.get(&provider) else {
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
                producer_config.tls(tls_config.clone().into());
            }

            if let Some(auth_config) = endpoint.authentication.as_ref() {
                producer_config.authentication(auth_config.clone().into());
            }

            let producer = kafka::producer(&name, &endpoint.bootstrap_servers, &topic, producer_config)?;
            producers.insert(name, producer);
        }

        let jq_selection = Rc::new(RefCell::new(JqSelection::default()));

        Ok(Self {
            producers,
            endpoints,
            jq_selection,
        })
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

        let DirectiveKind::Publish = directive_kind else {
            let msg = "The @kafkaSubscription directive must be used in a subscription field";
            return Err(msg.to_string().into());
        };

        let args: KafkaPublish<'_> = directive
            .arguments()
            .map_err(|e| format!("Error deserializing directive arguments: {e}"))?;

        let Some(producer) = self.producers.get(args.producer) else {
            return Err(format!("Producer not found: {}", args.producer).into());
        };

        let body = args.body().unwrap_or(&serde_json::Value::Null);
        let body = serde_json::to_vec(body).unwrap();

        producer.produce(args.key, &body)?;

        Ok(FieldOutputs::new(inputs, true)?)
    }

    fn resolve_subscription<'a>(
        &'a mut self,
        _headers: SubgraphHeaders,
        _subgraph_name: &str,
        directive: FieldDefinitionDirective<'_>,
    ) -> Result<Box<dyn Subscription + 'a>, Error> {
        let Ok(directive_kind) = DirectiveKind::from_str(directive.name()) else {
            return Err(format!("Invalid directive: {}", directive.name()).into());
        };

        let DirectiveKind::Subscription = directive_kind else {
            let msg = String::from("The @kafkaPublish directive must be used in a mutation field");
            return Err(msg.into());
        };

        let KafkaSubscription {
            provider,
            topic,
            selection,
            key_filter,
            consumer_config,
        } = directive
            .arguments()
            .map_err(|e| format!("Error deserializing directive arguments: {e}"))?;

        let Some(endpoint) = self.endpoints.get(provider) else {
            return Err(format!("Provider not found: {provider}").into());
        };

        let mut config = KafkaConsumerConfig::default();

        if let Some(max_batch_size) = consumer_config.max_batch_size {
            config.max_batch_size(max_batch_size);
        }

        if let Some(min_batch_size) = consumer_config.min_batch_size {
            config.min_batch_size(min_batch_size);
        }

        match consumer_config.start_offset() {
            KafkaConsumerStartOffset::Latest => config.from_latest_offset(),
            KafkaConsumerStartOffset::Earliest => config.from_earliest_offset(),
            KafkaConsumerStartOffset::Specific(offset) => config.from_specific_offset(offset),
        }

        if let Some(partitions) = consumer_config.partitions {
            config.partitions(partitions);
        }

        if let Some(max_wait_time) = consumer_config.max_wait_time_ms {
            let Ok(max_wait_time) = u64::try_from(max_wait_time) else {
                let msg = format!("Invalid max_wait_time (must be a positive integer): {max_wait_time}");
                return Err(msg.into());
            };

            config.max_wait_time(Duration::from_millis(max_wait_time));
        }

        if let Some(tls) = endpoint.tls.as_ref() {
            config.tls(tls.clone().into());
        }

        if let Some(authentication) = endpoint.authentication.as_ref() {
            config.authentication(authentication.clone().into());
        }

        let consumer = kafka::consumer(&endpoint.bootstrap_servers, topic, config)?;

        let key_filter = match key_filter {
            Some(filter) => Some(Regex::new(filter).map_err(|e| {
                let msg = format!("Invalid key filter regex: {e}");
                Error::from(msg)
            })?),
            None => None,
        };

        Ok(Box::new(FilteredSubscription::new(
            consumer,
            self.jq_selection.clone(),
            selection,
            key_filter,
        )))
    }

    fn subscription_key(
        &mut self,
        _headers: &SubgraphHeaders,
        subgraph_name: &str,
        directive: FieldDefinitionDirective<'_>,
    ) -> Option<Vec<u8>> {
        let mut identifier = Vec::new();

        identifier.extend(subgraph_name.as_bytes());
        identifier.extend(directive.name().as_bytes());
        identifier.extend(directive.site().parent_type_name().as_bytes());
        identifier.extend(directive.site().name().as_bytes());
        identifier.extend(directive.arguments_bytes());

        Some(identifier)
    }
}
