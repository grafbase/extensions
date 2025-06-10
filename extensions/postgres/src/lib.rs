mod config;
mod context;
mod introspect;
mod logger;
mod resolve;

use std::{collections::HashMap, time::Duration};

use config::PostgresConfig;
use context::Context;
use grafbase_database_definition::DatabaseDefinition;
use grafbase_sdk::{
    ResolverExtension,
    host_io::postgres,
    types::{Configuration, Error, ResolvedField, Response, SubgraphHeaders, SubgraphSchema, Variables},
};

#[derive(ResolverExtension)]
struct PostgresExtension {
    // from database name to pool
    pools: HashMap<String, postgres::Pool>,
    // from subgraph name to definition
    database_definitions: HashMap<String, DatabaseDefinition>,
}

impl ResolverExtension for PostgresExtension {
    fn new(subgraph_schemas: Vec<SubgraphSchema<'_>>, config: Configuration) -> Result<Self, Error> {
        logger::init();

        let mut pools = HashMap::new();
        let config: PostgresConfig = config.deserialize()?;

        for database in config.databases {
            let pool = create_pool(&database)?;
            pools.insert(database.name, pool);
        }

        let database_definitions = introspect::from_sdl(subgraph_schemas);

        Ok(Self {
            pools,
            database_definitions,
        })
    }

    fn resolve(&mut self, prepared: &[u8], _headers: SubgraphHeaders, variables: Variables) -> Result<Response, Error> {
        let field = ResolvedField::try_from(prepared)?;
        let Some(database_definition) = self.database_definitions.get(field.subgraph_name()) else {
            return Err(format!("Subgraph {} is not a Postgres subgraph", field.subgraph_name()).into());
        };

        let Some(pool) = self.pools.get(database_definition.name()) else {
            return Err(format!("Database {} is not configured", database_definition.name()).into());
        };

        let Some(operation) = database_definition.get_operation(field.definition_id()) else {
            return Err("operation not found".into());
        };

        let ctx = Context {
            operation,
            variables: &variables,
            database_definition,
            pool,
            field: field.as_ref(),
        };

        Ok(resolve::execute(ctx).into())
    }
}

fn create_pool(database: &config::DatabaseConfig) -> Result<postgres::Pool, Error> {
    let pool = match database.pool {
        Some(ref pool_config) => {
            let mut pool_opts = postgres::PoolOptions::new();

            if let Some(max_connections) = pool_config.max_connections {
                pool_opts = pool_opts.max_connections(max_connections);
            }

            if let Some(min_connections) = pool_config.min_connections {
                pool_opts = pool_opts.min_connections(min_connections);
            }

            if let Some(idle_timeout_ms) = pool_config.idle_timeout_ms {
                pool_opts = pool_opts.idle_timeout(Duration::from_millis(idle_timeout_ms));
            }

            if let Some(acquire_timeout_ms) = pool_config.acquire_timeout_ms {
                pool_opts = pool_opts.acquire_timeout(Duration::from_millis(acquire_timeout_ms));
            }

            if let Some(max_lifetime_ms) = pool_config.max_lifetime_ms {
                pool_opts = pool_opts.max_lifetime(Duration::from_millis(max_lifetime_ms));
            }

            postgres::Pool::connect_with_options(&database.name, &database.url, pool_opts)?
        }
        None => postgres::Pool::connect(&database.name, &database.url)?,
    };

    Ok(pool)
}
