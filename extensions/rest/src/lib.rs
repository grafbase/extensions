mod types;

use grafbase_sdk::{
    ResolverExtension,
    host_io::http::{self, HttpRequest, Url},
    jq_selection::JqSelection,
    types::{Configuration, Error, ResolvedField, Response, SubgraphHeaders, SubgraphSchema, Variables},
};
use serde_json::Value;
use template::Templates;
use types::{Rest, RestEndpoint};

use crate::types::{Body, BodyCase};

#[derive(ResolverExtension)]
struct RestExtension {
    endpoints: Vec<RestEndpoint>,
    templates: Templates,
    jq_selection: JqSelection,
}

impl ResolverExtension for RestExtension {
    fn new(schemas: Vec<SubgraphSchema<'_>>, _config: Configuration) -> Result<Self, Error> {
        let mut endpoints = Vec::<RestEndpoint>::new();

        endpoints.sort_by(|a, b| {
            let by_name = a.args.name.cmp(&b.args.name);
            let by_subgraph = a.subgraph_name.cmp(&b.subgraph_name);
            by_name.then(by_subgraph)
        });

        for schema in schemas {
            for directive in schema.directives() {
                let endpoint = RestEndpoint {
                    subgraph_name: schema.subgraph_name().to_string(),
                    args: directive.arguments()?,
                };

                endpoints.push(endpoint);
            }
        }

        Ok(Self {
            endpoints,
            templates: Default::default(),
            jq_selection: JqSelection::default(),
        })
    }

    fn resolve(&mut self, prepared: &[u8], headers: SubgraphHeaders, variables: Variables) -> Result<Response, Error> {
        let field = ResolvedField::try_from(prepared)?;

        let rest: Rest<'_> = field
            .directive()
            .arguments()
            .map_err(|e| format!("Could not parse directive arguments: {e}"))?;
        let field_arguments: serde_json::Value = field.arguments(&variables)?;
        let ctx = serde_json::json!({"args": field_arguments});

        let Some(endpoint) = self.get_endpoint(rest.endpoint, field.subgraph_name()) else {
            return Err(format!("Endpoint not found: {}", rest.endpoint).into());
        };

        let mut url = Url::parse(&endpoint.args.base_url).map_err(|e| format!("Could not parse URL: {e}"))?;

        let path = self.templates.get_or_insert(rest.path)?.render_url(&ctx);
        let path = path.strip_prefix("/").unwrap_or(&path);

        if !path.is_empty() {
            let mut path_segments = url.path_segments_mut().map_err(|_| "Could not parse URL")?;

            path_segments.push(path);
        }

        let url = url.join(path).map_err(|e| format!("Could not parse URL path: {e}"))?;

        let mut builder = HttpRequest::builder(url, rest.method.into());

        for (key, value) in headers.iter() {
            builder.push_header(key.to_string(), value.to_str().unwrap().to_string());
        }

        let request = if let Some(body) = rest.body {
            builder.json(self.render_body(body, ctx)?)
        } else {
            builder.build()
        };

        let resp = http::execute(&request).map_err(|e| format!("HTTP request failed: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("HTTP request failed with status: {}", resp.status()).into());
        }

        if let Some(selection) = rest.selection {
            let data: serde_json::Value = resp.json().map_err(|e| format!("Error deserializing response: {e}"))?;

            if !(data.is_object() || data.is_array()) {
                return Ok(Response::data(data));
            }

            let data = self.render_jq_template(selection, data)?;

            Ok(Response::data(data))
        } else {
            Ok(Response::json(resp.into_bytes()))
        }
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

    fn render_body(&mut self, body: Body<'_>, ctx: Value) -> Result<Value, Error> {
        match body.into_case() {
            Some(BodyCase::Selection(source)) => self.render_jq_template(source, ctx),
            Some(BodyCase::Static(value)) => Ok(value),
            None => Ok(Value::Null),
        }
    }

    fn render_jq_template(&mut self, source: &str, ctx: Value) -> Result<Value, Error> {
        let selection = self.templates.get_or_insert(source)?.render_json(&ctx);
        let mut values = self
            .jq_selection
            .select(&selection, ctx)
            .map_err(|e| format!("Failed to filter with selection: {}", e))?
            .collect::<Result<Vec<Value>, _>>()
            .map_err(|e| format!("Failed to collect filtered value: {}", e))?;
        if values.len() == 1 {
            Ok(values.pop().unwrap())
        } else {
            Ok(serde_json::Value::Array(values))
        }
    }
}
