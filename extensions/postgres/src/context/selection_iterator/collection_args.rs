use std::collections::BTreeMap;

use grafbase_database_definition::{DatabaseDefinition, TableColumnWalker, TableWalker};
use grafbase_sdk::{SdkError, host_io::postgres::types::DatabaseValue};

use sql_ast::ast::{Aliasable, Column, Comparable, ConditionTree, Expression, Order, OrderDefinition};

#[derive(Debug, Clone, Default)]
pub struct CollectionOrdering {
    inner: Vec<((String, String), Option<Order>)>,
    outer: Vec<(String, Option<Order>)>,
}

impl CollectionOrdering {
    pub fn inner(&self) -> impl ExactSizeIterator<Item = OrderDefinition<'static>> + '_ {
        self.inner
            .iter()
            .map(|((table, column), order)| (Column::from((table.clone(), column.clone())).into(), *order))
    }

    pub fn outer(&self) -> impl ExactSizeIterator<Item = OrderDefinition<'static>> + '_ {
        self.outer.iter().map(|(column, order)| {
            let column = Column::from(column.clone());
            (column.into(), *order)
        })
    }
}

/// Argument defining a relay-style GraphQL collection.
#[derive(Debug, Clone)]
pub struct CollectionArgs {
    first: Option<u64>,
    last: Option<u64>,
    order_by: CollectionOrdering,
    extra_columns: Vec<Column<'static>>,
}

#[derive(Default, Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionParameters {
    pub first: Option<u64>,
    pub last: Option<u64>,
    #[serde(default)]
    pub order_by: Vec<OrderParameter>,
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct OrderParameter {
    #[serde(flatten)]
    pub field: BTreeMap<String, OrderDirection>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl CollectionArgs {
    pub(crate) fn new(
        database_definition: &DatabaseDefinition,
        table: TableWalker<'_>,
        mut params: CollectionParameters,
    ) -> Result<Self, SdkError> {
        if let (Some(_), Some(_)) = (params.first, params.last) {
            return Err(SdkError::from("first and last parameters can't be both defined"));
        }

        let constraint = table
            .implicit_ordering_key()
            .expect("tables at this point must have at least one unique constraint");

        for column in constraint.columns() {
            if params
                .order_by
                .iter()
                .any(|v| v.field.contains_key(column.table_column().client_name()))
            {
                continue;
            }

            params.order_by.push({
                let mut map = BTreeMap::new();
                map.insert(column.table_column().client_name().to_string(), OrderDirection::Asc);

                OrderParameter { field: map }
            });
        }

        // ordering the innermost query
        let mut order_by = CollectionOrdering::default();

        // extra columns we have to select (based on ordering)
        let mut extra_columns = Vec::new();

        for mut value in params.order_by {
            let Some((field, direction)) = value.field.pop_first() else {
                continue;
            };

            // For `last` to work, we must reverse the order of the inner query.
            let inner_direction = match direction {
                OrderDirection::Desc if params.last.is_some() => Order::AscNullsFirst,
                OrderDirection::Desc => Order::DescNullsFirst,
                _ if params.last.is_some() => Order::DescNullsFirst,
                _ => Order::AscNullsFirst,
            };

            // and then reverse the order again for the outer query.
            let outer_direction = match inner_direction {
                Order::DescNullsFirst if params.last.is_some() => Order::AscNullsFirst,
                Order::AscNullsFirst if params.last.is_some() => Order::DescNullsFirst,
                _ => inner_direction,
            };

            let column = database_definition
                .find_column_for_client_field(&field, table.id())
                .ok_or_else(|| {
                    SdkError::from(format!(
                        "ordering type {} with field{}, which does not exist",
                        table.client_name(),
                        &field
                    ))
                })?;

            let sql_column = Column::from((table.database_name().to_string(), column.client_name().to_string()));

            // We must name our order columns for them to be visible in the order by statement of the
            // outer queries.
            let alias = format!("{}_{}", table.database_name(), column.database_name());
            extra_columns.push(sql_column.clone().alias(alias.clone()));

            order_by.inner.push((
                (table.database_name().to_string(), column.database_name().to_string()),
                Some(inner_direction),
            ));

            order_by.outer.push((alias, Some(outer_direction)));
        }

        Ok(Self {
            first: params.first,
            last: params.last,
            order_by,
            extra_columns,
        })
    }

    /// Select the first N items. An example GraphQL definition: `userCollection(first: N)`.
    pub(crate) fn first(&self) -> Option<u64> {
        self.first
    }

    /// Select the last N items. An example GraphQL definition: `userCollection(last: N)`.
    pub(crate) fn last(&self) -> Option<u64> {
        self.last
    }

    /// Defines the ordering of the collection. The first item in a tuple is the ordering for the innermost
    /// query, and the second one of all the outer queries. An example GraphQL definition:
    /// `userCollection(orderBy: [{ name: DESC }])`.
    pub(crate) fn order_by(&self) -> &CollectionOrdering {
        &self.order_by
    }

    /// A set of extra columns needing to select in the collecting query. Needed to handle the ordering of the outer
    /// layers.
    pub(crate) fn extra_columns(&self) -> impl ExactSizeIterator<Item = Column<'static>> + '_ {
        self.extra_columns.clone().into_iter()
    }
}

// sigh, this is for pagination
fn _generate_filter(
    table_column: TableColumnWalker<'_>,
    fields: &[(&str, &serde_json::Value, OrderDirection)],
) -> Result<Option<Expression<'static>>, SdkError> {
    let mut filters: Vec<Expression<'static>> = Vec::new();
    let max_id = fields.len() - 1;

    for (i, (column, value, direction)) in fields.iter().enumerate() {
        let column = Column::from((*column).to_string());

        if i == max_id {
            if value.is_null() {
                if let OrderDirection::Asc = direction {
                    filters.push(column.is_not_null().into());
                }
            } else {
                let value = DatabaseValue::from_json_input(
                    (*value).clone(),
                    table_column.database_type(),
                    table_column.is_array(),
                )?;

                let expression = match table_column.enum_database_name() {
                    Some(enum_name) => Expression::enum_value(value, enum_name),
                    None => Expression::value(value),
                };

                match direction {
                    OrderDirection::Asc => {
                        filters.push(column.greater_than(expression).into());
                    }
                    OrderDirection::Desc => {
                        let tree = ConditionTree::Or(vec![
                            column.clone().less_than(expression).into(),
                            column.is_null().into(),
                        ]);

                        filters.push(tree.into());
                    }
                }
            }
        } else {
            let value = DatabaseValue::from_json_input(
                (*value).clone(),
                table_column.database_type(),
                table_column.is_array(),
            )?;

            let expression = match table_column.enum_database_name() {
                Some(enum_name) => Expression::enum_value(value, enum_name),
                None => Expression::value(value),
            };

            filters.push(column.equals(expression).into());
        }
    }

    if filters.is_empty() {
        Ok(None)
    } else if filters.len() == 1 {
        Ok(Some(filters.pop().unwrap()))
    } else {
        Ok(Some(ConditionTree::And(filters).into()))
    }
}
