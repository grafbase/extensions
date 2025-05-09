pub mod create_input;
pub mod filter;
pub mod order;
pub mod selection_iterator;
pub mod update_input;

use std::collections::HashMap;

use create_input::{CreateInputIterator, CreateInputParameters, CreateManyInputParameters};
use filter::{FilterIterator, LookupFilterIterator, MultipleFilterIterator, UniqueFilterIterator};
use grafbase_database_definition::{
    DatabaseDefinition, DatabaseType, EnumWalker, Operation, TableColumnWalker, TableWalker,
};
use grafbase_sdk::{
    SdkError,
    host_io::postgres::{
        self,
        types::{DatabaseType as _, DatabaseValue},
    },
    types::{ArgumentValues, Field, SelectionSet},
};
use order::LookupOrderIterator;
use selection_iterator::SelectionIterator;
use serde_json::{Map, Value};
use update_input::UpdateInputIterator;

#[derive(Clone, Copy)]
pub struct Context<'a> {
    pub(super) arguments: ArgumentValues<'a>,
    pub(super) database_definition: &'a DatabaseDefinition,
    pub(super) pool: &'a postgres::Pool,
    pub(super) operation: Operation,
    pub(super) field: Field<'a>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
enum InputManyFilter {
    Filter {
        filter: Map<String, serde_json::Value>,
    },
    Lookup {
        lookup: HashMap<String, Vec<serde_json::Value>>,
    },
}
#[derive(Debug, Clone, serde::Deserialize)]
struct FilterUnique {
    lookup: serde_json::Map<String, Value>,
}

#[derive(Clone, Copy)]
pub struct PageInfo {
    has_next_page: bool,
    has_previous_page: bool,
    start_cursor: bool,
    end_cursor: bool,
}

impl PageInfo {
    pub fn new(database_definition: &DatabaseDefinition, inner: SelectionSet<'_>) -> Self {
        let has_next_page = inner
            .fields()
            .any(|f| database_definition.get_name_for_field_definition(f.definition_id()) == Some("hasNextPage"));

        let has_previous_page = inner
            .fields()
            .any(|f| database_definition.get_name_for_field_definition(f.definition_id()) == Some("hasPreviousPage"));

        let start_cursor = inner
            .fields()
            .any(|f| database_definition.get_name_for_field_definition(f.definition_id()) == Some("startCursor"));

        let end_cursor = inner
            .fields()
            .any(|f| database_definition.get_name_for_field_definition(f.definition_id()) == Some("endCursor"));

        Self {
            has_next_page,
            has_previous_page,
            start_cursor,
            end_cursor,
        }
    }

    pub fn selects_has_next_page(&self) -> bool {
        self.has_next_page
    }

    pub fn selects_has_previous_page(&self) -> bool {
        self.has_previous_page
    }

    pub fn selects_start_cursor(&self) -> bool {
        self.start_cursor
    }

    pub fn selects_end_cursor(&self) -> bool {
        self.end_cursor
    }

    pub fn needs_cursor(&self) -> bool {
        self.selects_start_cursor() || self.selects_end_cursor()
    }
}

impl<'a> Context<'a> {
    pub fn operation(self) -> Operation {
        self.operation
    }

    /// Creates a `SelectionIterator` for a flat selection based on the current field and context.
    /// For pagination, use `collection_selection`.
    pub(crate) fn selection(self, table: TableWalker<'a>) -> Result<SelectionIterator<'a>, SdkError> {
        SelectionIterator::unique(self, table, self.field, Some(self.field.selection_set()))
    }

    /// Creates a `SelectionIterator` for a collection of items (edges) based on the current field and context.
    /// Use this method for pagination.
    pub fn collection_selection(self, table: TableWalker<'a>) -> Result<SelectionIterator<'a>, SdkError> {
        SelectionIterator::edges(self, table, self.field, Some(self.field.selection_set()))
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
        let filter = self.field.arguments::<FilterUnique>(self.arguments)?;
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

        let iterator = SelectionIterator::unique(self, table, returning, Some(returning.selection_set()))?;

        Ok(Some(iterator))
    }

    /// A complex `user(filter: { id: { eq: 1 } })` filter, or a
    /// lookup filter `user(lookup: { id: [1, 2, 3] })`.
    pub fn filter(&'a self, table: TableWalker<'a>) -> Result<FilterIterator<'a>, SdkError> {
        let filter = self.field.arguments::<InputManyFilter>(self.arguments)?;

        match filter {
            InputManyFilter::Filter { filter } => {
                let iterator = MultipleFilterIterator::new(self, table, filter);
                Ok(FilterIterator::Multiple(iterator))
            }
            InputManyFilter::Lookup { lookup } => {
                let iterator = LookupFilterIterator::new(self, table, lookup);
                Ok(FilterIterator::Lookup(iterator))
            }
        }
    }

    /// Parses the `lookup` argument if present and creates a `LookupOrderIterator`
    /// to preserve the order of results based on the input lookup values.
    /// Returns `Ok(None)` if the `lookup` argument is not present or not the correct variant.
    pub fn lookup_order(&self, table: TableWalker<'a>) -> Result<Option<LookupOrderIterator>, SdkError> {
        let InputManyFilter::Lookup { lookup } = self.field.arguments::<InputManyFilter>(self.arguments)? else {
            return Ok(None);
        };

        Ok(Some(LookupOrderIterator::new(self, table, lookup)))
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
