use candid_parser::types::IDLType;
use pretty_assertions::assert_eq;
use serde_yaml::Value as YamlValue;
use yaml2candid::Yaml2Candid;

#[test]
fn should_convert_sns_yaml() {
    let converter = Yaml2Candid::from_did_file("tests/nns-sns-wasm.did").unwrap();
    let yaml_str = include_str!("nns-sns-wasm/SnsInitPayload.yaml");
    let idl_str = include_str!("nns-sns-wasm/SnsInitPayload.idl");
    let yaml_value: YamlValue = serde_yaml::from_str(yaml_str).unwrap();
    let idl = converter
        .convert(&IDLType::VarT("SnsInitPayload".to_string()), &yaml_value)
        .unwrap();
    let actual_idl_str = idl.to_string();
    assert_eq!(&idl_str, &actual_idl_str);
}

#[test]
fn should_convert_sns_yaml_strings() {
    let converter = Yaml2Candid::from_did_file("tests/nns-sns-wasm.did").unwrap();
    let yaml_str = include_str!("nns-sns-wasm/SnsInitPayload.yaml");
    let expected_idl_str = include_str!("nns-sns-wasm/SnsInitPayload.idl");
    let actual_idl_str = converter.convert_str("SnsInitPayload", yaml_str).unwrap();
    assert_eq!(expected_idl_str, &actual_idl_str);
}

#[test]
fn should_fail_if_absolute_did_file_is_missing() {
    let converter = Yaml2Candid::from_did_file("/nonexistent.did");
    assert!(converter.is_err());
}

#[test]
fn should_fail_if_relative_did_file_is_missing() {
    let converter = Yaml2Candid::from_did_file("tests/nonexistent.did");
    assert!(converter.is_err());
}
