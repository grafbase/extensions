use std::collections::VecDeque;

use grafbase_database_definition::{TableColumnWalker, TableWalker};
use serde_json::Value;

use super::Context;

#[derive(Clone)]
pub enum OrderKind<'a> {
    Single(TableColumnWalker<'a>, Value),
    Composite(Vec<(TableColumnWalker<'a>, Value)>),
}

#[derive(Clone)]
pub struct LookupOrderIterator<'a> {
    order: VecDeque<OrderKind<'a>>,
}

impl<'a> LookupOrderIterator<'a> {
    pub fn new(
        context: &'a Context<'a>,
        table: TableWalker<'a>,
        filter: impl IntoIterator<Item = (String, Vec<Value>)>,
    ) -> Self {
        let mut order = VecDeque::new();

        for (field, values) in filter {
            let column = context
                .database_definition
                .find_column_for_client_field(&field, table.id());

            for value in values {
                match value {
                    Value::Object(map) => {
                        let mut composite = Vec::new();

                        for (field, value) in map {
                            let column = context
                                .database_definition
                                .find_column_for_client_field(&field, table.id())
                                .unwrap();

                            composite.push((column, value));
                        }

                        order.push_back(OrderKind::Composite(composite));
                    }
                    value => {
                        order.push_back(OrderKind::Single(column.unwrap(), value));
                    }
                }
            }
        }

        Self { order }
    }

    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
}

impl<'a> Iterator for LookupOrderIterator<'a> {
    type Item = OrderKind<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.order.pop_front()
    }
}
