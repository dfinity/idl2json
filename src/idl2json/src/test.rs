use crate::{idl2json, JsonValue};
use candid::IDLArgs;
use std::fs;

/// Returns the absolute path to a file in the samples directory.
macro_rules! sample_file {
    ($file:ident) => {
        format!("{}/../../samples/{}", env!("CARGO_MANIFEST_DIR"), $file)
    };
}

fn idl_is_parsed_as_expected(idl_filename: &str, json_filename: &str) {
    let expected_json_string: String =
        fs::read_to_string(sample_file!(json_filename)).expect("Could not read sample JSON");
    let expected_json_value: JsonValue =
        serde_json::from_str(&expected_json_string).expect("Could not parse sample JSON");

    let idl_string: String =
        fs::read_to_string(sample_file!(idl_filename)).expect("Could not read sample IDL");
    let idl_value: IDLArgs = idl_string.parse::<IDLArgs>().expect("Malformed input");
    let json_value: JsonValue = idl2json(&idl_value.args[0]);

    let diff = json_patch::diff(&expected_json_value, &json_value);
    if !(diff.0).is_empty() {
        panic!(
            "Unexpected changes in {idl_filename} conversion:\n{:?}",
            diff
        );
    }
}

#[test]
fn sample_idls_are_parsed_as_expected() {
    idl_is_parsed_as_expected("proposal.idl", "proposal.json");
    idl_is_parsed_as_expected("all_types.idl", "all_types.json");
}
