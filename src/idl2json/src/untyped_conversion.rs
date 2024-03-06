use serde_json::value::Value as JsonValue;
use candid::types::value::IDLValue;
use candid_parser::IDLArgs;
use crate::{bytes::{convert_bytes, format_blob}, Idl2JsonOptions};

/// Converts a candid IDLValue to a serde JsonValue, without type information.
///
/// Note: The textual format in parentheses `(  )` represents IDLArgs containing
/// zero or more IDLValues.  Unless you definitely wish to convert a single value
/// you may wish to consider `idl_args2json` instead.
pub fn idl2json(idl: &IDLValue, options: &Idl2JsonOptions) -> JsonValue {
    match idl {
        IDLValue::Blob(bytes) => format_blob(bytes, &options.bytes_as.unwrap_or_default()),
        IDLValue::Bool(bool) => JsonValue::Bool(*bool),
        IDLValue::Null => JsonValue::Null,
        IDLValue::Text(s) => JsonValue::String(s.clone()),
        IDLValue::Number(s) => JsonValue::String(s.clone()), // Unspecified number type
        IDLValue::Float64(f) => serde_json::Number::from_f64(*f)
            .map(JsonValue::Number)
            .unwrap_or_else(|| JsonValue::String("NaN".to_string())),
        IDLValue::Opt(value) => JsonValue::Array(vec![idl2json(value, options)]),
        IDLValue::Vec(value) => convert_bytes(value, options)
            .unwrap_or_else(|_| convert_non_bytes_array(value, options)),
        IDLValue::Record(value) => JsonValue::Object(
            value
                .iter()
                .map(|field| (format!("{}", field.id), idl2json(&field.val, options)))
                .collect(),
        ),
        IDLValue::Variant(field) => JsonValue::Object(
            vec![(format!("{}", field.0.id), idl2json(&field.0.val, options))]
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
        IDLValue::Float32(f) => serde_json::Number::from_f64(*f as f64)
            .map(JsonValue::Number)
            .unwrap_or_else(|| JsonValue::String("NaN".to_string())),
        IDLValue::Reserved => JsonValue::String(idl.to_string()),
    }
}

/// Conver
pub(crate) fn convert_non_bytes_array(value: &[IDLValue], options: &Idl2JsonOptions) -> JsonValue {
    JsonValue::Array(value.iter().map(|item| idl2json(item, options)).collect())
}

/// Converts a candid IDLArgs to a serde JsonValue, without type information.
///
/// Note: The textual format `( )` containing zero or more values represents an IDLArgs.
pub fn idl_args2json(args: &IDLArgs, options: &Idl2JsonOptions) -> JsonValue {
    convert_non_bytes_array(&args.args, options)
}
