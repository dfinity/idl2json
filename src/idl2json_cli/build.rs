use toml::{self, Value as TomlValue};

fn main() {
    // The Cargo.lock file can be in any of the following locations:
    // * The current directory, when built by `cargo install`.
    // * The grandparent directory, when built by `cargo build`.
    // * The great-grandparent directory, when publishing to crates.io.
    // * ... and probably many others that I haven't encountered yet.
    let cargo_lock = (0..6)
        .find_map(|depth| {
            std::fs::read_to_string(format!("{}Cargo.lock", "../".repeat(depth))).ok()
        })
        .expect("Failed to read Cargo.lock in idl2json_cli build.rs");
    let cargo_lock: TomlValue =
        toml::from_str(&cargo_lock).expect("Failed to parse Cargo.lock as toml");
    let cargo_lock = if let TomlValue::Array(entries) = &cargo_lock["package"] {
        entries
    } else {
        panic!("Malformed cargo lock file");
    };
    let cargo_lock = cargo_lock
        .iter()
        .find(|entry| entry["name"] == TomlValue::String("candid".to_string()))
        .expect("Could not find candid");
    let version = if let TomlValue::String(version) = &cargo_lock["version"] {
        version
    } else {
        panic!("Candid version not specified in Cargo.lock")
    };
    println!("cargo:rustc-env=CARGO_CANDID_VERSION={version}");
}
