use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use argh::FromArgs;

mod cargo_toml;

#[derive(FromArgs)]
/// Test matrix runner for Grafbase extensions
struct Args {
    /// skip specific extensions by name (comma-delimited list)
    #[argh(option)]
    skip: Option<String>,

    /// run in verbose mode
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// number of test threads to use (passed to nextest)
    #[argh(option, short = 'j')]
    test_threads: Option<usize>,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let skip_extensions: HashSet<String> = args
        .skip
        .map(|s| s.split(',').map(|ext| ext.trim().to_string()).collect())
        .unwrap_or_default();

    if args.verbose {
        eprintln!("Test matrix runner started");
        if !skip_extensions.is_empty() {
            eprintln!(
                "Skipping extensions: {}",
                skip_extensions.iter().cloned().collect::<Vec<_>>().join(", ")
            );
        }
    }

    let extensions_dir = Path::new("./extensions");
    let extensions = discover_extensions(extensions_dir, &skip_extensions)?;

    if extensions.is_empty() {
        println!("No extensions found to build and test.");
        return Ok(());
    }

    println!("Found {} extension(s) to process", extensions.len());

    for extension in &extensions {
        build_extension(extension, args.verbose)?;
    }

    run_tests(&extensions, args.verbose, args.test_threads)?;

    println!("\n✓ All done!");
    Ok(())
}

struct Extension {
    name: String,
    path: PathBuf,
}

fn discover_extensions(extensions_dir: &Path, skip_set: &HashSet<String>) -> Result<Vec<Extension>> {
    let mut extensions = Vec::new();

    let entries = fs::read_dir(extensions_dir).with_context(|| {
        format!(
            "Failed to read extensions directory: {path}",
            path = extensions_dir.display()
        )
    })?;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let cargo_toml_path = path.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            continue;
        }

        let cargo_toml_content = fs::read_to_string(&cargo_toml_path)
            .with_context(|| format!("Failed to read {path}", path = cargo_toml_path.display()))?;

        let cargo_toml = cargo_toml::parse(&cargo_toml_content)
            .with_context(|| format!("Failed to parse {path}", path = cargo_toml_path.display()))?;

        let name = cargo_toml.name().to_string();

        if skip_set.contains(&name) {
            eprintln!("  Skipping extension: {name}");
            continue;
        }

        extensions.push(Extension { name, path });
    }

    Ok(extensions)
}

fn build_extension(extension: &Extension, verbose: bool) -> Result<()> {
    println!("** Building {name} **", name = extension.name);

    let mut cmd = duct::cmd("grafbase", ["extension", "build", "--debug"]).dir(&extension.path);

    if !verbose {
        cmd = cmd.stdout_null().stderr_null();
    }

    cmd.run().with_context(|| {
        format!(
            "Failed to build extension {name} (is grafbase in your path?)",
            name = extension.name
        )
    })?;

    Ok(())
}

fn run_tests(extensions: &[Extension], verbose: bool, test_threads: Option<usize>) -> Result<()> {
    if extensions.is_empty() {
        println!("No extensions to test.");
        return Ok(());
    }

    println!("\n** Running tests for {} extension(s) **", extensions.len());

    let mut test_arguments = vec!["nextest".to_string(), "run".to_string(), "--profile=ci".to_string()];

    if let Some(threads) = test_threads {
        test_arguments.push("-j".to_string());
        test_arguments.push(threads.to_string());
    }

    for extension in extensions {
        test_arguments.push("-p".to_string());
        test_arguments.push(extension.name.clone());
    }

    let mut cmd = duct::cmd("cargo", &test_arguments).env("PREBUILT_EXTENSION", "1");

    if !verbose {
        cmd = cmd.stdout_null();
    }

    cmd.run().context("Failed to run tests")?;

    Ok(())
}
