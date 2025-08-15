mod auth;
mod config;
mod directives;
mod statements;

use self::config::{Authentication, SnowflakeConfig};
use grafbase_sdk::{
    ResolverExtension,
    types::{AuthorizedOperationContext, Configuration, Error, ResolvedField, Response, SubgraphHeaders, SubgraphSchema, Variables},
};
use template::Template;

#[derive(ResolverExtension)]
struct Snowflake {
    jwt: String,
    config: SnowflakeConfig,
}

impl ResolverExtension for Snowflake {
    fn new(_schemas: Vec<SubgraphSchema>, config: Configuration) -> Result<Self, Error> {
        let config: SnowflakeConfig = config.deserialize()?;

        Ok(Self {
            jwt: auth::generate_jwt(&config),
            config,
        })
    }

    fn resolve(&mut self, _ctx: &AuthorizedOperationContext, prepared: &[u8], _headers: SubgraphHeaders, variables: Variables) -> Result<Response, Error> {
        let field = ResolvedField::try_from(prepared)?;
        let arguments: serde_json::Value = field.arguments(&variables)?;
        let ctx = serde_json::json!({
            "args": arguments,
        });

        match field.directive().name() {
            "snowflakeQuery" => {
                let directives::SnowflakeQueryDirective { sql, bindings } = field.directive().arguments()?;

                let bindings = bindings
                    .map(|binding| {
                        let t = Template::new(binding)
                            .map_err(|err| Error::new(format!("Failed to parse bindings: {err}")))?;
                        let json = t.render_json(&ctx);
                        serde_json::from_str(&json)
                            .map_err(|err| Error::new(format!("Failed to parse bindings: {err}")))
                    })
                    .transpose()?
                    .unwrap_or(vec![]);

                let response = self.execute_statement(&sql, &bindings)?;

                let Some(data) = response.data else {
                    return Err(Error::new(format!(
                        "No data returned from Snowflake query. SQL State: {}, Code: {}. Message: {}",
                        response.sql_state, response.code, response.message
                    )));
                };

                Ok(Response::data(data))
            }
            other => Err(Error::new(format!("Unsupported directive \"{other}\""))),
        }
    }
}
