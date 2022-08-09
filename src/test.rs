use candid::IDLArgs;
use std::fs;
use std::io::{self, Read};
use crate::{idl_to_serde, JsonValue};

#[test]
fn idl_is_parsed_as_expected() {
    let idl_string: String = fs::read_to_string("samples/proposal.idl").expect("Could not read sample IDL");
    let expected_json_string: String = fs::read_to_string("samples/proposal.json").expect("Could not read sample JSON");

    let idl_value: IDLArgs = idl_string.parse::<IDLArgs>().expect("Malformed input");
    let json_value: JsonValue = idl_to_serde(&idl_value.args[0]);
    let json_string = serde_json::to_string(&json_value).expect("Cannot stringify JSON");
    assert_eq!(expected_json_string, json_string);
}
