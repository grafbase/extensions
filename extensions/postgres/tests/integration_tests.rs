mod create_many;
mod create_one;
mod delete_many;
mod delete_one;
mod find_one;
mod introspection;

use std::{cell::RefCell, fmt::Display, path::Path, sync::Arc};

use grafbase_postgres_introspection::IntrospectionOptions;
use grafbase_sdk::test::{DynamicSchema, TestConfig, TestRunner};
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

struct Inner {
    pool: PgPool,
    config: String,
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

        let database_url = format!("{BASE_CONNECTION_STRING}{database_name}");

        let config = formatdoc! {r#"
            [graph]
            introspection = true

            [[extensions.postgres.config.databases]]
            name = "default"
            default_schema = "public"
            url = "{database_url}"

            {config}
        "#};

        let pool = PgPool::connect(&database_url).await.unwrap();
        let inner = Arc::new(Inner { pool, config });
        let this = Self { inner };

        init(this.clone()).await;

        this
    }

    async fn runner_spawn(&self) -> TestRunner {
        let extension_path = std::env::current_dir().unwrap().join("build");
        let schema = self.introspect_local_extension(&extension_path).await;
        // println!("{schema}");
        let schema = DynamicSchema::builder(schema)
            .into_extension_only_subgraph("test", &extension_path)
            .unwrap();

        let config = TestConfig::builder()
            .with_subgraph(schema)
            .enable_networking()
            .enable_stderr()
            .enable_stdout()
            .log_level(grafbase_sdk::test::LogLevel::WasiDebug)
            .build(&self.inner.config)
            .unwrap();

        TestRunner::new(config).await.unwrap()
    }

    async fn execute_sql(&self, sql: &str) {
        sqlx::query(sql).execute(&self.inner.pool).await.unwrap();
    }

    async fn introspect(&self) -> String {
        let mut conn = self.inner.pool.acquire().await.unwrap();

        grafbase_postgres_introspection::introspect(
            &mut conn,
            IntrospectionOptions {
                database_name: "default",
                extension_url: "https://grafbase.com/extensions/postgres/0.1.0",
                default_schema: "public",
            },
        )
        .await
        .unwrap()
    }

    async fn introspect_local_extension(&self, extension_path: &Path) -> String {
        let mut conn = self.inner.pool.acquire().await.unwrap();

        let extension_url = format!("file://{}", extension_path.display());

        grafbase_postgres_introspection::introspect(
            &mut conn,
            IntrospectionOptions {
                database_name: "default",
                extension_url: &extension_url,
                default_schema: "public",
            },
        )
        .await
        .unwrap()
    }
}
