use std::collections::VecDeque;

use grafbase_database_definition::TableWalker;
use grafbase_sdk::{SdkError, host_io::postgres::types::DatabaseValue};
use serde_json::Value;
use sql_ast::ast::{Column, Comparable, ConditionTree, Expression};

use crate::context::Context;

#[derive(Clone)]
pub struct LookupFilterIterator<'a> {
    context: &'a Context<'a>,
    table: TableWalker<'a>,
    filter: VecDeque<(String, Vec<Value>)>,
}

impl<'a> LookupFilterIterator<'a> {
    pub fn new(
        context: &'a Context<'a>,
        table: TableWalker<'a>,
        filter: impl IntoIterator<Item = (String, Vec<Value>)>,
    ) -> Self {
        Self {
            context,
            table,
            filter: filter.into_iter().collect(),
        }
    }
}

impl<'a> Iterator for LookupFilterIterator<'a> {
    type Item = Result<ConditionTree<'a>, SdkError>;

    fn next(&mut self) -> Option<Self::Item> {
        let (field, value) = self.filter.pop_front()?;

        let column = self
            .context
            .database_definition
            .find_column_for_client_field(&field, self.table.id())
            .expect("column for input field not found");

        let value = DatabaseValue::from_json_input(Value::Array(value), column.database_type(), column.is_array());

        let expression = match value {
            Ok(value) => match column.enum_database_name() {
                Some(name) => Expression::enum_value(value, name),
                None => Expression::value(value),
            },
            Err(e) => return Some(Err(e)),
        };

        let column = Column::new(column.database_name()).table(self.table.database_name());

        Some(Ok(column.any_selection(expression).into()))
    }
}
