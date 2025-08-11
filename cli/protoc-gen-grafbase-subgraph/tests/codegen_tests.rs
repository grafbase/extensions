#![allow(clippy::panic)]

use std::{collections::BTreeMap, fs, path::Path, process};

/// Parse a test file containing multiple proto files marked with //!file: comments
fn parse_test_file(content: &str) -> BTreeMap<String, String> {
    let mut files = BTreeMap::new();
    let mut current_file = None;
    let mut current_content = String::new();

    for line in content.lines() {
        if let Some(filename) = line.strip_prefix("//!file:") {
            // Save previous file if any
            if let Some(file) = current_file.take() {
                files.insert(file, current_content.trim().to_string());
                current_content.clear();
            }
            current_file = Some(filename.trim().to_string());
        } else if current_file.is_some() {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Save last file
    if let Some(file) = current_file {
        files.insert(file, current_content.trim().to_string());
    }

    files
}

/// Write parsed proto files to a temporary directory
fn setup_proto_files(tmp_dir: &Path, files: &BTreeMap<String, String>) -> Vec<String> {
    let mut main_protos = Vec::new();

    for (filename, content) in files {
        let file_path = tmp_dir.join(filename);

        // Create parent directories if needed
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::write(&file_path, content).unwrap();

        // Track main proto files (not in subdirectories like grafbase/options.proto)
        if !filename.contains('/') || !filename.starts_with("grafbase/") {
            main_protos.push(filename.clone());
        }
    }

    main_protos
}

#[test]
fn codegen_tests() {
    let bin_path = env!("CARGO_BIN_EXE_protoc-gen-grafbase-subgraph");

    if cfg!(windows) {
        eprintln!("Skipping these tests on Windows, since there is no official protoc binary release.");
        return;
    }

    insta::glob!("testcases/*.proto", |test_path| {
        let snapshot_path = test_path.parent().unwrap();
        let test_name = test_path.file_stem().unwrap().to_str().unwrap();

        insta::with_settings!({
            snapshot_path => snapshot_path.to_str().unwrap(),
            prepend_module_to_snapshot => false,
            snapshot_suffix => "",
        }, {
            let test_content = fs::read_to_string(test_path).unwrap();
            let mut files = parse_test_file(&test_content);

            // If no //!file: markers, treat as single file for backwards compatibility
            if files.is_empty() {
                files.insert(
                    test_path.file_name().unwrap().to_str().unwrap().to_string(),
                    test_content,
                );
            }

            // Always insert the options.proto from the source of truth
            // This will override any inline options.proto in test files
            let options_proto_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("proto")
                .join("grafbase")
                .join("options.proto");
            let options_content = fs::read_to_string(options_proto_path).unwrap();
            files.insert("grafbase/options.proto".to_string(), options_content);

            let proto_tmp = tempfile::tempdir().unwrap();
            let main_protos = setup_proto_files(proto_tmp.path(), &files);

            let output_tmp = tempfile::tempdir().unwrap();

            for main_proto in main_protos {
                let mut cmd = process::Command::new("protoc");

                cmd.arg("--plugin")
                    .arg(bin_path)
                    .arg("--grafbase-subgraph_out")
                    .arg(output_tmp.path())
                    .arg("-I")
                    .arg(proto_tmp.path())
                    .arg(&main_proto)
                    .stderr(process::Stdio::inherit());

                let output = cmd.output().unwrap();

                assert!(
                    output.status.success(),
                    "Expected success for {}, got {}\n{}",
                    main_proto,
                    output.status,
                    std::str::from_utf8(&output.stderr).unwrap(),
                );
            }

            // Collect all generated .graphql files
            let mut graphql_files = BTreeMap::new();
            for entry in walkdir::WalkDir::new(output_tmp.path()) {
                let entry = entry.unwrap();

                if !entry.file_type().is_file() {
                    continue;
                }

                if let Some("graphql") = entry.path().extension().and_then(|s| s.to_str()) {
                    let filename = entry.file_name().to_str().unwrap().to_string();
                    let content = fs::read_to_string(entry.path()).unwrap();
                    graphql_files.insert(filename, content);
                }
            }

            // Debug: print what files were generated
            if test_name.contains("multi") {
                eprintln!("Generated files for {}: {:?}", test_name, graphql_files.keys().collect::<Vec<_>>());
            }

            // Concatenate all files in deterministic order
            let mut combined_output = String::new();
            for (filename, content) in graphql_files {
                if !combined_output.is_empty() {
                    combined_output.push_str("\n#--- ");
                    combined_output.push_str(&filename);
                    combined_output.push_str(" ---#\n\n");
                } else {
                    combined_output.push_str("#--- ");
                    combined_output.push_str(&filename);
                    combined_output.push_str(" ---#\n\n");
                }
                combined_output.push_str(&content);
            }

            insta::assert_snapshot!(test_name, combined_output);
        });
    });
}
