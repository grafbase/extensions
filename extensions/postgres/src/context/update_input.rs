use grafbase_database_definition::{TableColumnWalker, TableWalker};
use grafbase_sdk::{
    SdkError,
    host_io::postgres::types::{DatabaseType as _, DatabaseValue},
};
use indexmap::IndexMap;
use serde_json::Value;
use sql_ast::ast::{Column, Expression, SqlOp};
use std::{collections::VecDeque, fmt::Debug};

use super::Context;

pub struct UpdateInputItem<'a> {
    pub column: TableColumnWalker<'a>,
    pub expression: Expression<'a>,
}

impl Debug for UpdateInputItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateInputItem")
            .field("column", &self.column.client_name())
            .field("expression", &self.expression)
            .finish()
    }
}

pub struct UpdateInputIterator<'a> {
    input: VecDeque<(TableColumnWalker<'a>, UpdateOperation)>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum UpdateOperation {
    /// Set the column to a specific value
    Set { set: Value },

    /// Increment the column by the specified value
    Increment { increment: Value },

    /// Decrement the column by the specified value
    Decrement { decrement: Value },

    /// Delete a key from a JSON object
    DeleteKey {
        #[serde(rename = "deleteKey")]
        delete_key: String,
    },

    /// Multiply the column by the specified value
    Multiply { multiply: Value },

    /// Divide the column by the specified value
    Divide { divide: Value },

    /// Append a value to an array or concatenate with a string
    Append { append: Value },

    /// Prepend a value to an array or concatenate with a string
    Prepend { prepend: Value },

    /// Delete a key path from a JSON object
    DeleteAtPath {
        #[serde(rename = "deleteAtPath")]
        delete_at_path: Vec<String>,
    },
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateInputParameters {
    pub input: IndexMap<String, UpdateOperation>,
}

impl<'a> UpdateInputIterator<'a> {
    pub fn new(ctx: &'a Context<'a>, table: TableWalker<'a>) -> Result<Self, SdkError> {
        let mut input = VecDeque::new();

        for (ref field_name, op) in ctx.field.arguments::<UpdateInputParameters>(ctx.variables)?.input {
            let Some(column) = ctx
                .database_definition
                .find_column_for_client_field(field_name, table.id())
            else {
                return Err(SdkError::from(format!("field {field_name} not found")));
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

            super::rename_enum_variants(column, value)
        };

        let value_expression = |value: DatabaseValue| match column.enum_database_name() {
            Some(enum_type) => Expression::enum_value(value, enum_type),
            None => Expression::value(value),
        };

        let expression = match value {
            Set { set } => match as_value(set) {
                Ok(value) => value_expression(value),
                Err(err) => return Some(Err(err)),
            },
            Increment { increment } => match as_value(increment) {
                Ok(value) => Expression::from(sql_column) + value_expression(value),
                Err(err) => return Some(Err(err)),
            },
            Decrement { decrement } => match as_value(decrement) {
                Ok(value) => Expression::from(sql_column) - value_expression(value),
                Err(err) => return Some(Err(err)),
            },
            DeleteKey { delete_key } => {
                Expression::from(sql_column) - Expression::value(delete_key.into_bound_value(0))
            }
            UpdateOperation::Multiply { multiply } => match as_value(multiply) {
                Ok(value) => Expression::from(sql_column) * value_expression(value),
                Err(err) => return Some(Err(err)),
            },
            UpdateOperation::Divide { divide } => match as_value(divide) {
                Ok(value) => Expression::from(sql_column) / value_expression(value),
                Err(err) => return Some(Err(err)),
            },
            UpdateOperation::Append { append } => match as_value(append) {
                Ok(value) => {
                    let op = SqlOp::Append(Expression::from(sql_column), value_expression(value));
                    Expression::from(op)
                }
                Err(err) => return Some(Err(err)),
            },
            UpdateOperation::Prepend { prepend } => match as_value(prepend) {
                Ok(value) => {
                    let op = SqlOp::Append(value_expression(value), Expression::from(sql_column));
                    Expression::from(op)
                }
                Err(err) => return Some(Err(err)),
            },
            UpdateOperation::DeleteAtPath { delete_at_path } => {
                let op = SqlOp::JsonDeleteAtPath(
                    Expression::from(sql_column),
                    Expression::value(delete_at_path.into_bound_value(0)),
                );

                Expression::from(op)
            }
        };

        Some(Ok(UpdateInputItem { column, expression }))
    }
}
