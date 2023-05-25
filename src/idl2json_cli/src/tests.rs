//! Tests for idl2json_cli.
#![warn(missing_docs)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use super::{main, Args};
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
        // On the command line we should see:
        // $ echo "(record{canister_creation_cycles_cost= opt 999;})" | didc encode | didc decode | tee /dev/stderr | idl2json --did samples/internet_identity.did  --typ InternetIdentityInit
        // (record { 2_138_241_783 = opt (999 : int) })
        // {"canister_creation_cycles_cost":["999"]}
        TestVector {
            stdin: "(record { 2_138_241_783 = opt (999 : int) })",
            args: typed_arg!("internet_identity.did", "InternetIdentityInit"),
            stdout: r#"{"canister_creation_cycles_cost":["999"]}"#,
        },
    ];
    for vector in vectors {
        eprintln!("{vector:#?}");
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
            name: "Typo in type name",
            stdin: r#"( "perfectly valid candid" )"#,
            args: typed_arg!("internet_identity.did", "IIInnit"),
            err: r#"Type 'IIInnit' not found in .did file"#,
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
