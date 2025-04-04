use super::{
    RelationWalker, Walker, forward_relation::ForwardRelationWalker, key::KeyWalker, table_column::TableColumnWalker,
};
use crate::database_definition::{
    KeyId, RelationId, Table, TableColumnId, TableId,
    ids::{BackRelationId, ForwardRelationId},
    names::StringId,
};

/// Definition of a table.
pub type TableWalker<'a> = Walker<'a, TableId>;

impl<'a> TableWalker<'a> {
    /// The name of the schema this table is located.
    pub fn schema(self) -> &'a str {
        &self.database_definition.schemas[self.get().schema_id().0 as usize]
    }

    /// The name of the table in the database.
    pub fn database_name(self) -> &'a str {
        self.get_name(self.get().database_name())
    }

    /// The name of the table in the GraphQL APIs.
    pub fn client_name(self) -> &'a str {
        self.get_name(self.get().client_name())
    }

    /// An iterator over all the columns in the table.
    pub fn columns(self) -> impl Iterator<Item = TableColumnWalker<'a>> + 'a {
        let range = super::range_for_key(&self.database_definition.table_columns, self.id, |column| {
            column.table_id()
        });

        range.map(move |id| self.walk(TableColumnId(id as u32)))
    }

    /// A table can be used in the client, if it has at least one supported column
    /// and at least one unique constraint that contains columns of supported type.
    pub fn allowed_in_client(self) -> bool {
        self.columns().next().is_some() && self.keys().next().is_some()
    }

    /// A special unique index that acts as the primary key of the column.
    pub fn primary_key(self) -> Option<KeyWalker<'a>> {
        self.keys().find(|constraint| constraint.is_primary())
    }

    /// The key used to implicitly order a result set if no order defined by the user.
    pub fn implicit_ordering_key(self) -> Option<KeyWalker<'a>> {
        self.primary_key().or_else(|| self.keys().next())
    }

    /// An iterator over all the unqiue constraints, including the primary key.
    pub fn keys(self) -> impl Iterator<Item = KeyWalker<'a>> + 'a {
        let range = super::range_for_key(&self.database_definition.keys, self.id, |constraint| {
            constraint.table_id()
        });

        range.map(move |id| self.walk(KeyId(id as u32)))
    }

    /// Find a column by database name.
    pub fn find_database_column(self, name: &str) -> Option<TableColumnWalker<'a>> {
        self.database_definition
            .names
            .get_table_column_id(self.id, name)
            .map(|id| self.walk(id))
    }

    /// Find a unique constraint by name.
    pub fn find_unique_constraint(self, constraint_name: &str) -> Option<KeyWalker<'a>> {
        self.database_definition
            .names
            .get_unique_constraint_id(self.id, constraint_name)
            .map(|id| self.walk(id))
    }

    /// Iterate over all relations stemming from a foreign key on this table.
    pub(crate) fn forward_relations(self) -> impl Iterator<Item = ForwardRelationWalker<'a>> {
        let range = super::range_for_key(&self.database_definition.relations.from, self.id, |(table_id, _)| {
            *table_id
        });

        range
            .map(move |id| self.walk(ForwardRelationId(id as u32)))
            .filter(|relation| relation.referenced_table_is_allowed_in_client())
    }

    /// An iterator over relations having the foreign key constraint defined from or into this table.
    pub fn relations(self) -> impl Iterator<Item = RelationWalker<'a>> + 'a {
        let range = super::range_for_key(&self.database_definition.relations.to, self.id, |(table_id, _)| {
            *table_id
        });

        let back = range
            .map(move |id| self.walk(RelationId::Back(BackRelationId(id as u32))))
            .filter(|relation| relation.referenced_table_is_allowed_in_client());

        self.forward_relations()
            .map(move |fwd| self.walk(RelationId::Forward(fwd.id)))
            .chain(back)
    }

    fn get(self) -> &'a Table<StringId> {
        &self.database_definition.tables[self.id.0 as usize]
    }
}
