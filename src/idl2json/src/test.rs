#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
use crate::{
    candid_types::internal_candid_type_to_idl_type, idl2json, idl2json_with_weak_names,
    BytesFormat, Idl2JsonOptions, JsonValue,
};
use candid::{
    parser::{
        types::{IDLType, PrimType, TypeField},
        value::IDLValue,
    },
    types::internal::Label,
    CandidType, Decode, Deserialize, IDLArgs,
};
use serde::Serialize;
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
    let json_value: JsonValue = idl2json(&idl_value.args[0], &Idl2JsonOptions::default());

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

/// A test type: The Rust equivalent of the eponymous type in the sample did file.
#[derive(CandidType, Serialize, Deserialize)]
struct InternetIdentityInit {
    pub assigned_user_number_range: Option<(u64, u64)>,
    pub archive_module_hash: Option<[u8; 32]>,
    pub canister_creation_cycles_cost: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize)]
struct ChangedInternetIdentityInit {
    pub assigned_user_number_range: Option<(u64, u64)>,
    // Type change
    pub archive_module_hash: u32,
    // Removed
    // pub canister_creation_cycles_cost: Option<u64>,
    // Added
    pub new_field: Vec<String>,
}

struct BinaryTestVector {
    /// Some binary IDL.
    pub binary: Vec<u8>,
    /// Formatting options and how it should appear with untyped, typed and changed representations.
    pub json_options: Vec<(Idl2JsonOptions, String, String, String)>,
}

/// A test vector for the test type.
fn test_vector() -> BinaryTestVector {
    BinaryTestVector {
    binary: vec![
    68, 73, 68, 76, 5, 110, 1, 108, 2, 196, 136, 191, 215, 1, 2, 247, 245, 203, 251, 7, 4, 110,
    3, 109, 123, 110, 120, 1, 0, 1, 1, 32, 246, 145, 242, 105, 221, 102, 170, 79, 196, 78, 105,
    22, 174, 254, 224, 59, 183, 254, 184, 33, 174, 244, 52, 103, 82, 105, 116, 244, 112, 205,
    75, 7, 1, 0, 16, 165, 212, 232, 0, 0, 0,
    ],  json_options: vec![
    ( Idl2JsonOptions{ bytes_as: Some(BytesFormat::Numbers), long_bytes_as: None, ..Idl2JsonOptions::default() },
        r#"[{
            "2_138_241_783":["1000000000000"],
            "451_920_964":[[246,145,242,105,221,102,170,79,196,78,105,22,174,254,224,59,183,254,184,33,174,244,52,103,82,105,116,244,112,205,75,7]]
        }]"#.to_string(),
        r#"[{
            "archive_module_hash":[[246,145,242,105,221,102,170,79,196,78,105,22,174,254,224,59,183,254,184,33,174,244,52,103,82,105,116,244,112,205,75,7]],
            "canister_creation_cycles_cost":["1000000000000"]
        }]"#.to_string(),
        r#"[{
            "2_138_241_783":["1000000000000"],
            "archive_module_hash":[[246,145,242,105,221,102,170,79,196,78,105,22,174,254,224,59,183,254,184,33,174,244,52,103,82,105,116,244,112,205,75,7]]
        }]"#.to_string()
    ),
    ( Idl2JsonOptions{ bytes_as: Some(BytesFormat::Hex), long_bytes_as: None, ..Idl2JsonOptions::default() },
        r#"[{
            "2_138_241_783":["1000000000000"],
            "451_920_964":["f691f269dd66aa4fc44e6916aefee03bb7feb821aef43467526974f470cd4b07"]
        }]"#.to_string(),
        r#"[
            {"archive_module_hash":["f691f269dd66aa4fc44e6916aefee03bb7feb821aef43467526974f470cd4b07"],
            "canister_creation_cycles_cost":["1000000000000"]
        }]"#.to_string(),
        r#"[{
            "2_138_241_783":["1000000000000"],
            "archive_module_hash":["f691f269dd66aa4fc44e6916aefee03bb7feb821aef43467526974f470cd4b07"]
        }]"#.to_string()
    ),
    ( Idl2JsonOptions{ bytes_as: Some(BytesFormat::Sha256), long_bytes_as: None, ..Idl2JsonOptions::default() },
        r#"[{
            "2_138_241_783":["1000000000000"],
            "451_920_964":["Bytes with sha256: ac0c88f389e4af11790089d940f8483905e8766de960ccd847d0500b4caf6acf"]}]"#.to_string(),
        r#"[{
            "archive_module_hash":["Bytes with sha256: ac0c88f389e4af11790089d940f8483905e8766de960ccd847d0500b4caf6acf"],
            "canister_creation_cycles_cost":["1000000000000"]
        }]"#.to_string(),
        r#"[{
            "2_138_241_783":["1000000000000"],
            "archive_module_hash":["Bytes with sha256: ac0c88f389e4af11790089d940f8483905e8766de960ccd847d0500b4caf6acf"]
        }]"#.to_string()
    ),
        ( Idl2JsonOptions{ bytes_as: Some(BytesFormat::Sha256), long_bytes_as: Some((1000, BytesFormat::Hex)), ..Idl2JsonOptions::default() },
        r#"[
            {"2_138_241_783":["1000000000000"],
            "451_920_964":["Bytes with sha256: ac0c88f389e4af11790089d940f8483905e8766de960ccd847d0500b4caf6acf"]
        }]"#.to_string(),
        r#"[
            {"archive_module_hash":["Bytes with sha256: ac0c88f389e4af11790089d940f8483905e8766de960ccd847d0500b4caf6acf"],
            "canister_creation_cycles_cost":["1000000000000"]
         }]"#.to_string(),
        r#"[{
            "2_138_241_783":["1000000000000"],
            "archive_module_hash":["Bytes with sha256: ac0c88f389e4af11790089d940f8483905e8766de960ccd847d0500b4caf6acf"]
        }]"#.to_string()
        ),
         ( Idl2JsonOptions{ bytes_as: Some(BytesFormat::Sha256), long_bytes_as: Some((5, BytesFormat::Hex)), ..Idl2JsonOptions::default() },
         r#"[{
            "2_138_241_783":["1000000000000"],
            "451_920_964":["f691f269dd66aa4fc44e6916aefee03bb7feb821aef43467526974f470cd4b07"]
        }]"#.to_string(),
            r#"[{
                "archive_module_hash":["f691f269dd66aa4fc44e6916aefee03bb7feb821aef43467526974f470cd4b07"],
                "canister_creation_cycles_cost":["1000000000000"]
            }]"#.to_string(),
            r#"[{
                "2_138_241_783":["1000000000000"],
                "archive_module_hash":["f691f269dd66aa4fc44e6916aefee03bb7feb821aef43467526974f470cd4b07"]
            }]"#.to_string()
        ),
         ]}
}

/// The expected IDLType of the test type
fn test_idl_type() -> IDLType {
    IDLType::OptT(Box::new(IDLType::RecordT(vec![
        TypeField {
            label: Label::Named("archive_module_hash".to_string()),
            typ: IDLType::OptT(Box::new(IDLType::VecT(Box::new(IDLType::PrimT(
                PrimType::Nat8,
            ))))),
        },
        TypeField {
            label: Label::Named("assigned_user_number_range".to_string()),
            typ: IDLType::OptT(Box::new(IDLType::RecordT(vec![
                TypeField {
                    label: Label::Unnamed(0),
                    typ: IDLType::PrimT(PrimType::Nat64),
                },
                TypeField {
                    label: Label::Unnamed(1),
                    typ: IDLType::PrimT(PrimType::Nat64),
                },
            ]))),
        },
        TypeField {
            label: Label::Named("canister_creation_cycles_cost".to_string()),
            typ: IDLType::OptT(Box::new(IDLType::PrimT(PrimType::Nat64))),
        },
    ])))
}

/// Verifies that the buffer is parsed to the expected JSON using the provided IDLType.
#[test]
fn sample_binaries_are_parsed_with_idl_type() {
    // The inputs:
    // .. At the time of writing, this type is `InternetIdentityInit` from `internet_identity.did`.
    let idl_type = test_idl_type();
    let BinaryTestVector {
        binary,
        json_options,
    } = test_vector();
    for (options, _, expected_json_string, _) in &json_options {
        let expected_json: JsonValue =
            serde_json::from_str(expected_json_string).expect("Invalid JSON in test");
        // Let the conversion begin
        let idl_value = Decode!(&binary[..], IDLValue).expect("Failed to parse buffer");
        let json_value: JsonValue = idl2json_with_weak_names(&idl_value, &idl_type, options);
        assert_eq!(expected_json, json_value);
    }
}

/// Verifies that the buffer is parsed to the expected JSON using the derived IDLType.
#[test]
fn sample_binaries_are_parsed_with_derived_idl_type() {
    // The inputs:
    // .. At the time of writing, this type is `InternetIdentityInit` from `internet_identity.did`.
    let idl_type = internal_candid_type_to_idl_type(&InternetIdentityInit::ty());
    let idl_type = IDLType::OptT(Box::new(idl_type));
    let BinaryTestVector {
        binary,
        json_options,
    } = test_vector();
    for (options, _, expected_json_string, _) in &json_options {
        let expected_json: JsonValue =
            serde_json::from_str(expected_json_string).expect("Invalid JSON in test");
        // Let the conversion begin
        let idl_value = Decode!(&binary[..], IDLValue).expect("Failed to parse buffer");
        let json_value: JsonValue = idl2json_with_weak_names(&idl_value, &idl_type, options);
        assert_eq!(expected_json, json_value);
    }
}

/// Verifies that the buffer is parsed to the expected JSON if no type is provided.
#[test]
fn sample_binaries_are_parsed_without_type() {
    // The inputs:
    let BinaryTestVector {
        binary,
        json_options,
    } = test_vector();
    for (options, expected_json_string, _, _) in &json_options {
        let expected_json: JsonValue =
            serde_json::from_str(expected_json_string).expect("Invalid JSON in test");
        // Let the conversion begin
        let idl_value = Decode!(&binary[..], IDLValue).expect("Failed to parse buffer");
        let json_value: JsonValue = idl2json(&idl_value, options);
        if expected_json != json_value {
            panic!(
                "Mismatched JSON:\nExpected: {expected_json_string}\nGot:      {}",
                serde_json::to_string(&json_value).expect("Failed to stringify JSON")
            );
        }
    }
}

/// Verifies that the buffer is parsed to the expected JSON using the derived IDLType.
#[test]
fn sample_binaries_are_parsed_with_changed_idl_type() {
    // The inputs:
    // .. At the time of writing, this type is `InternetIdentityInit` from `internet_identity.did`.
    let idl_type = internal_candid_type_to_idl_type(&ChangedInternetIdentityInit::ty());
    let idl_type = IDLType::OptT(Box::new(idl_type));
    let BinaryTestVector {
        binary,
        json_options,
    } = test_vector();
    for (options, _, _, expected_json_string) in &json_options {
        let expected_json: JsonValue =
            serde_json::from_str(expected_json_string).expect("Invalid JSON in test");
        // Let the conversion begin
        let idl_value = Decode!(&binary[..], IDLValue).expect("Failed to parse buffer");
        let json_value: JsonValue = idl2json_with_weak_names(&idl_value, &idl_type, options);
        if expected_json != json_value {
            panic!(
                "Mismatched JSON:\nExpected: {expected_json_string}\nGot:      {}",
                serde_json::to_string(&json_value).expect("Failed to stringify JSON")
            );
        }
    }
}

/// Verifies that every type is represented in JSON as expected
#[test]
fn types_should_be_represented_correctly() {
    struct TestVector {
        typ: IDLType,
        val: IDLValue,
        json: &'static str,
    }
    let test_vectors = vec![
        TestVector {
            typ: IDLType::PrimT(PrimType::Bool),
            val: IDLValue::Bool(true),
            json: "true",
        },
        TestVector {
            typ: IDLType::PrimT(PrimType::Null),
            val: IDLValue::Null,
            json: "null",
        },
        TestVector {
            typ: IDLType::PrimT(PrimType::Text),
            val: IDLValue::Text("Hi there".to_string()),
            json: r#""Hi there""#,
        },
        TestVector {
            typ: IDLType::PrimT(PrimType::Float32),
            val: IDLValue::Float32(91.0),
            json: r#"91.0"#,
        },
        TestVector {
            typ: IDLType::PrimT(PrimType::Float64),
            val: IDLValue::Float64(91.0),
            json: r#"91.0"#,
        },
    ];
    let options = Idl2JsonOptions::default();
    for TestVector { typ, val, json } in test_vectors {
        {
            let actual_json =
                serde_json::to_string(&idl2json(&val, &options)).expect("Failed to serialize JSON");
            assert_eq!(
                json, actual_json,
                "Failed to get expected representation for type: {typ:?}"
            );
        }
        {
            let actual_json =
                serde_json::to_string(&idl2json_with_weak_names(&val, &typ, &options))
                    .expect("Failed to serialize JSON");
            assert_eq!(
                json, actual_json,
                "Failed to get expected representation for type, when weak names are used: {typ:?}"
            );
        }
    }
}
