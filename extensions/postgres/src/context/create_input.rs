use std::collections::VecDeque;

use grafbase_database_definition::{DatabaseType, EnumWalker, TableColumnWalker, TableWalker};
use grafbase_sdk::{
    SdkError,
    host_io::postgres::types::{DatabaseType as _, DatabaseValue},
};

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
    input: serde_json::Map<String, serde_json::Value>,
}

impl<'a> CreateInputIterator<'a> {
    pub fn new(ctx: &'a Context<'a>, table: TableWalker<'a>) -> Result<Self, SdkError> {
        let mut input = VecDeque::new();

        for (ref field_name, value) in ctx.field.arguments::<CreateInputParameters>(ctx.arguments)?.input {
            let Some(column) = ctx
                .database_definition
                .find_column_for_client_field(field_name, table.id())
            else {
                return Err(SdkError::from("field {field_name} not found"));
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

                let value = match column.database_type() {
                    DatabaseType::Enum(r#enum) => {
                        if let Some(value) = value.as_str() {
                            match rename_enum_variant(r#enum, value) {
                                Ok(s) => s.into_bound_value(0),
                                Err(e) => return Some(Err(e)),
                            }
                        } else if let Some(values) = value.to_list() {
                            let mut result = Vec::with_capacity(values.len());

                            for value in values {
                                let Some(value) = value.as_str() else {
                                    return Some(Err(SdkError::from(format!(
                                        "got non-string value for enum field {}",
                                        column.client_name(),
                                    ))));
                                };

                                match rename_enum_variant(r#enum, value) {
                                    Ok(s) => result.push(s),
                                    Err(e) => return Some(Err(e)),
                                }
                            }

                            result.into_bound_value(0)
                        } else {
                            return Some(Err(SdkError::from(format!(
                                "got non-string value for enum field {}",
                                column.client_name(),
                            ))));
                        }
                    }
                    _ => value,
                };

                Some(Ok(CreateInputItem::Column(column, value)))
            }
            IteratorInput::Default(ref mut input) => Some(Ok(CreateInputItem::DefaultValue(input.pop_front()?))),
        }
    }
}

fn rename_enum_variant(r#enum: EnumWalker<'_>, variant: &str) -> Result<String, SdkError> {
    let result = r#enum
        .rename_variant(variant)
        .ok_or_else(|| SdkError::from(format!("invalid enum variant {}", variant)))?
        .to_string();

    Ok(result)
}
