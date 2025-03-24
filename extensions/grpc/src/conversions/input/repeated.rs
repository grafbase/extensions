use crate::schema;
use serde::de;

pub(crate) struct RepeatedFieldVisitor<'a> {
    pub(super) tag: u32,
    pub(super) schema: &'a schema::Schema,
    pub(super) field: &'a schema::Field,
    pub(super) out: &'a mut Vec<u8>,
}

impl<'de> de::DeserializeSeed<'de> for RepeatedFieldVisitor<'_> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}

impl<'de> de::Visitor<'de> for RepeatedFieldVisitor<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        while seq
            .next_element_seed(super::field::FieldVisitor {
                tag: self.tag,
                schema: self.schema,
                field_type: &self.field.ty,
                out: self.out,
            })?
            .is_some()
        {}

        Ok(())
    }
}
