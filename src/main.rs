use candid::parser::value::IDLValue;
use candid::IDLArgs;
use serde_json::value::Value as JsonValue;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: IDLArgs = buffer.parse().expect("Malformed input");
    println!(
        "{}",
        serde_json::to_string(&idl_to_serde(&args.args[0])).expect("Cannot get it out")
    );

    Ok(())
}

fn idl_to_serde(idl: &IDLValue) -> JsonValue {
    match idl {
        IDLValue::Bool(bool) => JsonValue::Bool(*bool),
        IDLValue::Null => JsonValue::Null,
        IDLValue::Text(s) => JsonValue::String(s.clone()),
        IDLValue::Number(s) => JsonValue::String(s.clone()), // Unspecified number type
        IDLValue::Float64(f) => {
            JsonValue::Number(serde_json::Number::from_f64(*f).expect("A float's a float"))
        }
        IDLValue::Opt(value) => JsonValue::Array(vec![idl_to_serde(value)]),
        IDLValue::Vec(value) => JsonValue::Array(value.iter().map(idl_to_serde).collect()),
        IDLValue::Record(value) => JsonValue::Object(
            value
                .iter()
                .map(|field| (format!("{}", field.id), idl_to_serde(&field.val)))
                .collect(),
        ),
        IDLValue::Variant(field, _) => JsonValue::Object(
            vec![(format!("{}", field.id), idl_to_serde(&field.val))]
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
        IDLValue::Nat64(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Int8(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Int16(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Int32(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Int64(i) => JsonValue::Number(serde_json::Number::from(*i)),
        IDLValue::Float32(f) => {
            JsonValue::Number(serde_json::Number::from_f64(*f as f64).expect("A float's a float"))
        }
        IDLValue::Reserved => panic!("Unimplemented: {:?}", idl),
    }
}
