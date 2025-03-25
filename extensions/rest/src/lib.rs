mod types;

use grafbase_sdk::{
    ResolverExtension,
    host_io::http::{self, HttpRequest, Url},
    jq_selection::JqSelection,
    types::{
        Configuration, Error, FieldDefinitionDirective, FieldInputs, FieldOutputs, SchemaDirective, SubgraphHeaders,
    },
};
use types::{Rest, RestEndpoint};

#[derive(ResolverExtension)]
struct RestExtension {
    endpoints: Vec<RestEndpoint>,
    jq_selection: JqSelection,
}

impl ResolverExtension for RestExtension {
    fn new(schema_directives: Vec<SchemaDirective>, _: Configuration) -> Result<Self, Error> {
        let mut endpoints = Vec::<RestEndpoint>::new();

        for directive in schema_directives {
            let endpoint = RestEndpoint {
                subgraph_name: directive.subgraph_name().to_string(),
                args: directive.arguments()?,
            };

            endpoints.push(endpoint);
        }

        endpoints.sort_by(|a, b| {
            let by_name = a.args.name.cmp(&b.args.name);
            let by_subgraph = a.subgraph_name.cmp(&b.subgraph_name);
            by_name.then(by_subgraph)
        });

        Ok(Self {
            endpoints,
            jq_selection: JqSelection::default(),
        })
    }

    fn resolve_field(
        &mut self,
        headers: SubgraphHeaders,
        subgraph_name: &str,
        directive: FieldDefinitionDirective<'_>,
        inputs: FieldInputs,
    ) -> Result<FieldOutputs, Error> {
        let rest: Rest<'_> = directive
            .arguments()
            .map_err(|e| format!("Could not parse directive arguments: {e}"))?;

        let Some(endpoint) = self.get_endpoint(rest.endpoint, subgraph_name) else {
            return Err(format!("Endpoint not found: {}", rest.endpoint).into());
        };

        let mut url = Url::parse(&endpoint.args.base_url).map_err(|e| format!("Could not parse URL: {e}"))?;

        let path = rest.path.strip_prefix("/").unwrap_or(rest.path);

        if !path.is_empty() {
            let mut path_segments = url.path_segments_mut().map_err(|_| "Could not parse URL")?;

            path_segments.push(path);
        }

        let url = url.join(path).map_err(|e| format!("Could not parse URL path: {e}"))?;

        let mut builder = HttpRequest::builder(url, rest.method.into());

        for (key, value) in headers.iter() {
            builder.push_header(key.to_string(), value.to_str().unwrap().to_string());
        }

        let request = match rest.body() {
            Some(ref body) => builder.json(body),
            None => builder.build(),
        };

        let result = http::execute(&request).map_err(|e| format!("HTTP request failed: {e}"))?;

        if !result.status().is_success() {
            return Err(format!("HTTP request failed with status: {}", result.status()).into());
        }

        let data: serde_json::Value = result
            .json()
            .map_err(|e| format!("Error deserializing response: {e}"))?;

        if !(data.is_object() || data.is_array()) {
            return Ok(FieldOutputs::new(inputs, data)?);
        }

        let filtered = self
            .jq_selection
            .select(rest.selection, data)
            .map_err(|e| format!("Error selecting result value: {e}"))?
            .collect::<Result<Vec<_>, _>>();

        Ok(match filtered {
            Ok(filtered) => {
                // TODO: We don't know whether a list of a single item is expected here...
                // Need engine to help
                if filtered.len() == 1 {
                    FieldOutputs::new(inputs, filtered.into_iter().next().unwrap())?
                } else {
                    FieldOutputs::new(inputs, filtered)?
                }
            }
            Err(error) => FieldOutputs::error(inputs, format!("Failed to filter with selection: {}", error)),
        })
    }
}

impl RestExtension {
    pub fn get_endpoint(&self, name: &str, subgraph_name: &str) -> Option<&RestEndpoint> {
        self.endpoints
            .binary_search_by(|e| {
                let by_name = e.args.name.as_str().cmp(name);
                let by_subgraph = e.subgraph_name.as_str().cmp(subgraph_name);

                by_name.then(by_subgraph)
            })
            .map(|i| &self.endpoints[i])
            .ok()
    }
}
