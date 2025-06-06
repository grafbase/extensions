use super::{Walker, r#enum::EnumWalker};
use crate::{EnumVariant, EnumVariantId, StringId};

/// An enum variant definition.
pub type EnumVariantWalker<'a> = Walker<'a, EnumVariantId>;

impl<'a> EnumVariantWalker<'a> {
    /// The enum this variant belongs to.
    pub fn r#enum(self) -> EnumWalker<'a> {
        self.walk(self.get().enum_id())
    }

    /// The name of the variant in the database.
    pub fn database_name(self) -> &'a str {
        self.get_name(self.get().database_name())
    }

    /// The name of the variant in the GraphQL APIs.
    pub fn client_name(self) -> &'a str {
        self.get_name(self.get().client_name())
    }

    /// The description for the variant.
    pub fn description(self) -> Option<&'a str> {
        self.get().description().map(|id| self.get_name(id))
    }

    fn get(self) -> &'a EnumVariant<StringId> {
        &self.database_definition.enum_variants[self.id.0 as usize]
    }
}
