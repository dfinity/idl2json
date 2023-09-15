//! Tests for idl2json_cli.
#![warn(missing_docs)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use super::{main, Args, BytesFormat};
use anyhow::anyhow;
use std::path::Path;

#[test]
fn simple_conversion_should_be_correct() {
    struct TestVector {
        stdin: &'static str,
        stdout: &'static str,
    }
    let args = Args::default();
    let vectors = [
        TestVector {
            stdin: "()",
            stdout: "",
        },
        TestVector {
            stdin: "( record {} )",
            stdout: "{}",
        },
        TestVector {
            stdin: "( record {}, record {} )",
            stdout: "{}\n{}",
        },
    ];
    for vector in vectors {
        let out = main(&args, vector.stdin)
            .map_err(|e| anyhow!("Failed to parse: {} due to: {e}", vector.stdin))
            .unwrap();
        assert_eq!(vector.stdout, &out)
    }
}

/// Returns the absolute path to a file in the samples directory.
macro_rules! sample_file {
    ($file:literal) => {
        Path::new(&format!(
            "{}/../../samples/{}",
            env!("CARGO_MANIFEST_DIR"),
            $file
        ))
        .to_path_buf()
    };
}
/// Constructs idl2json_cli arguments using a type and a did file in the samples directory.
macro_rules! typed_arg {
    ($did_file:literal, $typ:literal) => {
        Args {
            did: vec![sample_file!($did_file)],
            typ: Some($typ.to_string()),
            init: false,
            bytes_as: Some(BytesFormat::Numbers),
            compact: true,
        }
    };
}

#[test]
fn conversion_with_options_should_be_correct() {
    #[derive(Debug)]
    struct TestVector {
        stdin: &'static str,
        args: Args,
        stdout: &'static str,
    }
    let vectors = [
        // We should be able to use a named type in a did file:
        // On the command line we should see:
        // $ echo "(record{canister_creation_cycles_cost= opt 999;})" | didc encode | didc decode | tee /dev/stderr | idl2json --did samples/internet_identity.did  --typ InternetIdentityInit
        // (record { 2_138_241_783 = opt (999 : int) })
        // {"canister_creation_cycles_cost":["999"]}
        TestVector {
            stdin: "(record { 2_138_241_783 = opt (999 : int) })",
            args: typed_arg!("internet_identity.did", "InternetIdentityInit"),
            stdout: r#"{"canister_creation_cycles_cost":["999"]}"#,
        },
        // If a type name cannot be found, conversion should proceed on a best effort basis.
        // On the command line we should see:
        // $ echo "(record{canister_creation_cycles_cost= opt 999;})" | didc encode | didc decode | tee /dev/stderr | target/debug/idl2json --did samples/internet_identity.did  --typ IInnit
        // (record { 2_138_241_783 = opt (999 : int) })
        // {"2_138_241_783":["999"]}
        TestVector {
            stdin: "(record { 2_138_241_783 = opt (999 : int) })",
            args: typed_arg!("internet_identity.did", "IInnit"),
            stdout: r#"{"2_138_241_783":["999"]}"#,
        },
        // We should be able to specify a type literally.
        // On the command line we should see:
        // echo "(record{canister_creation_cycles_cost= opt 999;})" | didc encode | didc decode | tee /dev/stderr | target/debug/idl2json --did samples/internet_identity.did  --typ 'record { canister_creation_cycles_cost: nat32; }'
        // (record { 2_138_241_783 = opt (999 : int) })
        // {"canister_creation_cycles_cost":["911"]}
        TestVector {
            stdin: "(record { 2_138_241_783 = opt (911 : int) })",
            args: Args {
                typ: Some("record { canister_creation_cycles_cost: nat32; }".to_string()),
                compact: true,
                ..Args::default()
            },
            stdout: r#"{"canister_creation_cycles_cost":["911"]}"#,
        },
        // We should be able to specify IDLTypes literally
        // On the command line we should see:
        // echo "(record{canister_creation_cycles_cost= opt 999;})" | didc encode | didc decode | tee /dev/stderr | target/debug/idl2json --did samples/internet_identity.did  --typ '(record { canister_creation_cycles_cost: nat32; })'
        // (record { 2_138_241_783 = opt (999 : int) })
        // [{"canister_creation_cycles_cost":["911"]}]
        TestVector {
            stdin: "(record { 2_138_241_783 = opt (42 : int) })",
            args: Args {
                typ: Some("(record { canister_creation_cycles_cost: nat32; })".to_string()),
                compact: true,
                ..Args::default()
            },
            stdout: r#"[{"canister_creation_cycles_cost":["42"]}]"#,
        },
        // We shoudl be able to parse a service init type.
        // On the command line we should see:
        // $ echo "(opt record{canister_creation_cycles_cost= opt 6974;})" | didc encode | didc decode | tee /dev/stderr | target/debug/idl2json --did samples/internet_identity.did  --init
        // (opt record { 2_138_241_783 = opt (6_974 : int) })
        // [[{"canister_creation_cycles_cost":["6_974"]}]]
        TestVector {
            stdin: "(opt record { 2_138_241_783 = opt (6_974 : int) })",
            args: Args {
                did: vec![sample_file!("internet_identity.did")],
                typ: None,
                init: true,
                bytes_as: None,
                compact: true,
            },
            stdout: r#"[[{"canister_creation_cycles_cost":["6_974"]}]]"#,
        },
    ];
    for vector in vectors {
        let out = main(&vector.args, vector.stdin)
            .map_err(|e| anyhow!("Failed to parse: {} due to: {e}", vector.stdin))
            .unwrap();
        assert_eq!(vector.stdout, &out)
    }
}

#[test]
fn error_handling_should_be_correct() {
    struct TestVector {
        name: &'static str,
        stdin: &'static str,
        args: Args,
        err: &'static str,
    }
    let vectors = [
        TestVector {
            name: "Invalid candid on stdin",
            stdin: "( this is not candid",
            args: Args::default(),
            err: "Malformed input",
        },
        TestVector {
            name: "Request init args without supplying a did file",
            stdin: r#"("Perfictly  valid candid")"#,
            args: Args {
                init: true,
                ..Args::default()
            },
            err: "Please specify which .did file to use.",
        },
    ];
    for (index, vector) in vectors.iter().enumerate() {
        match main(&vector.args, vector.stdin) {
            Ok(json) => panic!(
                "#{index} ({}) should have caused an error but returned: {json}",
                vector.name
            ),
            Err(err) => {
                let error_message = format!("{err:?}");
                assert!(
                    error_message.contains(vector.err),
                    "The error message for #{index} ({}) should contain '{}': '{}'",
                    vector.name,
                    vector.err,
                    error_message
                );
            }
        }
    }
}
