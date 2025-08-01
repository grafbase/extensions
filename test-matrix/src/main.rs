use std::fs;

mod cargo_toml;

fn main() -> anyhow::Result<()> {
    let mut found_extensions = false;
    let mut test_arguments = vec!["nextest".to_string(), "run".to_string(), "--profile=ci".to_string()];

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

        println!("\n** {} **", cargo_toml.name());
        duct::cmd("grafbase", ["extension", "build", "--scratch-dir", "../../target"])
            .dir(path)
            .run()
            .map_err(|e| anyhow::anyhow!("Failed to build extension (is grafbase in your path?): {}", e))?;

        test_arguments.push("-p".to_string());
        test_arguments.push(cargo_toml.name().to_string());

        found_extensions = true;
    }

    if !found_extensions {
        return Ok(());
    }

    println!("\n** Running tests **");
    duct::cmd("cargo", &test_arguments)
        .env("PREBUILT_EXTENSION", "1")
        .run()?;

    Ok(())
}
