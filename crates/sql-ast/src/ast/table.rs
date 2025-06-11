use super::{Alias, Aliasable, ExpressionKind, Join, JoinData};
use crate::ast::{Expression, Select, Values};
use std::borrow::Cow;

#[derive(Clone, Debug)]
/// Either an identifier or a nested query.
pub enum TableType<'a> {
    Table(Cow<'a, str>),
    JoinedTable(Box<(Cow<'a, str>, Vec<Join<'a>>)>),
    Query(Box<Select<'a>>),
    Values(Values<'a>),
    Expression(Expression<'a>),
}

/// A table definition
#[derive(Clone, Debug)]
pub struct Table<'a> {
    pub typ: TableType<'a>,
    pub alias: Option<Alias<'a>>,
    pub database: Option<Cow<'a, str>>,
}

impl<'a> Table<'a> {
    /// Define in which database the table is located
    pub fn database<T>(mut self, database: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.database = Some(database.into());
        self
    }

    /// A qualified asterisk to this table
    pub fn asterisk(self) -> Expression<'a> {
        Expression {
            kind: ExpressionKind::Asterisk(Some(Box::new(self))),
            alias: None,
        }
    }

    /// Adds a `LEFT JOIN` clause to the query, specifically for that table.
    /// Useful to positionally add a JOIN clause in case you are selecting from multiple tables.
    pub fn left_join<J>(mut self, join: J) -> Self
    where
        J: Into<JoinData<'a>>,
    {
        match self.typ {
            TableType::Table(table_name) => {
                self.typ = TableType::JoinedTable(Box::new((table_name, vec![Join::Left(join.into())])))
            }
            TableType::JoinedTable(ref mut jt) => jt.1.push(Join::Left(join.into())),
            TableType::Expression(_) => {
                unreachable!("Please contact Grafbase support +555 420 69 69")
            }
            TableType::Query(_) => {
                unreachable!("You cannot left_join on a table of type Query")
            }
            TableType::Values(_) => {
                unreachable!("You cannot left_join on a table of type Values")
            }
        }

        self
    }

    /// Adds an `INNER JOIN` clause to the query, specifically for that table.
    /// Useful to positionally add a JOIN clause in case you are selecting from multiple tables.
    pub fn inner_join<J>(mut self, join: J) -> Self
    where
        J: Into<JoinData<'a>>,
    {
        match self.typ {
            TableType::Table(table_name) => {
                self.typ = TableType::JoinedTable(Box::new((table_name, vec![Join::Inner(join.into())])))
            }
            TableType::JoinedTable(ref mut jt) => jt.1.push(Join::Inner(join.into())),
            TableType::Expression(_) => {
                unreachable!("Please contact Grafbase support +555 420 69 69")
            }
            TableType::Query(_) => {
                unreachable!("You cannot inner_join on a table of type Query")
            }
            TableType::Values(_) => {
                unreachable!("You cannot inner_join on a table of type Values")
            }
        }

        self
    }

    /// Adds a `RIGHT JOIN` clause to the query, specifically for that table.
    /// Useful to positionally add a JOIN clause in case you are selecting from multiple tables.
    pub fn right_join<J>(mut self, join: J) -> Self
    where
        J: Into<JoinData<'a>>,
    {
        match self.typ {
            TableType::Table(table_name) => {
                self.typ = TableType::JoinedTable(Box::new((table_name, vec![Join::Right(join.into())])))
            }
            TableType::JoinedTable(ref mut jt) => jt.1.push(Join::Right(join.into())),
            TableType::Expression(_) => {
                unreachable!("Please contact Grafbase support +555 420 69 69")
            }
            TableType::Query(_) => {
                unreachable!("You cannot right_join on a table of type Query")
            }
            TableType::Values(_) => {
                unreachable!("You cannot right_join on a table of type Values")
            }
        }

        self
    }

    /// Adds a `FULL JOIN` clause to the query, specifically for that table.
    /// Useful to positionally add a JOIN clause in case you are selecting from multiple tables.
    pub fn full_join<J>(mut self, join: J) -> Self
    where
        J: Into<JoinData<'a>>,
    {
        match self.typ {
            TableType::Table(table_name) => {
                self.typ = TableType::JoinedTable(Box::new((table_name, vec![Join::Full(join.into())])))
            }
            TableType::JoinedTable(ref mut jt) => jt.1.push(Join::Full(join.into())),
            TableType::Expression(_) => {
                unreachable!("Please contact Grafbase support +555 420 69 69")
            }
            TableType::Query(_) => {
                unreachable!("You cannot full_join on a table of type Query")
            }
            TableType::Values(_) => {
                unreachable!("You cannot full_join on a table of type Values")
            }
        }

        self
    }
}

impl<'a> From<Expression<'a>> for Table<'a> {
    fn from(value: Expression<'a>) -> Self {
        Table {
            typ: TableType::Expression(value),
            alias: None,
            database: None,
        }
    }
}

impl<'a> From<&'a str> for Table<'a> {
    fn from(s: &'a str) -> Table<'a> {
        Table {
            typ: TableType::Table(s.into()),
            alias: None,
            database: None,
        }
    }
}

impl<'a> From<&'a String> for Table<'a> {
    fn from(s: &'a String) -> Table<'a> {
        Table {
            typ: TableType::Table(s.into()),
            alias: None,
            database: None,
        }
    }
}

impl<'a> From<Cow<'a, str>> for Table<'a> {
    fn from(s: Cow<'a, str>) -> Table<'a> {
        Table {
            typ: TableType::Table(s),
            alias: None,
            database: None,
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Table<'a> {
    fn from(s: (&'a str, &'a str)) -> Table<'a> {
        let table: Table<'a> = s.1.into();
        table.database(s.0)
    }
}

impl<'a> From<(&'a str, &'a String)> for Table<'a> {
    fn from(s: (&'a str, &'a String)) -> Table<'a> {
        let table: Table<'a> = s.1.into();
        table.database(s.0)
    }
}

impl<'a> From<(&'a String, &'a str)> for Table<'a> {
    fn from(s: (&'a String, &'a str)) -> Table<'a> {
        let table: Table<'a> = s.1.into();
        table.database(s.0)
    }
}

impl<'a> From<(&'a String, &'a String)> for Table<'a> {
    fn from(s: (&'a String, &'a String)) -> Table<'a> {
        let table: Table<'a> = s.1.into();
        table.database(s.0)
    }
}

impl From<String> for Table<'_> {
    fn from(s: String) -> Self {
        Table {
            typ: TableType::Table(s.into()),
            alias: None,
            database: None,
        }
    }
}

impl<'a> From<(String, String)> for Table<'a> {
    fn from(s: (String, String)) -> Table<'a> {
        let table: Table<'a> = s.1.into();
        table.database(s.0)
    }
}

impl<'a> From<Select<'a>> for Table<'a> {
    fn from(select: Select<'a>) -> Self {
        Table {
            typ: TableType::Query(Box::new(select)),
            alias: None,
            database: None,
        }
    }
}

impl<'a> Aliasable<'a> for Table<'a> {
    type Target = Table<'a>;

    fn alias<T>(mut self, alias: T) -> Self::Target
    where
        T: Into<Alias<'a>>,
    {
        self.alias = Some(alias.into());
        self
    }
}
