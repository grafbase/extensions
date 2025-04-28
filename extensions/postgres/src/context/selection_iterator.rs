pub mod collection_args;

use collection_args::{CollectionArgs, CollectionParameters};
use grafbase_database_definition::{DatabaseType, EnumWalker, RelationWalker, TableColumnWalker, TableWalker};
use grafbase_sdk::{
    SdkError,
    types::{Field, SelectionSet},
};
use sql_ast::ast::{self, Aliasable, Case, Column, Comparable, Expression, Select, json_agg, raw_str};
use std::{borrow::Cow, collections::HashMap};

use super::Context;

#[derive(Clone)]
pub struct SelectColumn<'a>(TableColumnWalker<'a>);

impl<'a> SelectColumn<'a> {
    pub fn into_expression(self, table_name: Option<Cow<'a, str>>) -> (TableColumnWalker<'a>, Expression<'a>) {
        let table_name = match table_name {
            Some(name) => name,
            None => Cow::Borrowed(self.0.table().database_name()),
        };

        let sql_col = Column::new(self.0.database_name()).table(table_name);

        let r#enum = match self.0.database_type() {
            DatabaseType::Scalar(scalar_type) => {
                let expr = match scalar_type.from_db_to_client_cast() {
                    Some(cast) => Expression::from(ast::cast(sql_col, cast)),
                    None => Expression::from(sql_col),
                };

                return (self.0, expr);
            }
            DatabaseType::Enum(walker) => walker,
        };

        let col = ast::cast(sql_col.clone(), "text");

        let builder = r#enum.variants().fold(Case::builder(), |builder, variant| {
            let when = Expression::from(col.clone()).equals(raw_str(variant.database_name()));
            let then = raw_str(variant.client_name());

            builder.when(when, then)
        });

        let expr = builder.r#else(Expression::from(col)).into();

        (self.0, expr)
    }
}

#[derive(Clone)]
pub struct Unnest<'a>(TableColumnWalker<'a>, EnumWalker<'a>);

impl<'a> Unnest<'a> {
    pub fn into_select(self, table_name: Option<Cow<'a, str>>) -> (TableColumnWalker<'a>, Select<'a>) {
        let unnest_col = Column::new(format!("unnest_{}", self.0.database_name()));
        let unnest_col = ast::cast(unnest_col, "text");

        let builder = self.1.variants().fold(Case::builder(), |acc, variant| {
            let when = Expression::from(unnest_col.clone()).equals(raw_str(variant.database_name()));
            let then = raw_str(variant.client_name());

            acc.when(when, then)
        });

        let case = builder.r#else(Expression::from(unnest_col));
        let aggregate = json_agg(Expression::from(case), None, false).alias("json_array");

        let mut column = Column::new(self.0.database_name());

        if let Some(table_name) = table_name {
            column = column.table(table_name);
        }

        let expr = Expression::from(ast::unnest(column, false)).alias(format!("unnest_{}", self.0.database_name()));

        let mut select = Select::from_table(expr);
        select.value(aggregate);

        (self.0, select)
    }
}

#[derive(Clone)]
pub enum TableSelection<'a> {
    /// Selects a single column.
    Column(SelectColumn<'a>),
    /// Returns a selection which transforms an array of enum values into an array of
    /// GraphQL enum values, renamed.
    ColumnUnnest(Unnest<'a>),
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

            return extra.map(|column| Ok(TableSelection::Column(SelectColumn(*column))));
        };

        self.index += 1;

        // Selecting a column.
        if let Some(column) = self
            .ctx
            .database_definition
            .column_for_field_definition(selection_field.definition_id())
        {
            match column.database_type() {
                DatabaseType::Enum(r#enum) if column.is_array() => {
                    return Some(Ok(TableSelection::ColumnUnnest(Unnest(column, r#enum))));
                }
                _ => {
                    return Some(Ok(TableSelection::Column(SelectColumn(column))));
                }
            }
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
