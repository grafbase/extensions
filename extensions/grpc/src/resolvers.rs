mod streaming_response;

use crate::{
    config::{self, Service},
    conversions,
    directives::{self, ProtoMethodDefinition},
    schema,
};
use grafbase_sdk::{
    Subscription,
    types::{Error, FieldDefinitionDirective, FieldInputs, FieldOutputs},
};
use streaming_response::StreamingResponse;

pub(crate) fn grpc_method(
    directive: FieldDefinitionDirective<'_>,
    inputs: FieldInputs,
    schema: &schema::Schema,
    configuration: &config::GrpcConfiguration,
) -> Result<FieldOutputs, Error> {
    let MethodInfo {
        input_message,
        output_message,
        service,
        method,
    } = extract_method_info(&directive, schema, configuration)?;

    let mut input_proto = Vec::new();

    directive.arguments_seed(conversions::GrpcMethodDirectiveArguments {
        schema,
        message: input_message,
        out: &mut input_proto,
    })?;

    let client = grafbase_sdk::host_io::grpc::GrpcClient::new(&service.address)?;

    let metadata = &[];

    match client.unary(&input_proto, &service.name, &method.name, metadata, None) {
        Ok(response) => FieldOutputs::new(
            inputs,
            conversions::MessageSerialize::new(&response.into_message().into(), output_message, schema),
        )
        .map_err(Error::from),
        Err(err) => Err(Error::new(format!(
            "gRPC error. Status code: {:?}. Message: {}",
            err.code(),
            err.message()
        ))),
    }
}

pub(crate) fn grpc_method_subscription<'a>(
    directive: FieldDefinitionDirective<'_>,
    schema: &'a schema::Schema,
    configuration: &'a config::GrpcConfiguration,
) -> Result<Box<dyn Subscription + 'a>, Error> {
    let MethodInfo {
        input_message,
        output_message,
        service,
        method,
    } = extract_method_info(&directive, schema, configuration)?;

    let mut input_proto = Vec::new();

    directive.arguments_seed(conversions::GrpcMethodDirectiveArguments {
        schema,
        message: input_message,
        out: &mut input_proto,
    })?;

    let client = grafbase_sdk::host_io::grpc::GrpcClient::new(&service.address)?;

    let metadata = &[];

    match client.streaming(&input_proto, &service.name, &method.name, metadata, None) {
        Ok(response) => Ok(Box::new(StreamingResponse {
            response,
            output_message,
            schema,
        })),
        Err(_) => todo!(),
    }
}

struct MethodInfo<'a> {
    input_message: &'a schema::Message,
    output_message: &'a schema::Message,
    service: &'a Service,
    method: &'a ProtoMethodDefinition,
}

fn extract_method_info<'a>(
    directive: &FieldDefinitionDirective<'_>,
    schema: &'a schema::Schema,
    configuration: &'a config::GrpcConfiguration,
) -> Result<MethodInfo<'a>, Error> {
    let grpc_method_directive: directives::GrpcMethod = directive.arguments()?;

    let Some(service) = schema.get_service(&grpc_method_directive.service) else {
        return Err(Error::new(format!(
            "Service not found: {}",
            grpc_method_directive.service
        )));
    };

    let Some(method) = service.get_method(&grpc_method_directive.method) else {
        return Err(Error::new(format!(
            "Method not found: {}",
            grpc_method_directive.method
        )));
    };

    let Some(service) = configuration.services.iter().find(|s| s.name == service.name) else {
        return Err(Error::new(format!("Service not found: {}", service.name)));
    };

    let Some(input_message) = schema.get_message(&method.input_type) else {
        return Err(Error::new(format!("Message not found: {}", method.input_type)));
    };

    let Some(output_message) = schema.get_message(&method.output_type) else {
        return Err(Error::new(format!("Message not found: {}", method.output_type)));
    };

    Ok(MethodInfo {
        service,
        method,
        input_message,
        output_message,
    })
}
