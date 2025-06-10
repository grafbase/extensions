use std::{cell::RefCell, rc::Rc};

use grafbase_sdk::{
    Subscription,
    host_io::kafka,
    jq_selection::JqSelection,
    types::{Error, Response, SubscriptionItem},
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
    fn next(&mut self) -> Result<Option<SubscriptionItem>, Error> {
        let item = match self.kafka.next() {
            Ok(Some(item)) => item,
            Ok(None) => {
                return Ok(None);
            }
            Err(e) => return Err(format!("Failed to receive message from NATS: {e}").into()),
        };

        if let Some(ref filter) = self.key_filter {
            match item.key() {
                Some(key) if filter.is_match(&key) => {}
                _ => return Ok(Some(Vec::new().into())),
            }
        }

        match &self.selection {
            Some(selection) => {
                let value: Option<serde_json::Value> = item
                    .value()
                    .map_err(|e| format!("Error parsing NATS value as JSON: {e}"))?;

                let value = match value {
                    Some(value) => value,
                    None => return Ok(Some(Response::null().into())),
                };

                let mut jq = self.jq_selection.borrow_mut();

                let items = jq
                    .select(selection, value)
                    .map_err(|e| format!("Failed to filter with selection: {e}"))?
                    .map(|result| match result {
                        Ok(value) => Response::data(value),
                        Err(err) => Response::error(err),
                    })
                    .collect::<Vec<_>>();

                Ok(Some(items.into()))
            }
            None => Ok(Some(
                item.into_raw_value().map(Response::json).unwrap_or_default().into(),
            )),
        }
    }
}
