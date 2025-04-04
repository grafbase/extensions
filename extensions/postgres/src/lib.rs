mod config;
mod database_definition;

use std::{collections::HashMap, time::Duration};

use config::PostgresConfig;
use database_definition::DatabaseDefinition;
use grafbase_sdk::{
    SelectionSetResolverExtension,
    host_io::postgres,
    types::{Configuration, Error, SubgraphHeaders, SubgraphSchema},
};

#[derive(SelectionSetResolverExtension)]
struct PostgresExtension {
    pools: HashMap<String, postgres::Pool>,
    subgraph_schemas: HashMap<String, DatabaseDefinition>,
}

impl SelectionSetResolverExtension for PostgresExtension {
    fn new(subgraph_schemas: Vec<SubgraphSchema<'_>>, config: Configuration) -> Result<Self, Error> {
        let mut pools = HashMap::new();
        let config: PostgresConfig = config.deserialize()?;

        for database in config.databases {
            let pool = create_pool(&database)?;
            pools.insert(database.name, pool);
        }

        Ok(Self {
            pools,
            subgraph_schemas: HashMap::new(),
        })
    }

    fn prepare(&mut self, subgraph_name: &str, field: grafbase_sdk::types::Field<'_>) -> Result<Vec<u8>, Error> {
        todo!()
    }

    fn resolve(
        &mut self,
        headers: SubgraphHeaders,
        subgraph_name: &str,
        prepared: &[u8],
        arguments: grafbase_sdk::types::ArgumentValues<'_>,
    ) -> Result<grafbase_sdk::types::Data, Error> {
        todo!()
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
