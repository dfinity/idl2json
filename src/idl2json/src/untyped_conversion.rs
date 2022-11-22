use candid::parser::value::IDLValue;
use serde_json::value::Value as JsonValue;

/// Converts a candid IDLValue to a serde JsonValue, without type information.
pub fn idl2json(idl: &IDLValue) -> JsonValue {
    match idl {
        IDLValue::Bool(bool) => JsonValue::Bool(*bool),
        IDLValue::Null => JsonValue::Null,
        IDLValue::Text(s) => JsonValue::String(s.clone()),
        IDLValue::Number(s) => JsonValue::String(s.clone()), // Unspecified number type
        IDLValue::Float64(f) => {
            JsonValue::Number(serde_json::Number::from_f64(*f).expect("A float's a float"))
        }
        IDLValue::Opt(value) => JsonValue::Array(vec![idl2json(value)]),
        IDLValue::Vec(value) => JsonValue::Array(value.iter().map(idl2json).collect()),
        IDLValue::Record(value) => JsonValue::Object(
            value
                .iter()
                .map(|field| (format!("{}", field.id), idl2json(&field.val)))
                .collect(),
        ),
        IDLValue::Variant(field) => JsonValue::Object(
            vec![(format!("{}", field.0.id), idl2json(&field.0.val))]
                .into_iter()
                .collect(),
        ),
        IDLValue::Principal(p) => JsonValue::String(format!("{}", p)),
        IDLValue::Service(p) => JsonValue::String(format!("{}", p)),
        IDLValue::Func(p, c) => JsonValue::Object(
            vec![
                ("principal".to_string(), JsonValue::String(format!("{}", p))),
                ("code".to_string(), JsonValue::String(c.to_string())),
            ]
            .into_iter()
            .collect(),
        ),
        IDLValue::None => JsonValue::Array(vec![]),
        IDLValue::Int(i) => JsonValue::String(format!("{}", i)),
        IDLValue::Nat(i) => JsonValue::String(format!("{}", i)),
        IDLValue::Nat8(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Nat16(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Nat32(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Nat64(i) => JsonValue::String(format!("{}", i)),
        IDLValue::Int8(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Int16(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Int32(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Int64(i) => JsonValue::String(format!("{}", i)),
        IDLValue::Float32(f) => {
            JsonValue::Number(serde_json::Number::from_f64(*f as f64).expect("A float's a float"))
        }
        IDLValue::Reserved => panic!("Unimplemented: {:?}", idl),
    }
}
