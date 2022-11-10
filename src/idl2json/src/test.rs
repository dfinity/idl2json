use crate::{idl_to_serde, JsonValue};
use candid::IDLArgs;
use std::fs;

/// Returns the absolute path to a file in the samples directory.
macro_rules! sample_file {
    ($file:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/", $file)
    };
}

#[test]
fn idl_is_parsed_as_expected() {
    let expected_json_string: String =
        fs::read_to_string(sample_file!("proposal.json")).expect("Could not read sample JSON");
    let expected_json_value: JsonValue =
        serde_json::from_str(&expected_json_string).expect("Could not parse sample JSON");

    let idl_string: String =
        fs::read_to_string(sample_file!("proposal.idl")).expect("Could not read sample IDL");
    let idl_value: IDLArgs = idl_string.parse::<IDLArgs>().expect("Malformed input");
    let json_value: JsonValue = idl_to_serde(&idl_value.args[0]);

    let diff = json_patch::diff(&expected_json_value, &json_value);
    if !(diff.0).is_empty() {
        panic!("Unexpected changes:\n{:?}", diff);
    }
}
