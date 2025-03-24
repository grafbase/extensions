use crate::schema;
use serde::de;

pub(crate) struct GrpcMethodDirectiveArguments<'a> {
    pub(crate) schema: &'a schema::Schema,
    pub(crate) message: &'a schema::Message,
    pub(crate) out: &'a mut Vec<u8>,
}

impl<'de> de::DeserializeSeed<'de> for GrpcMethodDirectiveArguments<'_> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)?;

        Ok(())
    }
}

impl<'de> de::Visitor<'de> for GrpcMethodDirectiveArguments<'_> {
    type Value = ();

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        loop {
            let key: Option<&str> = map.next_key()?;

            match key {
                Some("method" | "service") => {
                    let _: String = map.next_value()?;
                }
                Some("input") => {
                    map.next_value_seed(super::arguments::ArgumentsDeserialize {
                        schema: self.schema,
                        message: self.message,
                        out: self.out,
                    })?;
                    return Ok(());
                }
                _ => {
                    return Err(de::Error::custom("expected an `input` argument on @grpcMethod"));
                }
            }
        }
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an `input` field")
    }
}
