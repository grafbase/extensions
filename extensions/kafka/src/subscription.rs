use std::{cell::RefCell, rc::Rc};

use grafbase_sdk::{
    Subscription,
    host_io::kafka,
    jq_selection::JqSelection,
    types::{Error, SubscriptionOutput},
};
use regex::Regex;

pub struct FilteredSubscription {
    kafka: kafka::KafkaConsumer,
    jq_selection: Rc<RefCell<JqSelection>>,
    selection: Option<String>,
    key_filter: Option<Regex>,
}

impl FilteredSubscription {
    pub fn new(
        kafka: kafka::KafkaConsumer,
        jq_selection: Rc<RefCell<JqSelection>>,
        selection: Option<String>,
        key_filter: Option<Regex>,
    ) -> Self {
        Self {
            kafka,
            jq_selection,
            selection,
            key_filter,
        }
    }
}

impl Subscription for FilteredSubscription {
    fn next(&mut self) -> Result<Option<SubscriptionOutput>, Error> {
        let item = match self.kafka.next() {
            Ok(Some(item)) => item,
            Ok(None) => return Ok(None),
            Err(e) => return Err(format!("Failed to receive message from NATS: {e}").into()),
        };

        let mut builder = SubscriptionOutput::builder();

        match self.key_filter {
            Some(ref filter) => match item.key() {
                Some(key) if filter.is_match(&key) => {}
                _ => return Ok(Some(builder.build())),
            },
            None => return Ok(Some(builder.build())),
        }

        let value: Option<serde_json::Value> = item
            .value()
            .map_err(|e| format!("Error parsing NATS value as JSON: {e}"))?;

        let value = match value {
            Some(value) => value,
            None => return Ok(Some(builder.build())),
        };

        match self.selection {
            Some(ref selection) => {
                let mut jq = self.jq_selection.borrow_mut();

                let filtered = jq
                    .select(selection, value)
                    .map_err(|e| format!("Failed to filter with selection: {e}"))?;

                for payload in filtered {
                    match payload {
                        Ok(payload) => builder.push(payload)?,
                        Err(error) => builder.push_error(format!("Error parsing result value: {error}")),
                    }
                }
            }
            None => {
                builder.push(value)?;
            }
        };

        Ok(Some(builder.build()))
    }
}
