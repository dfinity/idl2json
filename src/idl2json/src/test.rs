use crate::{idl2json, JsonValue};
use candid::{parser::types::IDLType, IDLArgs};
use std::fs;

/// Returns the absolute path to a file in the samples directory.
macro_rules! sample_file {
    ($file:ident) => {
        format!("{}/../../samples/{}", env!("CARGO_MANIFEST_DIR"), $file)
    };
}

/// Verifies that the idl at the given filename is parsed to the JSON at the other filename.
///
/// - Formatting differences are ignored.
/// - No interface definition file (.did file) is employed.
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

/// Checks untyped idl to json conversion test vectors.
#[test]
fn sample_idls_are_parsed_as_expected() {
    idl_is_parsed_as_expected("proposal.idl", "proposal.json");
    idl_is_parsed_as_expected("all_types.idl", "all_types.json");
}

/*
/// Verifies that the buffer is parsed to the expected JSON using the provided .did.
#[test]
fn sample_binaries_are_parsed_with_did() {
    let idl_type = IDLType::OptT(Box::new(IDLType::RecordT(vec![TypeField { label: Named("archive_module_hash"), typ: OptT(VecT(PrimT(Nat8))) }, TypeField { label: Named("assigned_user_number_range"), typ: OptT(RecordT([TypeField { label: Unnamed(0), typ: PrimT(Nat64) }, TypeField { label: Unnamed(1), typ: PrimT(Nat64) }])) }, TypeField { label: Named("canister_creation_cycles_cost"), typ: OptT(PrimT(Nat64)) }])));
    let test_vectors: [(&[i8], &str, &IDLType)] = [
        (&[
            68, 73, 68, 76, 5, 110, 1, 108, 2, 196, 136, 191, 215, 1, 2, 247, 245, 203, 251, 7, 4, 110,
            3, 109, 123, 110, 120, 1, 0, 1, 1, 32, 246, 145, 242, 105, 221, 102, 170, 79, 196, 78, 105,
            22, 174, 254, 224, 59, 183, 254, 184, 33, 174, 244, 52, 103, 82, 105, 116, 244, 112, 205,
            75, 7, 1, 0, 16, 165, 212, 232, 0, 0, 0,
        ], "[{"archive_module_hash":[[246,145,242,105,221,102,170,79,196,78,105,22,174,254,224,59,183,254,184,33,174,244,52,103,82,105,116,244,112,205,75,7]],"canister_creation_cycles_cost":["1000000000000"]}]")
    ];
}
*/
