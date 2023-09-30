//! Example of how to convert binary candid to JSON using a schema
use candid::{parser::types::IDLType, types::value::IDLValue, Decode, IDLProg};
use idl2json::{idl2json, idl2json_with_weak_names, polyfill, Idl2JsonOptions};
use std::str::FromStr;

/// Converts some sample candid bytes to JSON using a .did file.
fn main() {
    let type_name = "InternetIdentityInit";
    let prog = {
        let did_as_str = std::fs::read_to_string("../../samples/internet_identity.did")
            .expect("Could not read did file");
        IDLProg::from_str(&did_as_str).expect("Failed to parse did")
    };
    // TODO: This is still unimplemented in candid, but should be available soon.
    //let rust = idl_to_rust(&prog, &Config::default()).expect("Could not compute rust");
    //println!("Rust: {rust}");
    let idl_type = polyfill::idl_prog::get_type(&prog, type_name).expect("Failed to get idltype");
    let idl_type = IDLType::OptT(Box::new(idl_type));
    println!("Type: {:?}\n\n", &idl_type);
    let buffer = [
        68, 73, 68, 76, 5, 110, 1, 108, 2, 196, 136, 191, 215, 1, 2, 247, 245, 203, 251, 7, 4, 110,
        3, 109, 123, 110, 120, 1, 0, 1, 1, 32, 246, 145, 242, 105, 221, 102, 170, 79, 196, 78, 105,
        22, 174, 254, 224, 59, 183, 254, 184, 33, 174, 244, 52, 103, 82, 105, 116, 244, 112, 205,
        75, 7, 1, 0, 16, 165, 212, 232, 0, 0, 0,
    ];
    println!("data: {:?}\n\n", &buffer);
    let idl_value = Decode!(&buffer[..], IDLValue).expect("Failed to parse buffer");
    println!("Value: {:?}\n\n", idl_value);
    println!(
        "Untyped conversion: {:?}\n\n",
        serde_json::to_string(&idl2json(&idl_value, &Idl2JsonOptions::default()))
            .expect("Failed to stringify")
    );

    println!(
        "Typed conversion: {}\n\n",
        serde_json::to_string(&idl2json_with_weak_names(
            &idl_value,
            &idl_type,
            &Idl2JsonOptions::default()
        ))
        .expect("Failed to stringify")
    );
}
