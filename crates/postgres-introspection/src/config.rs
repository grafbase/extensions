use grafbase_database_definition::TableWalker;
use serde::Deserialize;
use std::collections::HashMap;

/// Returns the default value for enable_queries configuration.
fn default_enable_queries() -> bool {
    true
}

/// Returns the default value for enable_mutations configuration.
fn default_enable_mutations() -> bool {
    true
}

/// Represents the overall configuration for the application.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Determines whether mutations (write operations) are enabled for this configuration.
    #[serde(default = "default_enable_mutations")]
    pub enable_mutations: bool,
    /// Determines whether queries (read operations) are enabled for this configuration.
    #[serde(default = "default_enable_queries")]
    pub enable_queries: bool,
    /// The name of the database this virtual subgraph uses (the name from grafbase.toml).
    #[serde(default = "default_database_name")]
    pub database_name: String,
    /// The default schema name to use, e.g. objects in this schema will not have an
    /// explicit mention of the schema in their SDL counterparts.
    #[serde(default = "default_default_schema")]
    pub default_schema: String,
    /// The URL of the extension printed in the SDL file.
    pub extension_url: String,
    /// Configuration details for each schema within the database, keyed by schema name.
    #[serde(default)]
    pub schemas: HashMap<String, SchemaConfig>,
}

impl Config {
    /// Determines whether mutations (write operations) are allowed for the specified table.
    pub fn mutations_allowed(&self, table: TableWalker<'_>) -> bool {
        if !table.mutations_allowed() {
            return false;
        }

        let Some(schema_config) = self.schemas.get(table.schema()) else {
            return self.enable_mutations;
        };

        let Some(table_config) = schema_config.tables.get(table.database_name()) else {
            return schema_config.enable_mutations;
        };

        table_config.enable_mutations
    }

    /// Determines whether queries (read operations) are allowed for the specified table.
    pub fn queries_allowed(&self, table: TableWalker<'_>) -> bool {
        let Some(schema_config) = self.schemas.get(table.schema()) else {
            return self.enable_queries;
        };

        if table.relation_kind().is_view() {
            let Some(view_config) = schema_config.views.get(table.database_name()) else {
                return schema_config.enable_queries;
            };

            view_config.enable_queries
        } else {
            let Some(table_config) = schema_config.tables.get(table.database_name()) else {
                return schema_config.enable_queries;
            };

            table_config.enable_queries
        }
    }
}

/// Returns the default database name.
fn default_database_name() -> String {
    "default".to_string()
}

/// Returns the default schema name.
fn default_default_schema() -> String {
    "public".to_string()
}

/// Represents the overrides for a database schema.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SchemaConfig {
    /// Determines whether mutations (write operations) are enabled for this schema.
    #[serde(default = "default_enable_mutations")]
    pub enable_mutations: bool,
    /// Determines whether queries (read operations) are enabled for this schema.
    #[serde(default = "default_enable_queries")]
    pub enable_queries: bool,
    /// Configuration details for each view within the database, keyed by view name.
    #[serde(default)]
    pub views: HashMap<String, ViewConfig>,
    /// Configuration overrides for each table within the schema, keyed by table name.
    #[serde(default)]
    pub tables: HashMap<String, TableConfig>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TableConfig {
    /// Determines whether mutations (write operations) are enabled for this table.
    #[serde(default = "default_enable_mutations")]
    pub enable_mutations: bool,
    /// Determines whether queries (read operations) are enabled for this table.
    #[serde(default = "default_enable_queries")]
    pub enable_queries: bool,
    /// Configuration details for relationships originating from this view, keyed by relationship name.
    #[serde(default)]
    pub relations: HashMap<String, RelationConfig>,
}

/// Represents the configuration settings for a specific database relation (e.g., a view).
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ViewConfig {
    /// Determines whether mutations (write operations) are enabled for this table.
    #[serde(default = "default_enable_mutations")]
    pub enable_mutations: bool,
    /// Determines whether queries (read operations) are enabled for this table.
    #[serde(default = "default_enable_queries")]
    pub enable_queries: bool,
    /// Optional list of unique key constraints, where each constraint is a list of column names.
    pub unique_keys: Option<Vec<Vec<String>>>,
    /// Configuration details for each column within the relation, keyed by column name.
    #[serde(default)]
    pub columns: HashMap<String, ColumnConfig>,
    /// Configuration details for relationships originating from this view, keyed by relationship name.
    #[serde(default)]
    pub relations: HashMap<String, RelationConfig>,
}

/// Represents the configuration for a specific column within a view.
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct ColumnConfig {
    /// Specifies whether the column can contain null values. Defaults to `true`.
    #[serde(default = "default_nullable")]
    pub nullable: bool,
    /// Specifies whether the column values must be unique. Defaults to `false`.
    #[serde(default = "default_unique")]
    pub unique: bool,
    /// An optional new name for the column. If `None`, the original name is used.
    pub rename: Option<String>,
    /// An optional description for the column.
    pub description: Option<String>,
}

/// Represents the configuration for a relationship defined within a view.
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct RelationConfig {
    /// The name of the schema containing the referenced table. Defaults to the `public`.
    #[serde(default = "default_default_schema")]
    pub referenced_schema: String,
    /// The name of the table (or view) that this relationship points to.
    pub referenced_table: String,
    /// The list of column names in the current view that form the foreign key.
    pub referencing_columns: Vec<String>,
    /// The list of column names in the referenced table that form the primary/unique key.
    pub referenced_columns: Vec<String>,
}

// Helper function for default nullable value
fn default_nullable() -> bool {
    true
}

// Helper function for default unique value
fn default_unique() -> bool {
    false
}
