use crate::ast::{ConditionTree, Query, Table};

use super::Expression;

#[derive(Debug, PartialEq, Clone)]
/// A builder for a `DELETE` statement.
pub struct Delete<'a> {
    pub(crate) table: Table<'a>,
    pub(crate) conditions: Option<ConditionTree<'a>>,
    pub(crate) returning: Option<Vec<Expression<'a>>>,
}

impl<'a> From<Delete<'a>> for Query<'a> {
    fn from(delete: Delete<'a>) -> Self {
        Query::Delete(Box::new(delete))
    }
}

impl<'a> Delete<'a> {
    /// Creates a new `DELETE` statement for the given table.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let query = Delete::from_table("users");
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"DELETE FROM "users""#, sql);
    /// # }
    /// ```
    pub fn from_table<T>(table: T) -> Self
    where
        T: Into<Table<'a>>,
    {
        Self {
            table: table.into(),
            conditions: None,
            returning: None,
        }
    }

    /// Adds `WHERE` conditions to the query. See
    /// [Comparable](trait.Comparable.html#required-methods) for more examples.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Delete::from_table("users");
    /// query.so_that("bar".equals(false));
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"DELETE FROM "users" WHERE "bar" = $1"#, sql);
    /// assert_eq!(vec![Value::from(false)], params);
    /// # }
    /// ```
    pub fn so_that<T>(&mut self, conditions: T)
    where
        T: Into<ConditionTree<'a>>,
    {
        self.conditions = Some(conditions.into());
    }

    /// Adds a `RETURNING` definition to the `DELETE` statement. Defines the return
    /// value of the query.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Delete::from_table("users");
    /// query.returning([Column::from("id"), Column::from("name")]);
    ///
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"DELETE FROM "users" RETURNING "id", "name""#, sql);
    /// # }
    /// ```
    pub fn returning<E, T>(&mut self, returning: T)
    where
        T: IntoIterator<Item = E>,
        E: Into<Expression<'a>>,
    {
        self.returning = Some(returning.into_iter().map(Into::into).collect());
    }
}
