use crate::ast::{Column, ConditionTree, Expression, Query, Table};

type Type<'a> = Column<'a>;

/// A builder for an `UPDATE` statement.
#[derive(Debug, PartialEq, Clone)]
pub struct Update<'a> {
    pub(crate) table: Table<'a>,
    pub(crate) columns: Vec<Column<'a>>,
    pub(crate) values: Vec<Expression<'a>>,
    pub(crate) conditions: Option<ConditionTree<'a>>,
    pub(crate) returning: Option<Vec<Type<'a>>>,
}

impl<'a> From<Update<'a>> for Query<'a> {
    fn from(update: Update<'a>) -> Self {
        Query::Update(Box::new(update))
    }
}

impl<'a> Update<'a> {
    /// Creates the basis for an `UPDATE` statement to the given table.
    pub fn table<T>(table: T) -> Self
    where
        T: Into<Table<'a>>,
    {
        Self {
            table: table.into(),
            columns: Vec::new(),
            values: Vec::new(),
            conditions: None,
            returning: None,
        }
    }

    /// Add another column value assignment to the query
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Update::table("users");
    /// query.set("foo", 10);
    /// query.set("bar", false);
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    /// assert_eq!(r#"UPDATE "users" SET "foo" = $1, "bar" = $2"#, sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         Value::from(10),
    ///         Value::from(false),
    ///     ],
    ///     params,
    /// );
    /// # }
    /// ```
    pub fn set<K, V>(&mut self, column: K, value: V)
    where
        K: Into<Column<'a>>,
        V: Into<Expression<'a>>,
    {
        self.columns.push(column.into());
        self.values.push(value.into());
    }

    /// Adds `WHERE` conditions to the query. See
    /// [Comparable](trait.Comparable.html#required-methods) for more examples.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Update::table("users");
    /// query.set("foo", 1);
    /// query.so_that("bar".equals(false));
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"UPDATE "users" SET "foo" = $1 WHERE "bar" = $2"#, sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         Value::from(1),
    ///         Value::from(false),
    ///     ],
    ///     params,
    /// );
    /// # }
    /// ```
    ///
    /// We can also use a nested `SELECT` in the conditions.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut select = Select::from_table("bars");
    /// select.column("id");
    /// select.so_that("uniq_val".equals(3));
    ///
    /// let mut query = Update::table("users");
    /// query.set("foo", 1);
    /// query.so_that("bar".equals(select));
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(
    ///     r#"UPDATE "users" SET "foo" = $1 WHERE "bar" = (SELECT "id" FROM "bars" WHERE "uniq_val" = $2)"#,
    ///     sql
    /// );
    ///
    /// assert_eq!(
    ///     vec![
    ///         Value::from(1),
    ///         Value::from(3),
    ///     ],
    ///     params,
    /// );
    /// # }
    /// ```
    pub fn so_that<T>(&mut self, conditions: T)
    where
        T: Into<ConditionTree<'a>>,
    {
        self.conditions = Some(conditions.into());
    }

    /// Sets the returned columns.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut update = Update::table("users");
    /// update.set("foo", 10);
    /// update.returning(vec!["id"]);
    ///
    /// let (sql, _) = renderer::Postgres::build(update);
    ///
    /// assert_eq!(r#"UPDATE "users" SET "foo" = $1 RETURNING "id""#, sql);
    /// # }
    /// ```
    pub fn returning<K, I>(&mut self, columns: I)
    where
        K: Into<Column<'a>>,
        I: IntoIterator<Item = K>,
    {
        self.returning = Some(columns.into_iter().map(|k| k.into()).collect());
    }
}
