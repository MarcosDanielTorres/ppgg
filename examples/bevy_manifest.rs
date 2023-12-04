use std::env;
use std::path::PathBuf;

use toml_edit::Document;

fn main() {
    // so maps only takes options
    let x = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .map(|mut path| {
            path.push("Cargo.toml");
            // ../playground/Cargo.toml
            if !path.exists() {
                panic!(
                    "No Cargo manifest found for crate. Expected: {}",
                    path.display()
                );
            }
            let manifest = std::fs::read_to_string(path.clone())
                .unwrap_or_else(|_| panic!("Unable to read cargo manifest: {}", path.display()));
            manifest
                .parse::<Document>()
                .unwrap_or_else(|_| panic!("Failed to parse cargo manifest: {}", path.display()))
        })
        .expect("CARGO_MANIFEST_DIR is not defined.");

    println!("{:?}", x);
}

// probably should try to use serde here to get the contents of the Document to a file
// (serialization) to create a clone of a `Cargo.toml`
