use candid::parser::types::IDLType;
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
