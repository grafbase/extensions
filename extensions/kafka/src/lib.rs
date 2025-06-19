mod config;
mod directives;
mod subscription;

use std::{cell::RefCell, rc::Rc, time::Duration};

use fxhash::FxHashMap;
use grafbase_sdk::{
    IntoSubscription, ResolverExtension,
    host_io::kafka::{self, KafkaBatchConfig, KafkaConsumerConfig, KafkaProducerConfig},
    jq_selection::JqSelection,
    types::{Configuration, Error, ResolvedField, Response, SubgraphHeaders, SubgraphSchema, Variables},
};
use regex::Regex;
use serde_json::Value;
use subscription::FilteredSubscription;
use template::Templates;

use crate::{directives::*, subscription::DeduplicationKey};

#[derive(ResolverExtension)]
struct Kafka {
    producers: FxHashMap<String, kafka::KafkaProducer>,
    endpoints: FxHashMap<String, config::Endpoint>,
    templates: Templates,
    jq_selection: Rc<RefCell<JqSelection>>,
    key_filters: FxHashMap<String, Regex>,
}

impl ResolverExtension for Kafka {
    fn new(schemas: Vec<SubgraphSchema<'_>>, config: Configuration) -> Result<Self, Error> {
        let config: config::KafkaConfig = config.deserialize()?;
        let mut endpoints = FxHashMap::default();
        let mut producers = FxHashMap::default();

        for endpoint in config.endpoints {
            endpoints.insert(endpoint.name.clone(), endpoint);
        }

        for schema in schemas {
            for directive in schema.directives() {
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
        }

        let jq_selection = Rc::new(RefCell::new(JqSelection::default()));

        Ok(Self {
            producers,
            endpoints,
            jq_selection,
            templates: Default::default(),
            key_filters: Default::default(),
        })
    }

    fn resolve(&mut self, prepared: &[u8], _headers: SubgraphHeaders, variables: Variables) -> Result<Response, Error> {
        let field = ResolvedField::try_from(prepared)?;
        let arguments: Value = field.arguments(&variables)?;
        let ctx = serde_json::json!({
            "args": arguments,
        });

        let KafkaPublish { producer, key, body } = match KafkaDirective::try_from(field.directive())? {
            KafkaDirective::Publish(args) => args,
            KafkaDirective::Subscription(_) => {
                return Err(format!("@{KAFKA_SUBSCRIPTION} can only be used on subscription fields.").into());
            }
        };

        let key = key
            .map(|key| self.templates.get_or_insert(key).map(|t| t.render_unescaped(&ctx)))
            .transpose()?;
        let body = self.render_body(body, ctx)?;
        let Some(producer) = self.producers.get(producer) else {
            return Err(format!("Producer not found: {}", producer).into());
        };

        let body = serde_json::to_vec(&body).map_err(|err| format!("Failed to serialize body: {err}"))?;

        producer.produce(key.as_deref(), &body)?;

        Ok(Response::data(true))
    }

    fn resolve_subscription<'s>(
        &'s mut self,
        prepared: &'s [u8],
        _headers: SubgraphHeaders,
        variables: Variables,
    ) -> Result<impl IntoSubscription<'s>, Error> {
        let field = ResolvedField::try_from(prepared)?;
        let KafkaSubscription {
            provider,
            topic,
            selection,
            key_filter,
            consumer_config,
        } = field
            .directive()
            .arguments()
            .map_err(|e| format!("Error deserializing directive arguments: {e}"))?;
        let arguments: Value = field.arguments(&variables)?;
        let ctx = serde_json::json!({
            "args": arguments,
        });

        let topic = self.templates.get_or_insert(topic)?.render_unescaped(&ctx);
        let selection = selection
            .map(|s| self.templates.get_or_insert(&s).map(|t| t.render_json(&ctx)))
            .transpose()?;
        let key_filter = key_filter
            .map(|s| self.templates.get_or_insert(s).map(|t| t.render_unescaped(&ctx)))
            .transpose()?;

        let key = postcard::to_stdvec(&DeduplicationKey {
            provider,
            topic: &topic,
            key_filter: key_filter.as_deref(),
            selection: selection.as_deref(),
        })
        .ok();

        let callback = move || {
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

            let consumer = kafka::consumer(&endpoint.bootstrap_servers, &topic, config)?;

            let key_filter = match key_filter {
                Some(filter) => match self.key_filters.get(&filter) {
                    Some(filter) => Some(filter.clone()),
                    None => {
                        let regex = Regex::new(&filter).map_err(|e| {
                            let msg = format!("Invalid key filter regex: {e}");
                            Error::from(msg)
                        })?;

                        Some(regex)
                    }
                },
                None => None,
            };

            Ok(FilteredSubscription::new(
                consumer,
                self.jq_selection.clone(),
                selection,
                key_filter,
            ))
        };

        Ok((key, callback))
    }
}

impl Kafka {
    fn render_body(&mut self, body: Body<'_>, ctx: Value) -> Result<Value, Error> {
        match body.into_case() {
            Some(BodyCase::Selection(source)) => self.render_jq_template(source, ctx),
            Some(BodyCase::Static(value)) => Ok(value),
            None => Ok(Value::Null),
        }
    }

    fn render_jq_template(&mut self, source: &str, ctx: Value) -> Result<Value, Error> {
        let selection = self.templates.get_or_insert(source)?.render_json(&ctx);
        let mut values = self
            .jq_selection
            .borrow_mut()
            .select(&selection, ctx)
            .map_err(|e| format!("Failed to filter with selection: {}", e))?
            .collect::<Result<Vec<Value>, _>>()
            .map_err(|e| format!("Failed to collect filtered value: {}", e))?;
        // TODO: Be smarter, but not sure how with jq...
        if values.len() == 1 {
            Ok(values.pop().unwrap())
        } else {
            Ok(serde_json::Value::Array(values))
        }
    }
}
