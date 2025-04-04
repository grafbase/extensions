use std::borrow::Cow;

use inflector::Inflector;
use itertools::Itertools;

use super::{ForeignKeyWalker, TableColumnWalker, TableWalker, Walker};
use crate::RelationId;

pub type RelationWalker<'a> = Walker<'a, RelationId>;

impl<'a> RelationWalker<'a> {
    /// The table this relation starts from.
    pub fn referencing_table(self) -> TableWalker<'a> {
        match self.id() {
            RelationId::Forward(id) => self.walk(id).referencing_table(),
            RelationId::Back(id) => self.walk(id).referencing_table(),
        }
    }

    /// The opposite table.
    pub fn referenced_table(self) -> TableWalker<'a> {
        match self.id() {
            RelationId::Forward(id) => self.walk(id).referenced_table(),
            RelationId::Back(id) => self.walk(id).referenced_table(),
        }
    }

    /// The columns on this table that are forming the constraint.
    pub fn referencing_columns(self) -> Box<dyn ExactSizeIterator<Item = TableColumnWalker<'a>> + 'a> {
        match self.id() {
            RelationId::Forward(id) => Box::new(self.walk(id).referencing_columns()),
            RelationId::Back(id) => Box::new(self.walk(id).referencing_columns()),
        }
    }

    /// The columns on the other table that are forming the constraint.
    pub fn referenced_columns(self) -> Box<dyn ExactSizeIterator<Item = TableColumnWalker<'a>> + 'a> {
        match self.id() {
            RelationId::Forward(id) => Box::new(self.walk(id).referenced_columns()),
            RelationId::Back(id) => Box::new(self.walk(id).referenced_columns()),
        }
    }

    /// True, if the referenced column(s) is (are) unique, this means there can only be at most one row on the other side of the relation.
    pub fn is_other_side_one(self) -> bool {
        self.referenced_table()
            .keys()
            .any(|constraint| constraint.has_all_the_columns(self.referenced_columns()))
    }

    /// True, if we use the referenced table in the client. E.g. it has at least one
    /// column of supported type and one unique constraint.
    pub fn referenced_table_is_allowed_in_client(self) -> bool {
        self.referenced_table().allowed_in_client()
    }

    /// Returns `true` if this relation is from the side that has the foreign key.
    pub fn is_referencing_side(&self) -> bool {
        self.id().is_forward()
    }

    /// Returns `true` if this relation is from the side that is referenced.
    pub fn is_referenced_side(&self) -> bool {
        self.id().is_backward()
    }

    /// The client type of the relation field.
    pub fn client_type(self) -> Cow<'a, str> {
        let base_name = self.referenced_table().client_name();

        if self.is_other_side_one() {
            let columns_nullable = self.referencing_columns().any(|c| c.is_nullable());

            // The side that defines the foreign key is nullable if any of the referencing
            // columns are nullable. The referenced side is always nullable.
            if columns_nullable || self.is_referenced_side() {
                Cow::Borrowed(base_name)
            } else {
                Cow::Owned(format!("{base_name}!"))
            }
        } else {
            Cow::Owned(format!("{base_name}Connection!"))
        }
    }

    /// The client-side field name for this relation.
    pub fn client_field_name(&self) -> String {
        let base_name = if self.is_other_side_one() {
            self.referenced_table().client_field_name()
        } else {
            self.referenced_table().client_field_name_plural()
        };

        let mut is_name_collision = self
            .referencing_table()
            .columns()
            .any(|column| column.client_name() == base_name);

        let fk = self.foreign_key();

        is_name_collision |= {
            let constrained_table = fk.constrained_table();
            let referenced_table = fk.referenced_table();
            constrained_table
                .forward_relations()
                .any(|relation| relation.foreign_key() != fk && relation.referenced_table() == referenced_table)
        };

        if is_name_collision {
            let referencing_columns = fk
                .columns()
                .map(|column| column.constrained_column().client_name())
                .join("_");

            format!("{base_name}_by_{referencing_columns}").to_camel_case()
        } else {
            base_name.to_string()
        }
    }

    /// Returns the name of the relation.
    pub fn name(self) -> &'a str {
        self.foreign_key().name()
    }

    /// Returns the description of the relation.
    pub fn description(self) -> Option<&'a str> {
        self.foreign_key().description()
    }

    /// The foreign key backing the relation.
    fn foreign_key(self) -> ForeignKeyWalker<'a> {
        match self.id() {
            RelationId::Forward(id) => self.walk(id).foreign_key(),
            RelationId::Back(id) => self.walk(id).foreign_key(),
        }
    }
}
