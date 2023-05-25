use super::{main, Args};
use anyhow::anyhow;

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

#[test]
fn conversion_with_a_standalone_type_should_be_correct() {
    struct TestVector {
        name: &'static str,
        stdin: &'static str,
        typ: &'static str,
        stdout: &'static str,
    }
    let args = Args::default();
    let vectors = [
        TestVector {
            name: "typed record",
            stdin: "( record {foo=1; bar=2;} )",
            typ: "record { foo: nat8, bar: nat8 }",
            stdout: "{\"bar\":\"2\",\"foo\":\"1\"}",
        },
        /*
        TestVector {
            name: "untyped record",
            stdin: "(record { 4_895_187 = 2 : int; 5_097_222 = 1 : int })", // created with: echo "( record {foo=1; bar=2;} )" | didc encode | didc decode
            typ: "record { foo: nat8, bar: nat8 }",
            stdout: "{\"bar\":\"2\",\"foo\":\"1\"}",
        },*/
    ];
    for vector in vectors {
        let out = main(&args, vector.stdin)
            .map_err(|e| anyhow!("Failed to parse: {} due to: {e}", vector.stdin))
            .unwrap();
        assert_eq!(vector.stdout, &out, "Incorrect output for {}", vector.name)
    }
}

// TODO: conversion with a standalone IDLTypes

// TODO: conversion with alooked - up type

// TODO: Conversion with idlargs containing looked-up types

// TODO: Tarpaulin tests
