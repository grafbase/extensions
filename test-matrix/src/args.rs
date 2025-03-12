use clap::Parser;
use semver::Version;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// SDK version in semantic versioning format (e.g., "1.2.3")
    #[arg(long, value_parser = parse_version)]
    pub sdk_version: Vec<Version>,
}

fn parse_version(version_str: &str) -> Result<Version, String> {
    Version::parse(version_str).map_err(|e| format!("Invalid version format: {}", e))
}
