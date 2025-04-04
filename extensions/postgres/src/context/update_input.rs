use grafbase_database_definition::{TableColumnWalker, TableWalker};
use grafbase_sdk::{SdkError, host_io::postgres::types::DatabaseValue};
use indexmap::IndexMap;
use serde_json::Value;
use sql_ast::ast::{Column, Expression, SqlOp};
use std::collections::VecDeque;

use super::Context;

pub struct UpdateInputItem<'a> {
    pub column: TableColumnWalker<'a>,
    pub expression: Expression<'a>,
}

pub struct UpdateInputIterator<'a> {
    input: VecDeque<(TableColumnWalker<'a>, UpdateOperation)>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum UpdateOperation {
    /// Set the column to a specific value
    Set { set: Value },

    /// Increment the column by the specified value
    Increment { increment: Value },

    /// Decrement the column by the specified value
    Decrement { decrement: Value },

    /// Delete a key from a JSON object
    DeleteKey { delete_key: Value },

    /// Multiply the column by the specified value
    Multiply { multiply: Value },

    /// Divide the column by the specified value
    Divide { divide: Value },

    /// Append a value to an array or concatenate with a string
    Append { append: Value },

    /// Prepend a value to an array or concatenate with a string
    Prepend { prepend: Value },

    /// Delete a key path from a JSON object
    DeleteAtPath { delete_at_path: Value },
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateInputParameters {
    pub input: IndexMap<String, UpdateOperation>,
}

impl<'a> UpdateInputIterator<'a> {
    pub fn new(ctx: &'a Context<'a>, table: TableWalker<'a>) -> Result<Self, SdkError> {
        let mut input = VecDeque::new();

        for (ref field_name, op) in ctx.field.arguments::<UpdateInputParameters>(ctx.arguments)?.input {
            let Some(column) = ctx
                .database_definition
                .find_column_for_client_field(field_name, table.id())
            else {
                return Err(SdkError::from("field {field_name} not found"));
            };

            input.push_back((column, op));
        }

        Ok(Self {
            input: input.into_iter().collect(),
        })
    }
}

impl<'a> Iterator for UpdateInputIterator<'a> {
    type Item = Result<UpdateInputItem<'a>, SdkError>;

    fn next(&mut self) -> Option<Self::Item> {
        use UpdateOperation::*;

        let (column, value) = self.input.pop_front()?;
        let sql_column = Column::from(column.database_name());

        let as_value = |value: Value| {
            let value = DatabaseValue::from_json_input(value, column.database_type(), column.is_array())?;
            Result::<_, SdkError>::Ok(value)
        };

        let expression = match value {
            Set { set } => match as_value(set) {
                Ok(value) => Expression::from(value),
                Err(err) => return Some(Err(err)),
            },
            Increment { increment } => match as_value(increment) {
                Ok(value) => Expression::from(sql_column) + Expression::from(value),
                Err(err) => return Some(Err(err)),
            },
            Decrement { decrement } => match as_value(decrement) {
                Ok(value) => Expression::from(sql_column) - Expression::from(value),
                Err(err) => return Some(Err(err)),
            },
            DeleteKey { delete_key } => match as_value(delete_key) {
                Ok(value) => Expression::from(sql_column) - Expression::from(value),
                Err(err) => return Some(Err(err)),
            },
            UpdateOperation::Multiply { multiply } => match as_value(multiply) {
                Ok(value) => Expression::from(sql_column) * Expression::from(value),
                Err(err) => return Some(Err(err)),
            },
            UpdateOperation::Divide { divide } => match as_value(divide) {
                Ok(value) => Expression::from(sql_column) / Expression::from(value),
                Err(err) => return Some(Err(err)),
            },
            UpdateOperation::Append { append } => match as_value(append) {
                Ok(value) => {
                    let op = SqlOp::Append(Expression::from(sql_column), Expression::from(value));
                    Expression::from(op)
                }
                Err(err) => return Some(Err(err)),
            },
            UpdateOperation::Prepend { prepend } => match as_value(prepend) {
                Ok(value) => {
                    let op = SqlOp::Append(Expression::from(value), Expression::from(sql_column));
                    Expression::from(op)
                }
                Err(err) => return Some(Err(err)),
            },
            UpdateOperation::DeleteAtPath { delete_at_path } => match as_value(delete_at_path) {
                Ok(value) => {
                    let op = SqlOp::JsonDeleteAtPath(Expression::from(sql_column), Expression::from(value));

                    Expression::from(op)
                }
                Err(err) => return Some(Err(err)),
            },
        };

        Some(Ok(UpdateInputItem { column, expression }))
    }
}
