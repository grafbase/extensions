use super::{Walker, enum_variant::EnumVariantWalker};
use crate::{Enum, EnumId, EnumVariantId, StringId};

/// An enum definition in the database.
pub type EnumWalker<'a> = Walker<'a, EnumId>;

impl<'a> EnumWalker<'a> {
    /// The schema this enum belongs to.
    pub fn schema(self) -> &'a str {
        &self.database_definition.schemas[self.get().schema_id().0 as usize]
    }

    /// The name of the enum in the database.
    pub fn database_name(self) -> &'a str {
        self.get_name(self.get().database_name())
    }

    /// The name of the enum in the GraphQL APIs.
    pub fn client_name(self) -> &'a str {
        self.get_name(self.get().client_name())
    }

    /// The variants that are part of the enum.
    pub fn variants(self) -> impl ExactSizeIterator<Item = EnumVariantWalker<'a>> + 'a {
        let range = super::range_for_key(&self.database_definition.enum_variants, self.id, |variant| {
            variant.enum_id()
        });

        range.map(move |id| self.walk(EnumVariantId(id as u32)))
    }

    /// Find a variant with a given client name, returning the database name.
    pub fn rename_variant(self, client_name: &str) -> Option<&'a str> {
        self.variants()
            .find(|variant| variant.client_name() == client_name)
            .map(|variant| variant.database_name())
    }

    /// The description of the enum in the GraphQL APIs.
    pub fn description(self) -> Option<&'a str> {
        self.get().description().map(|id| self.get_name(id))
    }

    fn get(self) -> &'a Enum<StringId> {
        &self.database_definition.enums[self.id.0 as usize]
    }
}
