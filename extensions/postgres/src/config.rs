#[derive(Debug, Clone, serde::Deserialize)]
pub struct PostgresConfig {
    pub databases: Vec<DatabaseConfig>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct DatabaseConfig {
    pub name: String,
    pub url: String,
    pub pool: Option<PoolConfig>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            url: "postgres://localhost:5432/postgres".to_string(),
            pool: None,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct PoolConfig {
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub idle_timeout_ms: Option<u64>,
    pub acquire_timeout_ms: Option<u64>,
    pub max_lifetime_ms: Option<u64>,
}
