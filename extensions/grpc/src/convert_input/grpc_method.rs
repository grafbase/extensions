use crate::{Grpc, directives::GrpcMethod};
use serde::de;

pub(crate) struct GrpcMethodArgumentsDeserialize<'a> {
    pub(crate) schema: &'a Grpc,
}

impl<'de> de::DeserializeSeed<'de> for GrpcMethodArgumentsDeserialize<'de> {
    type Value = GrpcMethod;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RootVisitor<'a> {
            schema: &'a Grpc,
        };

        impl<'de> de::Visitor<'de> for RootVisitor<'de> {
            type Value = GrpcMethod;

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut service: Option<String> = None;
                let mut method: Option<String> = None;
                let mut input: Option<serde_json::Value> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "service" => {
                            service = Some(map.next_value()?);
                        }
                        "method" => {
                            method = Some(map.next_value()?);
                        }
                        "input" => {
                            input = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                key.as_str(),
                                &["service", "method", "input"],
                            ));
                        }
                    }
                }

                Ok(GrpcMethod {
                    service: service.ok_or_else(|| serde::de::Error::missing_field("service"))?,
                    method: method.ok_or_else(|| serde::de::Error::missing_field("method"))?,
                    input: input.ok_or_else(|| serde::de::Error::missing_field("input"))?,
                })
            }

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("valid arguments for @grpcMethod")
            }
        }

        deserializer.deserialize_map(RootVisitor { schema: self.schema })
    }
}
