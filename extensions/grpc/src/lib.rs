mod config;
mod conversions;
mod directives;
mod resolvers;
mod schema;

use grafbase_sdk::{
    IntoSubscription, ResolverExtension,
    types::{Configuration, Error, ResolvedField, Response, SubgraphHeaders, SubgraphSchema, Variables},
};

#[derive(ResolverExtension)]
struct Grpc {
    schema: schema::Schema,
    configuration: config::GrpcConfiguration,
}

impl ResolverExtension for Grpc {
    fn new(schemas: Vec<SubgraphSchema<'_>>, config: Configuration) -> Result<Self, Error> {
        let mut services = Vec::new();
        let mut messages = Vec::new();
        let mut enums = Vec::new();

        let configuration = config.deserialize()?;

        for schema in schemas {
            for directive in schema.directives() {
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
        }

        Ok(Grpc {
            schema: schema::Schema::new(services, messages, enums)?,
            configuration,
        })
    }

    fn resolve(&mut self, prepared: &[u8], _headers: SubgraphHeaders, variables: Variables) -> Result<Response, Error> {
        let field = ResolvedField::try_from(prepared)?;
        resolvers::grpc_method(field, variables, &self.schema, &self.configuration)
    }

    fn resolve_subscription<'s>(
        &'s mut self,
        prepared: &'s [u8],
        _headers: SubgraphHeaders,
        variables: Variables,
    ) -> Result<impl IntoSubscription<'s>, Error> {
        let field = ResolvedField::try_from(prepared)?;
        resolvers::grpc_method_subscription(field, variables, &self.schema, &self.configuration)
    }
}
