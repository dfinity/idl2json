mod typed_conversion;
mod untyped_conversion;
pub use untyped_conversion::idl2json;
pub use typed_conversion::idl2json_with_weak_names;
pub use serde_json::Value as JsonValue;
#[cfg(test)]
mod test;
