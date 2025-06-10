mod config;
mod subscription;
mod types;

use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Duration};

use config::AuthConfig;
use grafbase_sdk::{
    host_io::nats::{self, NatsAuth, NatsClient, NatsStreamConfig},
    jq_selection::JqSelection,
    types::{Configuration, Data, Error, ResolvedField, Response, SubgraphHeaders, SubgraphSchema, Variables},
    IntoSubscription, ResolverExtension,
};
use serde_json::Value;
use subscription::FilteredSubscription;
use template::Templates;
use types::*;

use crate::types::NatsDirective;

#[derive(ResolverExtension)]
struct Nats {
    clients: HashMap<String, NatsClient>,
    templates: Templates,
    jq_selection: Rc<RefCell<JqSelection>>,
}

impl ResolverExtension for Nats {
    fn new(_schemas: Vec<SubgraphSchema<'_>>, config: Configuration) -> Result<Self, Error> {
        let mut clients = HashMap::new();
        let config: config::NatsConfig = config.deserialize()?;

        for endpoint in config.endpoints {
            let auth = match endpoint.authentication {
                Some(AuthConfig::UsernamePassword { username, password }) => {
                    Some(NatsAuth::UsernamePassword((username, password)))
                }
                Some(AuthConfig::Token { token }) => Some(NatsAuth::Token(token)),
                Some(AuthConfig::Credentials { credentials }) => Some(NatsAuth::Credentials(credentials)),
                None => None,
            };

            let client = match auth {
                Some(ref auth) => nats::connect_with_auth(endpoint.servers, auth)?,
                None => nats::connect(endpoint.servers)?,
            };

            clients.insert(endpoint.name, client);
        }

        Ok(Self {
            clients,
            templates: Default::default(),
            jq_selection: Rc::new(RefCell::new(JqSelection::default())),
        })
    }

    fn resolve(&mut self, prepared: &[u8], _headers: SubgraphHeaders, variables: Variables) -> Result<Response, Error> {
        let field = ResolvedField::try_from(prepared)?;
        let arguments: Value = field.arguments(&variables)?;
        let ctx = serde_json::json!({
            "args": arguments,
        });
        match NatsDirective::try_from(field.directive())? {
            NatsDirective::Publish(args) => self.publish(args, ctx),
            NatsDirective::Request(args) => self.request(args, ctx),
            NatsDirective::KeyValue(args) => self.key_value(args, ctx),
            NatsDirective::Subscribe(_) => {
                return Err(format!("@{NATS_SUBSCRIBE} can only be used on subscription fields.").into())
            }
        }
        .map(Into::into)
    }

    fn resolve_subscription<'s>(
        &'s mut self,
        prepared: &'s [u8],
        _headers: SubgraphHeaders,
        variables: Variables,
    ) -> Result<impl IntoSubscription<'s>, Error> {
        let field = ResolvedField::try_from(prepared)?;
        let mut key = Vec::new();

        key.extend(field.subgraph_name().as_bytes());
        key.extend(field.directive().name().as_bytes());
        key.extend(u32::from(field.definition_id()).to_ne_bytes());
        key.extend(field.directive().arguments_bytes());

        let callback = move || {
            let args: SubscribeArguments<'_> = field
                .directive()
                .arguments()
                .map_err(|e| format!("Error deserializing directive arguments: {e}"))?;
            let arguments: Value = field.arguments(&variables)?;
            let ctx = serde_json::json!({
                "args": arguments,
            });

            let Some(client) = self.clients.get(args.provider) else {
                return Err(format!("NATS provider not found: {}", args.provider).into());
            };

            let subject = self.templates.get_or_insert(args.subject)?.render_unescaped(&ctx);
            let selection = args
                .selection
                .map(|s| self.templates.get_or_insert(s).map(|t| t.render_json(&ctx)))
                .transpose()?;

            let stream_config = args.stream_config.map(|config| {
                let mut stream_config = NatsStreamConfig::new(
                    config.stream_name.to_string(),
                    config.consumer_name.to_string(),
                    config.deliver_policy(),
                    Duration::from_millis(config.inactive_threshold_ms),
                );

                if let Some(name) = config.durable_name {
                    stream_config = stream_config.with_durable_name(name.to_string());
                }

                if let Some(description) = config.description {
                    stream_config = stream_config.with_description(description.to_string());
                }

                stream_config
            });

            let subscriber = client
                .subscribe(&subject, stream_config)
                .map_err(|e| format!("Failed to subscribe to subject '{}': {e}", subject))?;

            Ok(FilteredSubscription::new(
                subscriber,
                self.jq_selection.clone(),
                selection,
            ))
        };

        Ok((key, callback))
    }
}

impl Nats {
    fn publish(&mut self, args: PublishArguments<'_>, ctx: Value) -> Result<Data, Error> {
        let subject = self.templates.get_or_insert(args.subject)?.render_unescaped(&ctx);
        let payload = self.render_body(args.body, ctx)?;
        let Some(client) = self.clients.get(args.provider) else {
            return Err(format!("NATS provider not found: {}", args.provider).into());
        };

        let result = client.publish(&subject, &payload);

        Ok(Data::new(result.is_ok())?)
    }

    fn request(&mut self, args: RequestArguments<'_>, ctx: Value) -> Result<Data, Error> {
        let subject = self.templates.get_or_insert(args.subject)?.render_unescaped(&ctx);
        let payload = self.render_body(args.body, ctx)?;
        let Some(client) = self.clients.get(args.provider) else {
            return Err(format!("NATS provider not found: {}", args.provider).into());
        };

        let message = client
            .request::<_, Value>(&subject, &payload, Some(args.timeout))
            .map_err(|e| format!("Failed to request message: {}", e))?;

        let message = match args.selection {
            Some(selection) => self.render_jq_template(selection, message)?,
            None => message,
        };

        Ok(Data::new(message)?)
    }

    fn key_value(&mut self, args: KeyValueArguments<'_>, ctx: Value) -> Result<Data, Error> {
        let Some(client) = self.clients.get(args.provider) else {
            return Err(format!("NATS provider not found: {}", args.provider).into());
        };

        let bucket = self.templates.get_or_insert(args.bucket)?.render_unescaped(&ctx);

        let store = client
            .key_value(&bucket)
            .map_err(|e| format!("Failed to get key-value store: {e}"))?;

        let key = self.templates.get_or_insert(args.key)?.render_unescaped(&ctx);

        match args.action {
            KeyValueAction::Create => {
                let payload = self.render_body(args.body.unwrap_or_default(), ctx)?;

                match store.create(&key, &payload) {
                    Ok(sequence) => Ok(Data::new(sequence.to_string())?),
                    Err(error) => Err(format!("Failed to create key-value pair: {error}").into()),
                }
            }
            KeyValueAction::Put => {
                let payload = self.render_body(args.body.unwrap_or_default(), ctx)?;

                match store.put(&key, &payload) {
                    Ok(sequence) => Ok(Data::new(sequence.to_string())?),
                    Err(error) => Err(format!("Failed to put key-value pair: {error}").into()),
                }
            }
            KeyValueAction::Get => {
                let value = match store.get::<Value>(&key) {
                    Ok(Some(value)) => value,
                    Ok(None) => return Ok(Data::new(serde_json::Value::Null)?),
                    Err(error) => {
                        return Err(format!("Failed to get key-value pair: {error}").into());
                    }
                };

                let value = match args.selection {
                    Some(selection) => self.render_jq_template(selection, value)?,
                    None => value,
                };

                Ok(Data::new(value)?)
            }
            KeyValueAction::Delete => match store.delete(&key) {
                Ok(()) => Ok(Data::new(true)?),
                Err(error) => Err(format!("Failed to delete key-value pair: {error}").into()),
            },
        }
    }

    fn render_body(&mut self, body: Body<'_>, ctx: Value) -> Result<Value, Error> {
        match body.into_case() {
            Some(BodyCase::Selection(source)) => self.render_jq_template(source, ctx),
            Some(BodyCase::Static(value)) => Ok(value),
            None => Ok(Value::Null),
        }
    }

    fn render_jq_template(&mut self, source: &str, ctx: Value) -> Result<Value, Error> {
        let selection = self.templates.get_or_insert(source)?.render_json(&ctx);
        let value = self
            .jq_selection
            .borrow_mut()
            .select(&selection, ctx)
            .map_err(|e| format!("Failed to filter with selection: {}", e))?
            .collect::<Result<Value, _>>()
            .map_err(|e| format!("Failed to collect filtered value: {}", e))?;
        Ok(value)
    }
}
