//! Library of IDL (candid) to JSON conversion functions.
#![warn(missing_docs)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::unimplemented)] // Allowed in some specific places

pub mod candid_types;
mod typed_conversion;
mod untyped_conversion;
pub use serde_json::Value as JsonValue;
pub use typed_conversion::idl2json_with_weak_names;
pub use untyped_conversion::idl2json;
#[cfg(test)]
mod test;

/// Options for idl2json conversions
#[derive(Default, Clone, Eq, PartialEq)]
pub struct Idl2JsonOptions {
    /// How to represent `Vec<u8>`
    pub bytes_as: Option<BytesFormat>,
    /// How to represent `Vec<u8>` of at least some given length.
    pub long_bytes_as: Option<(usize, BytesFormat)>,
}

/// Options for how to represent `Vec<u8>`
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BytesFormat {
    /// Data is represented as an array of numbers: `[1,34,0]`
    Numbers,
    /// Data is represented as hex: `"A4B7"`
    Hex,
    /// Data is represented hex ending in an elipsis with at most the given total number of characters.
    /// E.g. `Ellipsis(7) -> "A5B8..."`
    // Ellipsis(usize), // TODO
    #[cfg(feature = "crypto")]
    /// Data is hashed:  "sha512:abbabababababababbababababab"
    Sha256,
}

impl Default for BytesFormat {
    fn default() -> Self {
        BytesFormat::Numbers
    }
}
