mod create_many;
mod create_one;
mod delete_many;
mod delete_one;
mod find_many;
mod find_one;
mod introspection;
mod lookup_many;
mod update_many;
mod update_one;

use std::{cell::RefCell, fmt::Display, path::Path, sync::Arc};

use grafbase_postgres_introspection::config::Config;
use grafbase_sdk::{
    host_io::http::Url,
    test::{DynamicSchema, DynamicSubgraph, TestConfig, TestRunner},
};
use indoc::formatdoc;
use names::{Generator, Name};
use sqlx::PgPool;
use tokio::sync::OnceCell;

thread_local! {
    static NAMES: RefCell<Option<Generator<'static>>> = const { RefCell::new(None) };
}

pub async fn admin_pool() -> &'static PgPool {
    // this is for creating/dropping databases, which _should not be done_ over pgbouncer.
    static ADMIN_CONNECTION_STRING: &str = "postgres://postgres:grafbase@localhost:5432/postgres";
    static POOL: OnceCell<PgPool> = OnceCell::const_new();

    POOL.get_or_init(|| async { PgPool::connect(ADMIN_CONNECTION_STRING).await.unwrap() })
        .await
}

pub async fn admin_mtls_pool() -> &'static PgPool {
    // this is for creating/dropping databases, which _should not be done_ over pgbouncer.
    static MTLS_ADMIN_CONNECTION_STRING: &str = concat!(
        "postgresql://testuser@localhost:5433/postgres?",
        "sslmode=verify-full&",
        "sslrootcert=../../docker/postgres-mtls/certs/ca.crt&",
        "sslcert=../../docker/postgres-mtls/certs/client.crt&",
        "sslkey=../../docker/postgres-mtls/certs/client.key",
    );

    static POOL: OnceCell<PgPool> = OnceCell::const_new();

    POOL.get_or_init(|| async { PgPool::connect(MTLS_ADMIN_CONNECTION_STRING).await.unwrap() })
        .await
}

fn random_name() -> String {
    NAMES.with(|maybe_generator| {
        maybe_generator
            .borrow_mut()
            .get_or_insert_with(|| Generator::with_naming(Name::Plain))
            .next()
            .unwrap()
            .replace('-', "")
    })
}

// url for the engine for introspecting, querying and mutating the database.
static BASE_CONNECTION_STRING: &str = "postgres://postgres:grafbase@localhost:5432/";

static MTLS_BASE_CONNECTION_STRING: &str = concat!(
    "postgresql://testuser@localhost:5433/?",
    "sslmode=verify-full&",
    "sslrootcert=../../docker/postgres-mtls/certs/ca.crt&",
    "sslcert=../../docker/postgres-mtls/certs/client.crt&",
    "sslkey=../../docker/postgres-mtls/certs/client.key",
);

struct Inner {
    pool: PgPool,
    config: String,
    subgraphs: Vec<DynamicSubgraph>,
}

#[derive(Clone)]
struct PgTestApi {
    inner: Arc<Inner>,
}

impl PgTestApi {
    async fn new<F, U>(config: impl Display, init: F) -> Self
    where
        F: FnOnce(PgTestApi) -> U,
        U: Future<Output = ()>,
    {
        Self::new_with_subgraphs(config, Vec::new(), init).await
    }

    async fn new_mtls<F, U>(config: impl Display, init: F) -> Self
    where
        F: FnOnce(PgTestApi) -> U,
        U: Future<Output = ()>,
    {
        Self::new_mtls_with_subgraphs(config, Vec::new(), init).await
    }

    async fn new_with_subgraphs<F, U>(config: impl Display, subgraphs: Vec<DynamicSubgraph>, init: F) -> Self
    where
        F: FnOnce(PgTestApi) -> U,
        U: Future<Output = ()>,
    {
        let database_name = random_name();
        let admin = admin_pool().await;

        sqlx::query(&format!("DROP DATABASE IF EXISTS {database_name}"))
            .execute(admin)
            .await
            .unwrap();

        sqlx::query(&format!("CREATE DATABASE {database_name}"))
            .execute(admin)
            .await
            .unwrap();

        let mut url = Url::parse(BASE_CONNECTION_STRING).unwrap();
        url.set_path(&database_name);

        Self::new_with_connection_string(config, subgraphs, url.as_ref(), init).await
    }

    async fn new_mtls_with_subgraphs<F, U>(config: impl Display, subgraphs: Vec<DynamicSubgraph>, init: F) -> Self
    where
        F: FnOnce(PgTestApi) -> U,
        U: Future<Output = ()>,
    {
        let database_name = random_name();
        let admin = admin_mtls_pool().await;

        sqlx::query(&format!("DROP DATABASE IF EXISTS {database_name}"))
            .execute(admin)
            .await
            .unwrap();

        sqlx::query(&format!("CREATE DATABASE {database_name}"))
            .execute(admin)
            .await
            .unwrap();

        let mut url = Url::parse(MTLS_BASE_CONNECTION_STRING).unwrap();
        url.set_path(&database_name);

        Self::new_with_connection_string(config, subgraphs, url.as_ref(), init).await
    }

    async fn new_with_connection_string<F, U>(
        config: impl Display,
        subgraphs: Vec<DynamicSubgraph>,
        database_url: &str,
        init: F,
    ) -> Self
    where
        F: FnOnce(PgTestApi) -> U,
        U: Future<Output = ()>,
    {
        let config = formatdoc! {r#"
            [graph]
            introspection = true

            [[extensions.postgres.config.databases]]
            name = "default"
            default_schema = "public"
            url = "{database_url}"

            {config}
        "#};

        let pool = PgPool::connect(database_url).await.unwrap();

        let inner = Arc::new(Inner {
            pool,
            config,
            subgraphs,
        });

        let this = Self { inner };

        init(this.clone()).await;

        this
    }

    async fn runner_spawn(&self) -> TestRunner {
        let extension_path = std::env::current_dir().unwrap().join("build");
        let schema = self.introspect_local_extension(&extension_path).await;

        let schema = DynamicSchema::builder(schema)
            .into_extension_only_subgraph("test", &extension_path)
            .unwrap();

        let mut config = TestConfig::builder().with_subgraph(schema);

        for subgraph in &self.inner.subgraphs {
            config = config.with_subgraph(subgraph.clone());
        }

        if std::env::var("PREBUILT_EXTENSION").is_ok() {
            config = config.with_extension("./build");
        }

        let config = config
            .enable_networking()
            .enable_stderr()
            .enable_stdout()
            .enable_environment_variables()
            .log_level(grafbase_sdk::test::LogLevel::EngineDebug)
            .build(&self.inner.config)
            .unwrap();

        TestRunner::new(config).await.unwrap()
    }

    async fn runner_spawn_with_config(&self, toml_config: &str) -> TestRunner {
        let extension_path = std::env::current_dir().unwrap().join("build");
        let extension_url = format!("file://{}", extension_path.display());
        let toml_config = format!("extension_url = \"{extension_url}\"\n\n{toml_config}");

        let config = toml::from_str(&toml_config).unwrap();
        let schema = self.introspect_inner(config).await;

        let schema = DynamicSchema::builder(schema)
            .into_extension_only_subgraph("test", &extension_path)
            .unwrap();

        let mut config = TestConfig::builder().with_subgraph(schema);

        for subgraph in &self.inner.subgraphs {
            config = config.with_subgraph(subgraph.clone());
        }

        if std::env::var("PREBUILT_EXTENSION").is_ok() {
            config = config.with_extension("./build");
        }

        let config = config
            .enable_networking()
            .enable_stderr()
            .enable_stdout()
            .enable_environment_variables()
            .log_level(grafbase_sdk::test::LogLevel::EngineDebug)
            .build(&self.inner.config)
            .unwrap();

        TestRunner::new(config).await.unwrap()
    }

    async fn execute_sql(&self, sql: &str) {
        sqlx::query(sql).execute(&self.inner.pool).await.unwrap();
    }

    async fn introspect_with_config(&self, toml_str: &str) -> String {
        let config = toml::from_str(toml_str).unwrap();
        self.introspect_inner(config).await
    }

    async fn introspect(&self) -> String {
        self.introspect_inner(Config {
            database_name: String::from("default"),
            extension_url: String::from("https://grafbase.com/extensions/postgres/0.1.1"),
            default_schema: String::from("public"),
            schemas: Default::default(),
            enable_mutations: true,
            enable_queries: true,
            schema_allowlist: Vec::new(),
            schema_denylist: Vec::new(),
        })
        .await
    }

    async fn introspect_local_extension(&self, extension_path: &Path) -> String {
        let extension_url = format!("file://{}", extension_path.display());

        self.introspect_inner(Config {
            database_name: String::from("default"),
            extension_url,
            default_schema: String::from("public"),
            schemas: Default::default(),
            enable_mutations: true,
            enable_queries: true,
            schema_allowlist: Vec::new(),
            schema_denylist: Vec::new(),
        })
        .await
    }

    async fn introspect_inner(&self, config: Config) -> String {
        let mut conn = self.inner.pool.acquire().await.unwrap();

        grafbase_postgres_introspection::introspect(&mut conn, config)
            .await
            .unwrap()
    }

    async fn introspect_error(&self, toml_str: &str) -> String {
        let config = toml::from_str(toml_str).unwrap();
        let mut conn = self.inner.pool.acquire().await.unwrap();

        grafbase_postgres_introspection::introspect(&mut conn, config)
            .await
            .unwrap_err()
            .to_string()
    }
}
