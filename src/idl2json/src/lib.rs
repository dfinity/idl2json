//! Library of IDL (candid) to JSON conversion functions.
#![warn(missing_docs)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::unimplemented)] // Allowed in some specific places

mod bytes;
pub mod candid_types;
pub mod polyfill;
mod typed_conversion;
mod untyped_conversion;

use candid_parser::types::IDLProg;
pub use serde_json::Value as JsonValue;
pub use typed_conversion::{idl2json_with_weak_names, idl_args2json_with_weak_names};
pub use untyped_conversion::{idl2json, idl_args2json};
#[cfg(test)]
mod test;

/// Options for idl2json conversions
#[derive(Default)]
pub struct Idl2JsonOptions {
    /// How to represent `Vec<u8>`
    pub bytes_as: Option<BytesFormat>,
    /// How to represent `Vec<u8>` of at least some given length.
    pub long_bytes_as: Option<(usize, BytesFormat)>,
    /// Type definitions.
    ///
    /// Note:
    /// - An `IDLProg`  corresponds to a parsed `.did` file.
    /// - Typically either no `IDLProg` is available or one `IDLProg`
    ///   is provided, corresponding to the `.did` file of a canister
    ///   and that one `.did` file has all required definitions.
    /// - In rare cases, multiple IDLProgs are needed.  If so,
    ///   `idl2json` will use the first match it finds.  It is the
    ///   caller's responsibility to ensure that there are no conflicting definitions.
    pub prog: Vec<IDLProg>,
    /// Compact JSON, without formatting whitespace.
    pub compact: bool,
}

/// Options for how to represent `Vec<u8>`
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "clap", clap(rename_all = "kebab_case"))]
pub enum BytesFormat {
    /// Data is represented as an array of numbers: `[1,34,0]`
    #[default]
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
