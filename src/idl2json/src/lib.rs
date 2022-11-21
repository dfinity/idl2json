//! Library of IDL (candid) to JSON conversion functions.
pub mod candid_types;
mod typed_conversion;
mod untyped_conversion;
pub use serde_json::Value as JsonValue;
pub use typed_conversion::idl2json_with_weak_names;
pub use untyped_conversion::idl2json;
#[cfg(test)]
mod test;
