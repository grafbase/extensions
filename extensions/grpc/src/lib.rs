mod convert_input;
mod convert_output;
mod directives;

use grafbase_sdk::{
    ResolverExtension, Subscription,
    types::{
        Configuration, Error, FieldDefinitionDirective, FieldInputs, FieldOutputs, SchemaDirective, SubgraphHeaders,
    },
};
use std::collections::HashMap;

#[derive(ResolverExtension)]
struct Grpc {
    services: HashMap<String, directives::ProtoService>,
    messages: HashMap<String, directives::ProtoMessage>,
}

impl ResolverExtension for Grpc {
    fn new(schema_directives: Vec<SchemaDirective>, config: Configuration) -> Result<Self, Error> {
        let mut services = HashMap::new();
        let mut messages = HashMap::new();

        for directive in schema_directives {
            match directive.name() {
                "protoMessages" => {
                    let directives::ProtoMessages {
                        messages: message_definitions,
                    } = directive.arguments()?;

                    messages.extend(
                        message_definitions
                            .into_iter()
                            .map(|message| (message.name.clone(), message)),
                    );
                }
                "protoServices" => {
                    let directives::ProtoServices {
                        services: service_definitions,
                    } = directive.arguments()?;

                    services.extend(
                        service_definitions
                            .into_iter()
                            .map(|service| (service.name.clone(), service)),
                    );
                }
                other => unreachable!("Unknown directive: {other}"),
            }
        }

        Ok(Grpc { services, messages })
    }

    fn resolve_field(
        &mut self,
        _headers: SubgraphHeaders,
        _subgraph_name: &str,
        directive: FieldDefinitionDirective<'_>,
        inputs: FieldInputs,
    ) -> Result<FieldOutputs, Error> {
        let directive_arguments: directives::GrpcMethod =
            directive.deserialize_arguments_seed(convert_input::GrpcMethodArgumentsDeserialize { schema: self })?;

        let input_protos: Vec<serde_json::Value> = inputs.deserialize()?;

        // let input_protos = inputs
        //     .into_iter()
        //     .map(|input| {
        //         dbg!(&input);
        //         input.transcode(crate::convert_input::MessageSerializer(input_type))
        //     })
        //     .collect::<Result<Vec<_>, _>>()?;

        dbg!(&input_protos);

        let outputs = FieldOutputs::new(
            inputs,
            serde_json::json!({
                "name": "hi", "location": {"latitude": 1, "longitude": -1}
            }),
        )?;

        Ok(outputs)
    }

    fn resolve_subscription(
        &mut self,
        _headers: SubgraphHeaders,
        _subgraph_name: &str,
        directive: FieldDefinitionDirective<'_>,
    ) -> Result<Box<dyn Subscription>, Error> {
        todo!("resolve_subscription()")
    }
}
