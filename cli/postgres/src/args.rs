use std::path::PathBuf;

use clap::{ArgGroup, Parser, Subcommand};
use semver::Version;
use url::Url;

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
#[command(group(
    ArgGroup::new("extension_identifier")
        .required(true)
        .args(["extension_url", "extension_version"]),
))]
pub struct IntrospectCommand {
    /// Output file path. If not provided, the SDL will be printed to stdout.
    #[arg(short, long)]
    pub output_file: Option<PathBuf>,
    /// The name of the database to be used in the GraphQL SDL
    #[arg(short, long, default_value = "default")]
    pub database_name: String,
    /// Default schema to be used in the GraphQL SDL (will be omitted from definitions)
    #[arg(short = 's', long, default_value = "public")]
    pub default_schema: String,
    /// URL to the extension
    #[arg(long, short = 'u')]
    pub extension_url: Option<Url>,
    /// Extension version following semver
    #[arg(long, short = 'v')]
    pub extension_version: Option<Version>,
}

impl IntrospectCommand {
    pub fn extension_url(&self) -> String {
        match self.extension_version.as_ref() {
            Some(version) => format!("https://grafbase.com/extensions/postgres/{version}"),
            None => self.extension_url.as_ref().unwrap().to_string(),
        }
    }
}

pub fn parse() -> Args {
    Args::parse()
}
