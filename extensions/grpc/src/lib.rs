mod config;
mod conversions;
mod directives;
mod resolvers;
mod schema;

use grafbase_sdk::{
    FieldResolverExtension, Subscription,
    types::{
        Configuration, Error, FieldDefinitionDirective, FieldInputs, FieldOutputs, SchemaDirective, SubgraphHeaders,
    },
};

#[derive(FieldResolverExtension)]
struct Grpc {
    schema: schema::Schema,
    configuration: config::GrpcConfiguration,
}

impl FieldResolverExtension for Grpc {
    fn new(schema_directives: Vec<SchemaDirective>, config: Configuration) -> Result<Self, Error> {
        let mut services = Vec::new();
        let mut messages = Vec::new();
        let mut enums = Vec::new();

        let configuration = config.deserialize()?;

        for directive in schema_directives {
            match directive.name() {
                "protoMessages" => {
                    let directives::ProtoMessages {
                        definitions: message_definitions,
                    } = directive.arguments()?;

                    messages.extend(message_definitions.into_iter());
                }
                "protoServices" => {
                    let directives::ProtoServices {
                        definitions: service_definitions,
                    } = directive.arguments()?;

                    services.extend(service_definitions.into_iter());
                }
                "protoEnums" => {
                    let directives::ProtoEnums {
                        definitions: enum_definitions,
                    } = directive.arguments()?;

                    enums.extend(enum_definitions.into_iter());
                }
                other => unreachable!("Unknown directive: {other}"),
            }
        }

        Ok(Grpc {
            schema: schema::Schema::new(services, messages, enums)?,
            configuration,
        })
    }

    fn resolve_field(
        &mut self,
        _headers: SubgraphHeaders,
        _subgraph_name: &str,
        directive: FieldDefinitionDirective<'_>,
        inputs: FieldInputs,
    ) -> Result<FieldOutputs, Error> {
        resolvers::grpc_method(directive, inputs, &self.schema, &self.configuration)
    }

    fn resolve_subscription<'a>(
        &'a mut self,
        _headers: SubgraphHeaders,
        _subgraph_name: &str,
        directive: FieldDefinitionDirective<'_>,
    ) -> Result<Box<dyn Subscription + 'a>, Error> {
        resolvers::grpc_method_subscription(directive, &self.schema, &self.configuration)
    }
}
