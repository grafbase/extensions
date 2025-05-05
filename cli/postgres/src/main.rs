use anyhow::Context;
use args::IntrospectCommand;
use sqlx::{Connection, PgConnection};

mod args;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

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
    let config = std::fs::read_to_string(&cmd.config).context("could not read the configuration file")?;
    let config = toml::from_str(&config)?;
    let sdl = grafbase_postgres_introspection::introspect(conn, config).await?;

    println!("{sdl}");

    Ok(())
}
