mod display_utils;
mod options_proto;
mod render_graphql_sdl;
mod schema;
mod translate_schema;

use protobuf::plugin::{CodeGeneratorRequest, CodeGeneratorResponse, code_generator_response::File};
use protobuf::{CodedOutputStream, Enum, Message};
use render_graphql_sdl::{render_graphql_sdl, render_graphql_sdl_filtered};
use std::{
    env,
    io::{self, Read as _, Write as _},
    process,
};
use translate_schema::translate_schema;

fn bail(error: String) -> CodeGeneratorResponse {
    let mut response = CodeGeneratorResponse::new();
    response.error = Some(error);
    response
}

fn generate(raw_request: &[u8]) -> CodeGeneratorResponse {
    let request = match CodeGeneratorRequest::parse_from_bytes(raw_request) {
        Ok(request) => request,
        Err(decode_err) => {
            return bail(format!(
                "Failed to decode CodeGeneratorRequest from stdin: {decode_err}",
            ));
        }
    };

    let translated_schema = translate_schema(request);

    let mut response = CodeGeneratorResponse::new();

    response.set_supported_features(
        protobuf::plugin::code_generator_response::Feature::FEATURE_PROTO3_OPTIONAL.value() as u64,
    );

    if translated_schema.services.is_empty() {
        return response;
    }

    // Check if we're in multi-file mode (any service has subgraph_name)
    let is_multi_file_mode = translated_schema
        .services
        .iter()
        .any(|service| service.subgraph_name.is_some());

    if is_multi_file_mode {
        // Multi-file mode: generate one file per subgraph
        use std::collections::HashMap;

        // Group services by subgraph name
        let mut services_by_subgraph: HashMap<String, Vec<schema::ProtoServiceId>> = HashMap::new();

        for service in translated_schema.iter_services() {
            if let Some(subgraph_name) = &service.subgraph_name {
                // Validate subgraph name
                if !is_valid_subgraph_name(subgraph_name) {
                    return bail(format!(
                        "Invalid subgraph name '{}' for service '{}'. Subgraph names must match [a-zA-Z][a-zA-Z0-9-]*",
                        subgraph_name, service.name
                    ));
                }
                services_by_subgraph
                    .entry(subgraph_name.to_string())
                    .or_default()
                    .push(service.id);
            }
            // Services without subgraph_name are ignored in multi-file mode
        }

        // Generate a file for each subgraph
        for (subgraph_name, service_ids) in services_by_subgraph {
            let mut graphql_schema = String::new();
            render_graphql_sdl_filtered(&translated_schema, Some(&service_ids), &mut graphql_schema)
                .expect("Failed to render GraphQL schema");

            let mut file = File::new();
            file.set_name(format!("{}.graphql", subgraph_name));
            file.set_content(graphql_schema);
            response.file.push(file);
        }
    } else {
        // Single-file mode: generate schema.graphql
        let mut graphql_schema = String::new();
        render_graphql_sdl(&translated_schema, &mut graphql_schema).expect("Failed to render GraphQL schema");

        let mut file = File::new();
        file.set_name("schema.graphql".to_owned());
        file.set_content(graphql_schema);
        response.file.push(file);
    }

    response
}

fn is_valid_subgraph_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut chars = name.chars();

    // First character must be a letter
    if let Some(first) = chars.next()
        && !first.is_ascii_alphabetic()
    {
        return false;
    }

    // Remaining characters must be letters, digits, or hyphens
    chars.all(|c| c.is_ascii_alphanumeric() || c == '-')
}

fn main() -> io::Result<()> {
    if env::args().any(|x| x == "--version") {
        println!(env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    let response = generate(&buf);

    let mut output_buf = Vec::new();
    {
        let mut output_stream = CodedOutputStream::vec(&mut output_buf);
        response.write_to(&mut output_stream).expect("error encoding response");
        output_stream.flush().expect("error flushing response");
    }

    io::stdout().write_all(&output_buf)?;

    Ok(())
}
