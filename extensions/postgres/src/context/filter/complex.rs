use crate::context::Context;
use grafbase_database_definition::{TableColumnWalker, TableWalker};
use grafbase_sdk::{
    SdkError,
    host_io::postgres::types::{DatabaseType, DatabaseValue},
};
use serde_json::{Map, Value};
use sql_ast::ast::{Column, Comparable, ConditionTree, Expression, Select};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct MultipleFilterIterator<'a> {
    context: &'a Context<'a>,
    table: TableWalker<'a>,
    filter: VecDeque<(String, Value)>,
}

impl<'a> MultipleFilterIterator<'a> {
    pub fn new(
        context: &'a Context<'a>,
        table: TableWalker<'a>,
        filter: impl IntoIterator<Item = (String, Value)>,
    ) -> Self {
        Self {
            context,
            table,
            filter: VecDeque::from_iter(filter),
        }
    }
}

impl<'a> Iterator for MultipleFilterIterator<'a> {
    type Item = Result<ConditionTree<'a>, SdkError>;

    fn next(&mut self) -> Option<Self::Item> {
        let (field, value) = self.filter.pop_front()?;

        // filtering from a related table.
        if let Some(relation) = self
            .context
            .database_definition
            .get_relation_for_client_name(self.table.id(), &field)
        {
            let object = if !relation.is_other_side_one() {
                match value {
                    Value::Object(mut object) => match object.remove("contains") {
                        Some(Value::Object(object)) => object,
                        _ => unreachable!("nested filters must be objects"),
                    },
                    _ => unreachable!("nested filters must be objects"),
                }
            } else {
                match value {
                    Value::Object(object) => object,
                    _ => unreachable!("nested filters must be objects"),
                }
            };

            let mut conditions = Vec::new();

            for (referenced, referencing) in relation.referenced_columns().zip(relation.referencing_columns()) {
                let referencing = Column::from((referencing.table().database_name(), referencing.database_name()));
                conditions.push(Expression::from(referenced.database_name().equals(referencing)));
            }

            let nested = Self::new(self.context, relation.referenced_table(), object);

            for condition in nested {
                match condition {
                    Ok(condition) => {
                        conditions.push(Expression::from(condition));
                    }
                    Err(e) => return Some(Err(e)),
                }
            }

            let table = relation.referenced_table();

            let mut select = Select::from_table((table.schema(), table.database_name()));
            select.value(Expression::value(1.into_bound_value(0)));
            select.so_that(ConditionTree::And(conditions));

            return Some(Ok(ConditionTree::exists(select)));
        }

        let operations = match value {
            Value::Object(operations) => operations,
            Value::Array(values) => {
                let mut operations = Vec::with_capacity(values.len());

                for operation in values.into_iter().filter_map(|operation| match operation {
                    Value::Object(obj) => Some(obj),
                    _ => None,
                }) {
                    let nested = Self::new(self.context, self.table, operation);

                    for operation in nested {
                        match operation {
                            Ok(operation) => {
                                operations.push(Expression::from(operation));
                            }
                            Err(e) => return Some(Err(e)),
                        }
                    }
                }

                let tree = match field.as_str() {
                    "ALL" => ConditionTree::And(operations),
                    "ANY" => ConditionTree::Or(operations),
                    "NONE" => ConditionTree::not(ConditionTree::Or(operations)),
                    _ => unreachable!(),
                };

                return Some(Ok(tree));
            }
            _ => return None,
        };

        let column = self
            .context
            .database_definition
            .find_column_for_client_field(&field, self.table.id())
            .expect("column for input field not found");

        match generate_conditions(operations, column) {
            Ok(conditions) => Some(Ok(conditions)),
            Err(err) => Some(Err(err)),
        }
    }
}

fn generate_conditions(
    operations: Map<String, Value>,
    column: TableColumnWalker<'_>,
) -> Result<ConditionTree<'_>, SdkError> {
    let mut compares = Vec::with_capacity(operations.len());

    for (key, value) in operations {
        let table_column = (column.table().client_name(), column.database_name());

        let expression = |value| {
            let db_value = DatabaseValue::from_json_input(value, column.database_type(), column.is_array())?;

            let expr = match column.enum_database_name() {
                Some(enum_type) => Expression::enum_value(db_value, enum_type),
                None => Expression::value(db_value),
            };

            Result::<_, SdkError>::Ok(expr)
        };

        let compare = match key.as_str() {
            "eq" => {
                if value.is_null() {
                    table_column.is_null()
                } else {
                    table_column.equals(expression(value)?)
                }
            }
            "ne" => {
                if value.is_null() {
                    table_column.is_not_null()
                } else {
                    table_column.not_equals(expression(value)?)
                }
            }
            "gt" => table_column.greater_than(expression(value)?),
            "lt" => table_column.less_than(expression(value)?),
            "gte" => table_column.greater_than_or_equals(expression(value)?),
            "lte" => table_column.less_than_or_equals(expression(value)?),
            "in" => table_column.any_selection(expression(value)?),
            "nin" => table_column.not_all_selection(expression(value)?),
            "contains" => table_column.array_contains(expression(value)?),
            "contained" => table_column.array_contained(expression(value)?),
            "overlaps" => table_column.array_overlaps(expression(value)?),
            "not" => {
                let operations = match value {
                    Value::Object(obj) => obj,
                    _ => unreachable!("non-object not filter"),
                };

                let condition = ConditionTree::not(generate_conditions(operations, column)?);
                let expression = Expression::from(condition);

                compares.push(expression);

                continue;
            }
            "like" => table_column.like(expression(value)?),
            _ => todo!(),
        };

        compares.push(Expression::from(compare));
    }

    Ok(ConditionTree::And(compares))
}
