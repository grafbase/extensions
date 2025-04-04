pub mod create_input;
pub mod filter;
pub mod selection_iterator;
pub mod update_input;

use create_input::{CreateInputIterator, CreateInputParameters, CreateManyInputParameters};
use filter::{FilterIterator, MultipleFilterIterator, UniqueFilterIterator};
use grafbase_database_definition::{
    DatabaseDefinition, DatabaseType, EnumWalker, Operation, TableColumnWalker, TableWalker,
};
use grafbase_sdk::{
    SdkError,
    host_io::postgres::{
        self,
        types::{DatabaseType as _, DatabaseValue},
    },
    types::{ArgumentValues, Field},
};
use selection_iterator::SelectionIterator;
use serde_json::Value;
use update_input::UpdateInputIterator;

#[derive(Clone, Copy)]
pub struct Context<'a> {
    pub(super) arguments: ArgumentValues<'a>,
    pub(super) database_definition: &'a DatabaseDefinition,
    pub(super) pool: &'a postgres::Pool,
    pub(super) operation: Operation,
    pub(super) field: Field<'a>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Filter {
    filter: Option<serde_json::Map<String, Value>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Lookup {
    lookup: serde_json::Map<String, Value>,
}

impl<'a> Context<'a> {
    pub fn operation(self) -> Operation {
        self.operation
    }

    pub(crate) fn selection(self, table: TableWalker<'a>) -> Result<SelectionIterator<'a>, SdkError> {
        SelectionIterator::new(self, table, self.field, self.field.selection_set())
    }

    pub fn collection_selection(self, table: TableWalker<'a>) -> Result<SelectionIterator<'a>, SdkError> {
        let field = self
            .field
            .selection_set()
            .fields()
            .find(|f| {
                self.database_definition
                    .get_name_for_field_definition(f.definition_id())
                    == Some("edges")
            })
            .ok_or_else(|| SdkError::from("edges field not defined in selection"))?
            .selection_set()
            .fields()
            .find(|f| {
                self.database_definition
                    .get_name_for_field_definition(f.definition_id())
                    == Some("node")
            })
            .ok_or_else(|| SdkError::from("node field not defined in edges selection"))?;

        let selection = field.selection_set();

        SelectionIterator::new(self, table, field, selection)
    }

    pub(crate) fn create_input(&'a self, table: TableWalker<'a>) -> Result<CreateInputIterator<'a>, SdkError> {
        let args = self.field.arguments::<CreateInputParameters>(self.arguments)?;
        let iterator = CreateInputIterator::new(self, table, args.input)?;
        Ok(iterator)
    }

    pub(crate) fn create_many_input(
        &'a self,
        table: TableWalker<'a>,
    ) -> Result<Vec<CreateInputIterator<'a>>, SdkError> {
        let args = self.field.arguments::<CreateManyInputParameters>(self.arguments)?;
        let mut result = Vec::with_capacity(args.input.len());

        for args in args.input {
            result.push(CreateInputIterator::new(self, table, args)?);
        }

        Ok(result)
    }

    pub(crate) fn update_input(&'a self, table: TableWalker<'a>) -> Result<UpdateInputIterator<'a>, SdkError> {
        let iterator = UpdateInputIterator::new(self, table)?;
        Ok(iterator)
    }

    pub(crate) fn unique_filter(self, table: TableWalker<'a>) -> Result<FilterIterator<'a>, SdkError> {
        let filter = self.field.arguments::<Lookup>(self.arguments)?;
        let iterator = UniqueFilterIterator::new(self, table, filter.lookup);

        Ok(FilterIterator::Unique(iterator))
    }

    pub fn mutation_is_returning(self) -> bool {
        self.field.selection_set().fields().any(|f| {
            self.database_definition
                .get_name_for_field_definition(f.definition_id())
                == Some("returning")
        })
    }

    pub fn returning_selection(self, table: TableWalker<'a>) -> Result<Option<SelectionIterator<'a>>, SdkError> {
        let Some(returning) = self.field.selection_set().fields().find(|f| {
            self.database_definition
                .get_name_for_field_definition(f.definition_id())
                == Some("returning")
        }) else {
            return Ok(None);
        };

        let iterator = SelectionIterator::new(self, table, returning, returning.selection_set())?;

        Ok(Some(iterator))
    }

    /// A complex `user(filter: { id: { eq: 1 } })` filter.
    pub fn filter(&'a self, table: TableWalker<'a>) -> Result<FilterIterator<'a>, SdkError> {
        let filter_map = self
            .field
            .arguments::<Filter>(self.arguments)?
            .filter
            .unwrap_or_default();

        let iterator = MultipleFilterIterator::new(self, table, filter_map);

        Ok(FilterIterator::Multiple(iterator))
    }
}

fn rename_enum_variants(column: TableColumnWalker<'_>, value: DatabaseValue) -> Result<DatabaseValue, SdkError> {
    let value = match column.database_type() {
        DatabaseType::Scalar(_) => value,
        DatabaseType::Enum(r#enum) => {
            if let Some(variant) = value.as_str() {
                match rename_enum_variant(r#enum, variant) {
                    Ok(new_variant) => new_variant.into_bound_value(0),
                    Err(err) => return Err(err),
                }
            } else if let Some(values) = value.to_list() {
                let mut new_values = Vec::with_capacity(values.len());

                for value in values {
                    let Some(variant) = value.as_str() else {
                        return Err(SdkError::from(format!(
                            "got non-string value for enum field {}",
                            column.client_name(),
                        )));
                    };

                    match rename_enum_variant(r#enum, variant) {
                        Ok(new_variant) => new_values.push(new_variant),
                        Err(err) => return Err(err),
                    }
                }

                new_values.into_bound_value(0)
            } else {
                return Err(SdkError::from(format!(
                    "got non-string value for enum field {}",
                    column.client_name(),
                )));
            }
        }
    };

    Ok(value)
}

fn rename_enum_variant(r#enum: EnumWalker<'_>, variant: &str) -> Result<String, SdkError> {
    let result = r#enum
        .rename_variant(variant)
        .ok_or_else(|| SdkError::from(format!("invalid enum variant {}", variant)))?
        .to_string();

    Ok(result)
}
