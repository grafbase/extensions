use std::borrow::Cow;

use super::{
    Column, CommonTableExpression, ConditionTree, Expression, ExpressionKind, Grouping, IntoGroupByDefinition,
    IntoOrderDefinition, Join, JoinData, Ordering, Query, Table,
};

type Type<'a> = ConditionTree<'a>;

/// A builder for a `SELECT` statement.
#[derive(Debug, Clone, Default)]
pub struct Select<'a> {
    pub(crate) ctes: Vec<CommonTableExpression<'a>>,
    pub(crate) distinct: bool,
    pub(crate) tables: Vec<Table<'a>>,
    pub(crate) columns: Vec<Expression<'a>>,
    pub(crate) conditions: Option<ConditionTree<'a>>,
    pub(crate) ordering: Ordering<'a>,
    pub(crate) grouping: Grouping<'a>,
    pub(crate) having: Option<Type<'a>>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
    pub(crate) joins: Vec<Join<'a>>,
    #[allow(dead_code)]
    pub(crate) comment: Option<Cow<'a, str>>,
}

impl<'a> From<Select<'a>> for Expression<'a> {
    fn from(sel: Select<'a>) -> Expression<'a> {
        Expression {
            kind: ExpressionKind::Selection(Box::new(sel)),
            alias: None,
        }
    }
}

impl<'a> From<Select<'a>> for Query<'a> {
    fn from(sel: Select<'a>) -> Query<'a> {
        Query::Select(Box::new(sel))
    }
}

impl<'a> Select<'a> {
    /// Creates a new `SELECT` statement for the given table.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{self, Renderer}};
    /// # fn main() {
    /// let query = Select::from_table("users");
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "users".* FROM "users""#, sql);
    /// # }
    /// ```
    ///
    /// The table can be in multiple parts, defining the schema.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{self, Renderer}};
    /// # fn main() {
    /// let query = Select::from_table(("crm", "users"));
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "crm"."users".* FROM "crm"."users""#, sql);
    /// # }
    /// ```
    ///
    /// Selecting from a nested `SELECT`.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{self, Renderer}};
    /// # fn main() {
    /// let mut inner_select = Select::default();
    /// inner_select.value(1);
    ///
    /// let select = Table::from(inner_select).alias("num");
    /// let query = Select::from_table(select.alias("num"));
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "num".* FROM (SELECT $1) AS "num""#, sql);
    /// assert_eq!(vec![Value::from(1)], params);
    /// # }
    /// ```
    pub fn from_table<T>(table: T) -> Self
    where
        T: Into<Table<'a>>,
    {
        Select {
            tables: vec![table.into()],
            ..Select::default()
        }
    }

    /// Adds a table to be selected.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    ///
    /// let mut inner_select = Select::default();
    /// inner_select.value(1);
    ///
    /// query.and_from(Table::from(inner_select).alias("num"));
    /// query.column(("users", "name"));
    /// query.value(Table::from("num").asterisk());
    ///
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "users"."name", "num".* FROM "users", (SELECT $1) AS "num""#, sql);
    /// # }
    /// ```
    pub fn and_from<T>(&mut self, table: T)
    where
        T: Into<Table<'a>>,
    {
        self.tables.push(table.into());
    }

    /// Selects a static value as the column.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::default();
    /// query.value(1);
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!("SELECT $1", sql);
    /// assert_eq!(vec![Value::from(1)], params);
    /// # }
    /// ```
    pub fn value<T>(&mut self, value: T)
    where
        T: Into<Expression<'a>>,
    {
        self.columns.push(value.into());
    }

    /// Adds a column to be selected.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    ///
    /// query.column("name");
    /// query.column(("users", "id"));
    /// query.column((("crm", "users"), "foo"));
    ///
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "name", "users"."id", "crm"."users"."foo" FROM "users""#, sql);
    /// # }
    /// ```
    pub fn column<T>(&mut self, column: T)
    where
        T: Into<Column<'a>>,
    {
        self.columns.push(column.into().into());
    }

    /// A bulk method to select multiple values.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    /// query.columns(["foo", "bar"]);
    ///
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "foo", "bar" FROM "users""#, sql);
    /// # }
    /// ```
    pub fn columns<T, C>(&mut self, columns: T)
    where
        T: IntoIterator<Item = C>,
        C: Into<Column<'a>>,
    {
        self.columns = columns.into_iter().map(|c| c.into().into()).collect();
    }

    /// Adds `DISTINCT` to the select query.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    ///
    /// query.column("foo");
    /// query.column("bar");
    /// query.distinct();
    ///
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT DISTINCT "foo", "bar" FROM "users""#, sql);
    /// # }
    /// ```
    pub fn distinct(&mut self) {
        self.distinct = true;
    }

    /// Adds `WHERE` conditions to the query, replacing the previous conditions.
    /// See [Comparable](trait.Comparable.html#required-methods) for more
    /// examples.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    /// query.so_that("foo".equals("bar"));
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "users".* FROM "users" WHERE "foo" = $1"#, sql);
    ///
    /// assert_eq!(vec![
    ///    Value::from("bar"),
    /// ], params);
    /// # }
    /// ```
    pub fn so_that<T>(&mut self, conditions: T)
    where
        T: Into<ConditionTree<'a>>,
    {
        self.conditions = Some(conditions.into());
    }

    /// Adds an additional `WHERE` condition to the query combining the possible
    /// previous condition with `AND`. See
    /// [Comparable](trait.Comparable.html#required-methods) for more examples.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    ///
    /// query.so_that("foo".equals("bar"));
    /// query.and_where("lol".equals("wtf"));
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "users".* FROM "users" WHERE ("foo" = $1 AND "lol" = $2)"#, sql);
    ///
    /// assert_eq!(vec![
    ///    Value::from("bar"),
    ///    Value::from("wtf"),
    /// ], params);
    /// # }
    /// ```
    pub fn and_where<T>(&mut self, conditions: T)
    where
        T: Into<ConditionTree<'a>>,
    {
        match self.conditions.take() {
            Some(previous) => {
                self.conditions = Some(previous.and(conditions.into()));
            }
            None => self.so_that(conditions),
        }
    }

    /// Adds an additional `WHERE` condition to the query combining the possible
    /// previous condition with `OR`. See
    /// [Comparable](trait.Comparable.html#required-methods) for more examples.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    ///
    /// query.so_that("foo".equals("bar"));
    /// query.or_where("lol".equals("wtf"));
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "users".* FROM "users" WHERE ("foo" = $1 OR "lol" = $2)"#, sql);
    ///
    /// assert_eq!(vec![
    ///    Value::from("bar"),
    ///    Value::from("wtf"),
    /// ], params);
    /// # }
    /// ```
    pub fn or_where<T>(&mut self, conditions: T)
    where
        T: Into<ConditionTree<'a>>,
    {
        match self.conditions.take() {
            Some(previous) => {
                self.conditions = Some(previous.or(conditions.into()));
            }
            None => self.so_that(conditions),
        }
    }

    /// Adds `INNER JOIN` clause to the query.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let join = Table::from("posts")
    ///     .alias("p")
    ///     .on(("p", "user_id").equals(Column::from(("users", "id"))));
    ///
    /// let mut query = Select::from_table("users");
    /// query.inner_join(join);
    ///
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(
    ///     r#"SELECT "users".* FROM "users" INNER JOIN "posts" AS "p" ON "p"."user_id" = "users"."id""#,
    ///     sql
    /// );
    /// # }
    /// ```
    pub fn inner_join<J>(&mut self, join: J)
    where
        J: Into<JoinData<'a>>,
    {
        self.joins.push(Join::Inner(join.into()));
    }

    /// Adds `LEFT JOIN` clause to the query.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let join = Table::from("posts")
    ///    .alias("p")
    ///    .on(("p", "visible").equals(true));
    ///
    /// let mut query = Select::from_table("users");
    /// query.left_join(join);
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(
    ///     r#"SELECT "users".* FROM "users" LEFT JOIN "posts" AS "p" ON "p"."visible" = $1"#,
    ///     sql
    /// );
    ///
    /// assert_eq!(
    ///     vec![
    ///         Value::from(true),
    ///     ],
    ///     params
    /// );
    /// # }
    /// ```
    pub fn left_join<J>(&mut self, join: J)
    where
        J: Into<JoinData<'a>>,
    {
        self.joins.push(Join::Left(join.into()));
    }

    /// Adds `RIGHT JOIN` clause to the query.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let join = Table::from("posts")
    ///    .alias("p")
    ///    .on(("p", "visible").equals(true));
    ///
    ///
    /// let mut query = Select::from_table("users");
    /// query.right_join(join);
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(
    ///     r#"SELECT "users".* FROM "users" RIGHT JOIN "posts" AS "p" ON "p"."visible" = $1"#,
    ///     sql
    /// );
    ///
    /// assert_eq!(
    ///     vec![
    ///         Value::from(true),
    ///     ],
    ///     params
    /// );
    /// # }
    /// ```
    pub fn right_join<J>(&mut self, join: J)
    where
        J: Into<JoinData<'a>>,
    {
        self.joins.push(Join::Right(join.into()));
    }

    /// Adds `FULL JOIN` clause to the query.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let join = Table::from("posts")
    ///    .alias("p")
    ///    .on(("p", "visible").equals(true));
    ///
    /// let mut query = Select::from_table("users");
    /// query.full_join(join);
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(
    ///     r#"SELECT "users".* FROM "users" FULL JOIN "posts" AS "p" ON "p"."visible" = $1"#,
    ///     sql
    /// );
    ///
    /// assert_eq!(
    ///     vec![
    ///         Value::from(true),
    ///     ],
    ///     params
    /// );
    /// # }
    /// ```
    pub fn full_join<J>(&mut self, join: J)
    where
        J: Into<JoinData<'a>>,
    {
        self.joins.push(Join::Full(join.into()));
    }

    /// Adds `CROSS JOIN` clause to the query.
    ///
    /// The cross join condition must be `None`.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let join = Table::from("posts").alias("p");
    ///
    /// let mut query = Select::from_table("users");
    /// query.cross_join(join);
    ///
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(
    ///     r#"SELECT "users".* FROM "users" CROSS JOIN "posts" AS "p""#,
    ///     sql
    /// );
    /// # }
    /// ```
    pub fn cross_join<J>(&mut self, join: J)
    where
        J: Into<JoinData<'a>>,
    {
        self.joins.push(Join::Cross(join.into()));
    }

    /// Adds an ordering to the `ORDER BY` section.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    /// query.order_by("foo");
    /// query.order_by("baz".ascend());
    /// query.order_by("bar".descend());
    ///
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "users".* FROM "users" ORDER BY "foo", "baz" ASC, "bar" DESC"#, sql);
    /// # }
    pub fn order_by<T>(&mut self, value: T)
    where
        T: IntoOrderDefinition<'a>,
    {
        self.ordering.append(value.into_order_definition());
    }

    /// Adds a grouping to the `GROUP BY` section.
    ///
    /// This does not check if the grouping is actually valid in respect to aggregated columns.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    ///
    /// query.column("foo");
    /// query.column("bar");
    /// query.group_by("foo");
    /// query.group_by("bar");
    ///
    /// let (sql, _) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "foo", "bar" FROM "users" GROUP BY "foo", "bar""#, sql);
    /// # }
    pub fn group_by<T>(&mut self, value: T)
    where
        T: IntoGroupByDefinition<'a>,
    {
        self.grouping.append(value.into_group_by_definition());
    }

    /// Adds group conditions to a query. Should be combined together with a
    /// [group_by](struct.Select.html#method.group_by) statement.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    ///
    /// query.column("foo");
    /// query.column("bar");
    /// query.group_by("foo");
    /// query.having("foo".greater_than(100));
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "foo", "bar" FROM "users" GROUP BY "foo" HAVING "foo" > $1"#, sql);
    /// assert_eq!(vec![Value::from(100)], params);
    /// # }
    pub fn having<T>(&mut self, conditions: T)
    where
        T: Into<ConditionTree<'a>>,
    {
        self.having = Some(conditions.into());
    }

    /// Sets the `LIMIT` value.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    /// query.limit(10);
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "users".* FROM "users" LIMIT $1"#, sql);
    /// assert_eq!(vec![Value::from(10_i64)], params);
    /// # }
    pub fn limit(&mut self, limit: u32) {
        self.limit = Some(limit);
    }

    /// Sets the `OFFSET` value.
    ///
    /// ```rust
    /// # use grafbase_sql_ast::{ast::*, renderer::{Renderer, self}};
    /// # fn main() {
    /// let mut query = Select::from_table("users");
    /// query.offset(10);
    ///
    /// let (sql, params) = renderer::Postgres::build(query);
    ///
    /// assert_eq!(r#"SELECT "users".* FROM "users" OFFSET $1"#, sql);
    /// assert_eq!(vec![Value::from(10_i64)], params);
    /// # }
    pub fn offset(&mut self, offset: u32) {
        self.offset = Some(offset);
    }

    /// Adds a common table expression to the select.
    pub fn with(&mut self, cte: CommonTableExpression<'a>) {
        self.ctes.push(cte);
    }
}
