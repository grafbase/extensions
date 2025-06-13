mod streaming_response;

use crate::{
    config::{self, Service},
    conversions::{self, ArgumentsDeserialize},
    directives::{self, ProtoMethodDefinition},
    schema,
};
use grafbase_sdk::types::{Error, ResolvedField, Response, Variables};
use streaming_response::StreamingResponse;

pub(crate) fn grpc_method(
    field: ResolvedField<'_>,
    variables: Variables,
    schema: &schema::Schema,
    configuration: &config::GrpcConfiguration,
) -> Result<Response, Error> {
    let MethodInfo {
        input_message,
        output_message,
        service,
        method,
    } = extract_method_info(&field, schema, configuration)?;

    let mut input_proto = Vec::new();

    field.arguments_seed(
        &variables,
        ArgumentsDeserialize {
            schema,
            message: input_message,
            out: &mut input_proto,
        },
    )?;

    let client = grafbase_sdk::host_io::grpc::GrpcClient::new(&service.address)?;

    let metadata = &[];

    match client.unary(&input_proto, &service.name, &method.name, metadata, None) {
        Ok(response) => Ok(Response::data(conversions::MessageSerialize::new(
            &response.into_message().into(),
            output_message,
            schema,
        ))),
        Err(err) => Err(Error::new(format!(
            "gRPC error. Status code: {:?}. Message: {}",
            err.code(),
            err.message()
        ))),
    }
}

pub(crate) fn grpc_method_subscription<'a>(
    field: ResolvedField<'a>,
    variables: Variables,
    schema: &'a schema::Schema,
    configuration: &'a config::GrpcConfiguration,
) -> Result<StreamingResponse<'a>, Error> {
    let MethodInfo {
        input_message,
        output_message,
        service,
        method,
    } = extract_method_info(&field, schema, configuration)?;

    let mut input_proto = Vec::new();
    field.arguments_seed(
        &variables,
        ArgumentsDeserialize {
            schema,
            message: input_message,
            out: &mut input_proto,
        },
    )?;

    let client = grafbase_sdk::host_io::grpc::GrpcClient::new(&service.address)?;

    let metadata = &[];

    match client.streaming(&input_proto, &service.name, &method.name, metadata, None) {
        Ok(response) => Ok(StreamingResponse {
            response,
            output_message,
            schema,
        }),
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
    field: &ResolvedField<'_>,
    schema: &'a schema::Schema,
    configuration: &'a config::GrpcConfiguration,
) -> Result<MethodInfo<'a>, Error> {
    let grpc_method_directive: directives::GrpcMethod = field.directive().arguments()?;

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
