use std::{cell::RefCell, rc::Rc};

use grafbase_sdk::{
    host_io::nats::{self, NatsSubscription},
    jq_selection::JqSelection,
    types::{Error, Response, SubscriptionItem},
    Subscription,
};

#[derive(serde::Serialize)]
pub struct DeduplicationKey<'a> {
    pub provider: &'a str,
    pub subject: &'a str,
    pub selection: Option<&'a str>,
}

pub struct FilteredSubscription {
    nats: nats::NatsSubscription,
    jq_selection: Rc<RefCell<JqSelection>>,
    selection: Option<String>,
}

impl FilteredSubscription {
    pub fn new(nats: NatsSubscription, jq_selection: Rc<RefCell<JqSelection>>, selection: Option<String>) -> Self {
        Self {
            nats,
            jq_selection,
            selection,
        }
    }
}

impl Subscription for FilteredSubscription {
    fn next(&mut self) -> Result<Option<SubscriptionItem>, Error> {
        let item = match self.nats.next() {
            Ok(Some(item)) => item,
            Ok(None) => return Ok(None),
            Err(e) => return Err(format!("Failed to receive message from NATS: {e}").into()),
        };

        let payload: serde_json::Value = item
            .payload()
            .map_err(|e| format!("Error parsing NATS value as JSON: {e}"))?;

        match self.selection {
            Some(ref selection) => {
                let mut jq = self.jq_selection.borrow_mut();

                let items = jq
                    .select(selection, payload)
                    .map_err(|e| format!("Failed to filter with selection: {e}"))?
                    .map(|result| match result {
                        Ok(value) => Response::data(value),
                        Err(err) => Response::error(err),
                    })
                    .collect::<Vec<_>>();
                Ok(Some(SubscriptionItem::Multiple(items)))
            }
            None => Ok(Some(SubscriptionItem::Single(Response::data(payload)))),
        }
    }
}
