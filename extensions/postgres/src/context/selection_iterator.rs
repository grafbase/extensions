pub mod collection_args;

use collection_args::{CollectionArgs, CollectionParameters};
use grafbase_database_definition::{RelationWalker, TableColumnWalker, TableWalker};
use grafbase_sdk::{
    SdkError,
    types::{Field, SelectionSet},
};
use std::collections::HashMap;

use super::Context;

pub enum TableSelection<'a> {
    /// Selects a single column.
    Column(TableColumnWalker<'a>),
    /// Joins a unique row with a nested selection.
    JoinUnique(RelationWalker<'a>, SelectionIterator<'a>),
    /// Joins a collection of rows with a nested selection.
    JoinMany(RelationWalker<'a>, SelectionIterator<'a>, CollectionArgs),
}

/// An iterator over a GraphQL selection. Returns either a column or a
/// join, which should be handled accordingly when generating an SQL query.
#[derive(Clone)]
pub struct SelectionIterator<'a> {
    ctx: Context<'a>,
    selection: SelectionSet<'a>,
    extra_columns: Vec<TableColumnWalker<'a>>,
    index: usize,
    extra_column_index: usize,
}

impl<'a> SelectionIterator<'a> {
    pub fn new(
        ctx: Context<'a>,
        table: TableWalker<'a>,
        field: Field<'a>,
        selection: SelectionSet<'a>,
    ) -> Result<Self, SdkError> {
        let mut extra_columns = Vec::new();

        let selection_columns: HashMap<_, _> = selection
            .fields()
            .flat_map(|f| ctx.database_definition.column_for_field_definition(f.definition_id()))
            .map(|c| (c.client_name(), c))
            .collect();

        if let Ok(params) = field.arguments::<CollectionParameters>(ctx.arguments) {
            for order_input in &params.order_by {
                for field_name in order_input.field.keys() {
                    if selection_columns.contains_key(field_name.as_str()) {
                        continue;
                    }

                    let column = ctx
                        .database_definition
                        .find_column_for_client_field(field_name, table.id())
                        .ok_or_else(|| {
                            SdkError::from(format!(
                                "ordering type {} with non-existing field {}",
                                table.client_name(),
                                field_name
                            ))
                        })?;

                    extra_columns.push(column);
                }
            }
        };

        for column in table.implicit_ordering_key().unwrap().columns() {
            if selection_columns.contains_key(column.table_column().client_name()) {
                continue;
            }

            if extra_columns.contains(&column.table_column()) {
                continue;
            }

            extra_columns.push(column.table_column());
        }

        Ok(Self {
            ctx,
            selection,
            extra_columns,
            index: 0,
            extra_column_index: 0,
        })
    }
}

impl<'a> Iterator for SelectionIterator<'a> {
    type Item = Result<TableSelection<'a>, SdkError>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(selection_field) = self.selection.fields().nth(self.index) else {
            let extra = self.extra_columns.get(self.extra_column_index);
            self.extra_column_index += 1;

            return extra.map(|column| Ok(TableSelection::Column(*column)));
        };

        self.index += 1;

        // Selecting a column.
        if let Some(column) = self
            .ctx
            .database_definition
            .column_for_field_definition(selection_field.definition_id())
        {
            return Some(Ok(TableSelection::Column(column)));
        }

        // Joining a table with the current one, selecting from the joined table.
        let relation = match self
            .ctx
            .database_definition
            .get_relation_id_for_client_field_id(selection_field.definition_id())
            .map(|id| self.ctx.database_definition.walk(id))
        {
            Some(relation) => relation,
            None => {
                return self.next();
            }
        };

        if relation.is_other_side_one() {
            // The other side has a unique constraint, so our join must return at most one row.
            let selection_set = selection_field.selection_set();

            let iterator = match Self::new(self.ctx, relation.referenced_table(), selection_field, selection_set) {
                Ok(iterator) => iterator,
                Err(err) => return Some(Err(err)),
            };

            Some(Ok(TableSelection::JoinUnique(relation, iterator)))
        } else {
            let params = selection_field
                .arguments::<CollectionParameters>(self.ctx.arguments)
                .ok()
                .unwrap_or_default();

            // The other side has not a unique constraint that matches with the foreign key,
            // meaning the resulting set is a collection.

            // `userCollection { edges { node { field } } }`, the selection part.
            //
            let selection_field = selection_field
                .selection_set()
                .fields()
                .find(|f| {
                    self.ctx
                        .database_definition
                        .get_name_for_field_definition(f.definition_id())
                        == Some("edges")
                })
                .unwrap()
                .selection_set()
                .fields()
                .find(|f| {
                    self.ctx
                        .database_definition
                        .get_name_for_field_definition(f.definition_id())
                        == Some("node")
                })
                .unwrap();

            let selection_set = selection_field.selection_set();

            let iterator = match Self::new(self.ctx, relation.referenced_table(), selection_field, selection_set) {
                Ok(iterator) => iterator,
                Err(error) => return Some(Err(error)),
            };

            // By defining this, we mark the next select to return a collecton.
            let args = CollectionArgs::new(self.ctx.database_definition, relation.referenced_table(), params);

            match args {
                Ok(args) => Some(Ok(TableSelection::JoinMany(relation, iterator, args))),
                Err(error) => Some(Err(error)),
            }
        }
    }
}
