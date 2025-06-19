use std::time::Duration;

use grafbase_sdk::host_io::nats::{self, OffsetDateTime};

pub const NATS_PUBLISH: &str = "natsPublish";
pub const NATS_REQUEST: &str = "natsRequest";
pub const NATS_KEY_VALUE: &str = "natsKeyValue";
pub const NATS_SUBSCRIBE: &str = "natsSubscribtion";

pub enum NatsDirective<'a> {
    Publish(PublishArguments<'a>),
    Request(RequestArguments<'a>),
    KeyValue(KeyValueArguments<'a>),
    #[allow(dead_code)]
    Subscribe(SubscribeArguments<'a>),
}

impl<'a> TryFrom<grafbase_sdk::types::Directive<'a>> for NatsDirective<'a> {
    type Error = grafbase_sdk::types::Error;
    fn try_from(directive: grafbase_sdk::types::Directive<'a>) -> Result<Self, Self::Error> {
        match directive.name() {
            NATS_PUBLISH => Ok(NatsDirective::Publish(directive.arguments()?)),
            NATS_REQUEST => Ok(NatsDirective::Request(directive.arguments()?)),
            NATS_KEY_VALUE => Ok(NatsDirective::KeyValue(directive.arguments()?)),
            NATS_SUBSCRIBE => Ok(NatsDirective::Subscribe(directive.arguments()?)),
            name => Err(format!("Unknown directive: {name}").into()),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishArguments<'a> {
    pub provider: &'a str,
    pub subject: &'a str,
    #[serde(borrow)]
    pub body: Body<'a>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestArguments<'a> {
    pub provider: &'a str,
    pub subject: &'a str,
    #[serde(borrow)]
    pub selection: Option<&'a str>,
    #[serde(rename = "timeoutMs", deserialize_with = "deserialize_duration_from_ms")]
    pub timeout: Duration,
    #[serde(borrow)]
    pub body: Body<'a>,
}

fn deserialize_duration_from_ms<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let ms = u64::deserialize(deserializer)?;

    Ok(Duration::from_millis(ms))
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValueArguments<'a> {
    pub provider: &'a str,
    pub bucket: &'a str,
    pub key: &'a str,
    pub action: KeyValueAction,
    #[serde(borrow)]
    pub selection: Option<&'a str>,
    #[serde(borrow)]
    pub body: Option<Body<'a>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KeyValueAction {
    Create,
    Put,
    Get,
    Delete,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body<'a> {
    #[serde(borrow)]
    pub selection: Option<&'a str>,
    pub r#static: Option<serde_json::Value>,
}

impl<'a> Body<'a> {
    pub fn into_case(self) -> Option<BodyCase<'a>> {
        self.r#static
            .map(BodyCase::Static)
            .or_else(|| self.selection.map(BodyCase::Selection))
    }
}

pub(crate) enum BodyCase<'a> {
    Selection(&'a str),
    Static(serde_json::Value),
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeArguments<'a> {
    pub provider: &'a str,
    pub subject: &'a str,
    #[serde(borrow)]
    pub selection: Option<&'a str>,
    #[serde(borrow)]
    pub stream_config: Option<NatsStreamConfiguration<'a>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NatsStreamConfiguration<'a> {
    pub stream_name: &'a str,
    pub consumer_name: &'a str,
    #[serde(borrow)]
    pub durable_name: Option<&'a str>,
    #[serde(borrow)]
    pub description: Option<&'a str>,
    pub inactive_threshold_ms: u64,
    deliver_policy: NatsStreamDeliverPolicy,
}

impl NatsStreamConfiguration<'_> {
    pub fn deliver_policy(self) -> nats::NatsStreamDeliverPolicy {
        match self.deliver_policy.r#type {
            NatsStreamDeliverPolicyType::All => nats::NatsStreamDeliverPolicy::All,
            NatsStreamDeliverPolicyType::Last => nats::NatsStreamDeliverPolicy::Last,
            NatsStreamDeliverPolicyType::New => nats::NatsStreamDeliverPolicy::New,
            NatsStreamDeliverPolicyType::ByStartSequence => {
                nats::NatsStreamDeliverPolicy::ByStartSequence(self.deliver_policy.start_sequence.unwrap_or(0))
            }
            NatsStreamDeliverPolicyType::ByStartTime => {
                let time = match self.deliver_policy.start_time_ms {
                    Some(ms) => OffsetDateTime::from_unix_timestamp_nanos((ms as i128) * 1_000_000).unwrap(),
                    None => OffsetDateTime::now_utc(),
                };

                nats::NatsStreamDeliverPolicy::ByStartTime(time)
            }
            NatsStreamDeliverPolicyType::LastPerSubject => nats::NatsStreamDeliverPolicy::LastPerSubject,
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NatsStreamDeliverPolicy {
    r#type: NatsStreamDeliverPolicyType,
    start_sequence: Option<u64>,
    start_time_ms: Option<u64>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum NatsStreamDeliverPolicyType {
    All,
    Last,
    New,
    ByStartSequence,
    ByStartTime,
    LastPerSubject,
}
