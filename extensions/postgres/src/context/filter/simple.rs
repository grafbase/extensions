use crate::context::Context;
use grafbase_database_definition::{TableColumnId, TableWalker};
use grafbase_sdk::{SdkError, host_io::postgres::types::DatabaseValue};
use indexmap::IndexSet;
use serde_json::Value;
use sql_ast::ast::{Column, Comparable, Compare, Expression};
use std::{collections::VecDeque, iter::Iterator};

/// An iterator for a "simple" filter, e.g. a filter that's defined
/// as `by` argument from the client, and has at most one unique equality
/// check.
#[derive(Clone)]
pub struct UniqueFilterIterator<'a> {
    context: Context<'a>,
    table: TableWalker<'a>,
    filter: VecDeque<(String, Value)>,
    nested: Option<Box<UniqueFilterIterator<'a>>>,
    constrained_columns: IndexSet<TableColumnId>,
}

impl<'a> UniqueFilterIterator<'a> {
    pub fn new(
        context: Context<'a>,
        table: TableWalker<'a>,
        filter: impl IntoIterator<Item = (String, Value)>,
    ) -> Self {
        Self {
            context,
            table,
            filter: VecDeque::from_iter(filter),
            nested: None,
            constrained_columns: IndexSet::new(),
        }
    }

    fn push_constrained_column(&mut self, column_id: TableColumnId) {
        self.constrained_columns.insert(column_id);
    }
}

impl<'a> Iterator for UniqueFilterIterator<'a> {
    type Item = Result<Compare<'a>, SdkError>;

    fn next(&mut self) -> Option<Self::Item> {
        // We are having a nested input type, which we iterate over.
        if let Some(item) = self.nested.as_mut().and_then(Iterator::next) {
            return Some(item);
        }

        let Some((field, value)) = self.filter.pop_front() else {
            // solves the issue where user emits a value for a nullable composite unique.
            return self
                .constrained_columns
                .pop()
                .map(|column_id| {
                    let column = self.context.database_definition.walk(column_id);
                    (self.table.database_name(), column.database_name()).is_null()
                })
                .map(Ok);
        };

        // If selecting an object, we don't care about the name of the object, but selecting the
        // fields defined in the input.
        //
        // E.g. in `user(by: { nameEmail: { name: "foo", email: "bar" }})`, we do not care about `nameEmail`,
        // but the nested values `name` and `email` are used in the query filters.
        if let Value::Object(map) = value {
            let mut nested = UniqueFilterIterator::new(self.context, self.table, map);

            let constraint = self
                .context
                .database_definition
                .find_unique_constraint_for_client_field(&field, self.table.id())
                .expect("constraint for input field not found");

            for column in constraint.columns() {
                nested.push_constrained_column(column.table_column().id());
            }

            let item = nested.next();
            self.nested = Some(Box::new(nested));

            return item;
        };

        let column = self
            .context
            .database_definition
            .find_column_for_client_field(&field, self.table.id())
            .expect("column for input field not found");

        self.constrained_columns.shift_remove(&column.id());

        match value {
            Value::Null => Some(Ok(Column::new(column.database_name())
                .table(self.table.database_name())
                .is_null())),
            _ => {
                let value = DatabaseValue::from_json_input(value, column.database_type(), column.is_array());

                let expression = match value {
                    Ok(value) => match column.enum_database_name() {
                        Some(name) => Expression::enum_value(value, name),
                        None => Expression::value(value),
                    },
                    Err(e) => return Some(Err(e)),
                };

                let column = Column::new(column.database_name()).table(self.table.database_name());

                Some(Ok(column.equals(expression)))
            }
        }
    }
}
