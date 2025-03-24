use crate::schema;

macro_rules! id_types {
    ($($field:ident[$id:ident] => $record:path),*) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub(crate) struct $id(usize);

            impl From<usize> for $id {
                fn from(id: usize) -> Self {
                    $id(id)
                }
            }

            impl From<$id> for usize {
                fn from(id: $id) -> Self {
                    id.0
                }
            }

            impl std::ops::Index<$id> for schema::Schema {
                type Output = $record;

                fn index(&self, index: $id) -> &Self::Output {
                    &self.$field[index.0]
                }
            }
        )*
    }
}

id_types! {
    messages[MessageDefinitionId] => schema::Message,
    services[ServiceId] => schema::Service,
    enum_definitions[EnumDefinitionId] => schema::EnumDefinition
}
