mod field_type;
mod ids;
mod records;

pub(crate) use self::{field_type::*, ids::*, records::*};

use crate::directives;
use grafbase_sdk::types::Error;
use std::collections::HashMap;

pub(crate) struct Schema {
    services: Vec<directives::ProtoServiceDefinition>,
    messages: Vec<Message>,
    enum_definitions: Vec<EnumDefinition>,
}

impl Schema {
    pub(crate) fn new(
        services: Vec<directives::ProtoServiceDefinition>,
        messages_from_directives: Vec<directives::ProtoMessageDefinition>,
        enum_definitions_from_directives: Vec<directives::ProtoEnumDefinition>,
    ) -> Result<Self, Error> {
        let mut enums_by_name: HashMap<&str, EnumDefinitionId> =
            HashMap::with_capacity(enum_definitions_from_directives.len());
        let mut messages_by_name: HashMap<&str, MessageDefinitionId> =
            HashMap::with_capacity(messages_from_directives.len());

        let mut messages = Vec::with_capacity(messages_from_directives.len());
        let mut enums = Vec::with_capacity(enum_definitions_from_directives.len());

        for r#enum in &enum_definitions_from_directives {
            let values = r#enum
                .values
                .iter()
                .map(|value| EnumValueDefinition {
                    name: value.name.clone(),
                    number: value.number,
                })
                .collect();

            let id = enums.len().into();
            enums_by_name.insert(r#enum.name.as_str(), id);
            enums.push(EnumDefinition {
                name: r#enum.name.clone(),
                values,
            })
        }

        for (idx, message) in messages_from_directives.iter().enumerate() {
            messages_by_name.insert(message.name.as_str(), MessageDefinitionId::from(idx));
        }

        for message in &messages_from_directives {
            let fields = message
                .fields
                .iter()
                .map(|field| -> Result<_, Error> {
                    Ok((
                        field.name.clone(),
                        Field {
                            name: field.name.clone(),
                            ty: ingest_type(&field.r#type, &messages_by_name, &enums_by_name).ok_or_else(|| {
                                Error::new(format!(
                                    "Unknown type `{}` at `{}.{}`",
                                    field.r#type, message.name, field.name
                                ))
                            })?,
                            number: field.number,
                            repeated: field.repeated,
                        },
                    ))
                })
                .collect::<Result<_, Error>>()?;

            messages.push(Message {
                name: message.name.clone(),
                fields,
            })
        }

        Ok(Schema {
            services,
            messages,
            enum_definitions: enums,
        })
    }

    pub(crate) fn get_service(&self, name: &str) -> Option<&directives::ProtoServiceDefinition> {
        self.services.iter().find(|s| s.name == name)
    }

    pub(crate) fn get_message(&self, name: &str) -> Option<&Message> {
        self.messages.iter().find(|m| m.name == name)
    }
}
