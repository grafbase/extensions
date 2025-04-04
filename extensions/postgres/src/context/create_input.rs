use std::collections::VecDeque;

use grafbase_database_definition::{TableColumnWalker, TableWalker};
use grafbase_sdk::{SdkError, host_io::postgres::types::DatabaseValue};

use super::Context;

pub enum CreateInputItem<'a> {
    /// Inserts a single column value.
    Column(TableColumnWalker<'a>, DatabaseValue),
    DefaultValue(TableColumnWalker<'a>),
}

enum IteratorInput<'a> {
    FromUser(VecDeque<(TableColumnWalker<'a>, DatabaseValue)>),
    Default(VecDeque<TableColumnWalker<'a>>),
}

pub struct CreateInputIterator<'a> {
    input: IteratorInput<'a>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateInputParameters {
    pub input: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateManyInputParameters {
    pub input: Vec<serde_json::Map<String, serde_json::Value>>,
}

impl<'a> CreateInputIterator<'a> {
    pub fn new(
        ctx: &'a Context<'a>,
        table: TableWalker<'a>,
        args: serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, SdkError> {
        let mut input = VecDeque::new();

        for (ref field_name, value) in args {
            let Some(column) = ctx
                .database_definition
                .find_column_for_client_field(field_name, table.id())
            else {
                return Err(SdkError::from(format!("field {field_name} not found")));
            };

            let value = DatabaseValue::from_json_input(value, column.database_type(), column.is_array())?;

            input.push_back((column, value));
        }

        let input = match input {
            input if input.is_empty() => {
                let mut input = VecDeque::new();

                for column in table.columns() {
                    input.push_back(column);
                }

                IteratorInput::Default(input)
            }
            input => IteratorInput::FromUser(input),
        };

        Ok(Self { input })
    }
}

impl<'a> Iterator for CreateInputIterator<'a> {
    type Item = Result<CreateInputItem<'a>, SdkError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input {
            IteratorInput::FromUser(ref mut input) => {
                let (column, value) = input.pop_front()?;

                let value = match super::rename_enum_variants(column, value) {
                    Ok(value) => value,
                    Err(err) => return Some(Err(err)),
                };

                Some(Ok(CreateInputItem::Column(column, value)))
            }
            IteratorInput::Default(ref mut input) => Some(Ok(CreateInputItem::DefaultValue(input.pop_front()?))),
        }
    }
}
