use clap::Parser;
use std::fs;

mod args;
mod cargo_toml;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let mut found_extensions = false;
    let mut test_arguments = vec!["nextest".to_string(), "run".to_string()];

    for entry in fs::read_dir("./extensions")? {
        let Ok(entry) = entry else {
            continue;
        };

        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let Ok(cargo_toml) = fs::read_to_string(path.join("Cargo.toml")) else {
            continue;
        };

        let cargo_toml = cargo_toml::parse(&cargo_toml)?;

        let Some(sdk_version) = cargo_toml.grafbase_sdk_version() else {
            continue;
        };

        if !sdk_version.matches(&args.sdk_version) {
            continue;
        }

        test_arguments.push("-p".to_string());
        test_arguments.push(cargo_toml.name().to_string());
        found_extensions = true;
    }

    if !found_extensions {
        return Ok(());
    }

    duct::cmd("cargo", &test_arguments).run()?;

    Ok(())
}
