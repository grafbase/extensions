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
    SdkError, SelectionSetResolverExtension,
    host_io::postgres,
    types::{ArgumentValues, Configuration, Data, Error, Field, SubgraphHeaders, SubgraphSchema},
};

#[derive(SelectionSetResolverExtension)]
struct PostgresExtension {
    // from database name to pool
    pools: HashMap<String, postgres::Pool>,
    // from subgraph name to definition
    database_definitions: HashMap<String, DatabaseDefinition>,
}

impl SelectionSetResolverExtension for PostgresExtension {
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

    fn prepare(&mut self, _: &str, field: Field<'_>) -> Result<Vec<u8>, Error> {
        Ok(field.into_bytes())
    }

    fn resolve(
        &mut self,
        _: SubgraphHeaders,
        subgraph_name: &str,
        prepared: &[u8],
        arguments: ArgumentValues<'_>,
    ) -> Result<Data, Error> {
        let Some(database_definition) = self.database_definitions.get(subgraph_name) else {
            return Err(Error::new(format!(
                "Subgraph {subgraph_name} is not a Postgres subgraph"
            )));
        };

        let Some(pool) = self.pools.get(database_definition.name()) else {
            return Err(Error::new(format!(
                "Database {} is not configured",
                database_definition.name()
            )));
        };

        let data = Field::with_bytes(prepared, |field| {
            let Some(operation) = database_definition.get_operation(field.definition_id()) else {
                return Err(SdkError::from("operation not found"));
            };

            let ctx = Context {
                operation,
                arguments,
                database_definition,
                pool,
                field,
            };

            resolve::execute(ctx)
        })??;

        Ok(data)
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
