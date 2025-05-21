use std::borrow::Cow;

use inflector::Inflector;

use super::{TableWalker, Walker};
use crate::{ColumnType, DatabaseType, IdentityGeneration, StringId, TableColumn, TableColumnId};

/// Definition of a column located in a table.
pub type TableColumnWalker<'a> = Walker<'a, TableColumnId>;

impl<'a> TableColumnWalker<'a> {
    /// The table this column is located.
    pub fn table(self) -> TableWalker<'a> {
        self.walk(self.get().table_id())
    }

    /// The name of the column in the database.
    pub fn database_name(self) -> &'a str {
        self.get_name(self.get().database_name())
    }

    /// The name of the column in the GraphQL APIs.
    pub fn client_name(self) -> &'a str {
        self.get_name(self.get().client_name())
    }

    /// Returns the provided alias if present, otherwise falls back to the client name.
    pub fn alias(self, alias: Option<&'a str>) -> &'a str {
        alias.unwrap_or_else(|| self.client_name())
    }

    /// The type of the column in the database.
    pub fn database_type(self) -> DatabaseType<'a> {
        match self.get().database_type() {
            ColumnType::Scalar(scalar) => DatabaseType::Scalar(scalar),
            ColumnType::Enum(r#enum) => DatabaseType::Enum(self.walk(r#enum.id)),
        }
    }

    /// If the column is an enum, returns the database type of the column for casting.
    pub fn enum_database_name(self) -> Option<String> {
        match self.database_type().enum_database_name() {
            Some(name) if self.is_array() => Some(format!("{name}[]")),
            Some(name) => Some(name.to_string()),
            None => None,
        }
    }

    pub fn client_base_type(self) -> Option<&'a str> {
        match self.database_type() {
            DatabaseType::Scalar(scalar_type) => scalar_type.client_type(),
            DatabaseType::Enum(walker) => Some(walker.client_name()),
        }
    }

    pub fn client_type(&self, prefix: Option<&str>) -> Option<Cow<'a, str>> {
        let r#type = match self.database_type() {
            DatabaseType::Scalar(scalar) if self.is_array() => {
                scalar.client_type().map(|t| format!("[{t}]")).map(Cow::from)
            }
            DatabaseType::Scalar(scalar) => scalar.client_type().map(Cow::from),
            DatabaseType::Enum(r#enum) if self.is_array() => {
                let name = match prefix {
                    Some(prefix) => format!("[{}_{}]", prefix, r#enum.client_name()).to_pascal_case(),
                    None => format!("[{}]", r#enum.client_name()),
                };

                Some(Cow::from(name))
            }
            DatabaseType::Enum(r#enum) => {
                let name = match prefix {
                    Some(prefix) => Cow::from(format!("{}_{}", prefix, r#enum.client_name()).to_pascal_case()),
                    None => Cow::from(r#enum.client_name()),
                };

                Some(name)
            }
        };

        if self.is_nullable() {
            r#type
        } else {
            r#type.map(|t| format!("{}!", t).into())
        }
    }

    /// The description of the column.
    pub fn description(self) -> Option<&'a str> {
        self.get().description().map(|id| self.get_name(id))
    }

    /// True, if the column is an array.
    pub fn is_array(self) -> bool {
        self.get().is_array()
    }

    /// True, if the column is nullable.
    pub fn is_nullable(self) -> bool {
        self.get().nullable
    }

    /// True, if the column allows null input.
    pub fn allows_null_input(self) -> bool {
        self.is_nullable() || self.get().has_default || self.identity_generation().is_some()
    }

    /// True, if user can define the column value manually.
    pub fn allows_user_input(self) -> bool {
        !matches!(self.identity_generation(), Some(IdentityGeneration::Always))
    }

    /// True, if the column is part of any key in the table (primary, unique, foreign).
    pub fn is_part_of_a_key(self) -> bool {
        self.table()
            .keys()
            .any(|k| k.columns().any(|c| c.table_column().id() == self.id()))
    }

    fn identity_generation(self) -> Option<IdentityGeneration> {
        self.get().identity_generation()
    }

    fn get(self) -> &'a TableColumn<StringId> {
        &self.database_definition.table_columns[self.id.0 as usize]
    }
}
