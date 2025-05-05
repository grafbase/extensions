use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "grafbase-postgres")]
#[command(about = "Grafbase Postgres Extension")]
pub struct Args {
    /// Connection string to the database
    #[arg(short, long, env = "DATABASE_URL")]
    pub database_url: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Introspect a PostgreSQL database
    #[command(name = "introspect")]
    Introspect(IntrospectCommand),
}

#[derive(Parser, Debug)]
pub struct IntrospectCommand {
    /// Configuration file location
    #[arg(short, long, default_value = "./grafbase-postgres.toml")]
    pub config: PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
