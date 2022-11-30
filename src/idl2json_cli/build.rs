use toml::{self, Value as TomlValue};

fn main() {
    let cargo_lock =
        std::fs::read_to_string("../../Cargo.lock").expect("Failed to read Cargo.lock");
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
