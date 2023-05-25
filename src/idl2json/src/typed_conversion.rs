use std::iter;

use candid::{
    parser::{
        types::{IDLType, IDLTypes, PrimType, TypeField},
        value::{IDLField, IDLValue},
    },
    IDLArgs,
};
use serde_json::value::Value as JsonValue;

use crate::{
    bytes::convert_bytes, idl2json, untyped_conversion::convert_non_bytes_array, Idl2JsonOptions,
};

/// Converts a candid IDLValue to a serde JsonValue, with keys as names where possible.
///
/// - Key names MAY be incorrect.  They are provided on a best-effort basis.
/// - If types are incompatible with the data, the data wins.
/// - Data is never omitted.
/// - Fields are never added, even if the schema suggests that some fields are missing.
///
/// The data is preserved at all cost, the schema is applied only to make the data easier to understand and use.
///
/// Note: The textual format in parentheses `(  )` represents IDLArgs containing
/// zero or more IDLValues.  Unless you definitely wish to convert a single value
/// you may wish to consider `idl_args2json_with_weak_names` instead.
pub fn idl2json_with_weak_names(
    idl: &IDLValue,
    idl_type: &IDLType,
    options: &Idl2JsonOptions,
) -> JsonValue {
    match (idl, idl_type) {
        (IDLValue::Bool(bool), _) => JsonValue::Bool(*bool),
        (IDLValue::Null, _) => JsonValue::Null,
        (IDLValue::Text(s), _) => JsonValue::String(s.clone()),
        (IDLValue::Number(s), _) => JsonValue::String(s.clone()), // Unspecified number type
        (IDLValue::Float64(f), _) => serde_json::Number::from_f64(*f)
            .map(JsonValue::Number)
            .unwrap_or_else(|| JsonValue::String("NaN".to_string())),
        (IDLValue::Opt(value), IDLType::OptT(opt_type)) => {
            JsonValue::Array(vec![idl2json_with_weak_names(value, opt_type, options)])
        }
        (IDLValue::Opt(_value), _) => idl2json(idl, options), // Fallback for mismatched types
        (IDLValue::Vec(value), IDLType::VecT(item_type)) => match &**item_type {
            IDLType::PrimT(prim_t) if *prim_t == PrimType::Nat8 => convert_bytes(value, options)
                .unwrap_or_else(|_| convert_non_bytes_array(value, options)),
            _ => JsonValue::Array(
                value
                    .iter()
                    .map(|item| idl2json_with_weak_names(item, item_type, options))
                    .collect(),
            ),
        },
        (IDLValue::Vec(_value), _) => idl2json(idl, options), // Fallback for mismatched types
        (IDLValue::Record(value), IDLType::RecordT(record_types)) => JsonValue::Object(
            value
                .iter()
                .map(|field| convert_idl_field(field, record_types, options))
                .collect(),
        ),
        (IDLValue::Record(_value), _) => idl2json(idl, options), // Fallback for mismatched types
        (IDLValue::Variant(field), IDLType::VariantT(record_types)) => JsonValue::Object(
            vec![convert_idl_field(&field.0, record_types, options)]
                .into_iter()
                .collect(),
        ),
        (IDLValue::Variant(_field), _) => idl2json(idl, options), // Fallback for mismatched types
        (IDLValue::Principal(p), _) => JsonValue::String(p.to_string()),
        (IDLValue::Service(p), _) => JsonValue::String(p.to_string()),
        (IDLValue::Func(p, c), _) => JsonValue::Object(
            vec![
                ("principal".to_string(), JsonValue::String(p.to_string())),
                ("code".to_string(), JsonValue::String(c.to_string())),
            ]
            .into_iter()
            .collect(),
        ),
        (IDLValue::None, _) => JsonValue::Array(vec![]),
        (IDLValue::Int(i), _) => JsonValue::String(i.to_string()),
        (IDLValue::Nat(i), _) => JsonValue::String(i.to_string()),
        (IDLValue::Nat8(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
        (IDLValue::Nat16(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
        (IDLValue::Nat32(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
        (IDLValue::Nat64(i), _) => JsonValue::String(i.to_string()),
        (IDLValue::Int8(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
        (IDLValue::Int16(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
        (IDLValue::Int32(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
        (IDLValue::Int64(i), _) => JsonValue::String(i.to_string()),
        (IDLValue::Float32(f), _) => {
            // As far as I can see, JsonValue does not have an explicit NaN type so we provide NaN as a string.
            serde_json::Number::from_f64(*f as f64)
                .map(JsonValue::Number)
                .unwrap_or_else(|| JsonValue::String("NaN".to_string()))
        }
        (IDLValue::Reserved, _) => JsonValue::String(idl.to_string()),
    }
}

/// Returns a typed IDLField as a (key, value) pair.
///
/// - The key is obtained from the type, if possible, else is the raw key as given.
/// - The value is a typed conversion, if the type is as specified, else it is converted without the benefit of type information.
fn convert_idl_field(
    field: &IDLField,
    record_types: &[TypeField],
    options: &Idl2JsonOptions,
) -> (String, JsonValue) {
    let field_id = field.id.get_id();
    let field_type = record_types
        .iter()
        .find(|field_type| field_type.label.get_id() == field_id);
    field_type
        .map(|field_type| {
            (
                field_type.label.to_string(),
                idl2json_with_weak_names(&field.val, &field_type.typ, options),
            )
        })
        .unwrap_or_else(|| (field.id.to_string(), idl2json(&field.val, options)))
}

/// Converts a candid IDLArgs to a serde JsonValue, with keys as names where possible.
pub fn idl_args2json_with_weak_names(
    _idl: &IDLArgs,
    idl_types: &IDLTypes,
    _options: &Idl2JsonOptions,
) -> JsonValue {
    /// If insufficient types are provided, we still include the remaining values interpreted with an empty type.
    let extension_type = IDLType::PrimT(PrimType::Null);
    let idl_type_extension = idl_types.args.iter().chain(iter::repeat(&extension_type));
    unimplemented!()
}
