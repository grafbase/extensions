use args::IntrospectCommand;
use grafbase_postgres_introspection::IntrospectionOptions;
use sqlx::{Connection, PgConnection};

mod args;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = args::parse();

    let mut conn = PgConnection::connect(&args.database_url).await?;

    match args.command {
        args::Commands::Introspect(introspect_cmd) => {
            introspect(&mut conn, introspect_cmd).await?;
        }
    }

    Ok(())
}

async fn introspect(conn: &mut PgConnection, cmd: IntrospectCommand) -> anyhow::Result<()> {
    let opts = IntrospectionOptions {
        database_name: &cmd.database_name,
        extension_url: &cmd.extension_url(),
        default_schema: &cmd.default_schema,
    };

    let sdl = grafbase_postgres_introspection::introspect(conn, opts).await?;

    match cmd.output_file {
        Some(path) => std::fs::write(path, sdl)?,
        None => println!("{sdl}"),
    }

    Ok(())
}
