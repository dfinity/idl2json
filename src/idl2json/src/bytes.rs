use crate::{BytesFormat, Idl2JsonOptions};
use candid::types::value::IDLValue;
use serde_json::value::Value as JsonValue;
use sha2::{Digest, Sha256};

/// Converts supposedly binary data.  Returns an error if the data is not binary.
pub fn convert_bytes(bytes: &[IDLValue], options: &Idl2JsonOptions) -> Result<JsonValue, ()> {
    if let Some((len, bytes_format)) = options.long_bytes_as {
        if bytes.len() >= len {
            return format_bytes(bytes, &bytes_format);
        }
    }
    format_bytes(bytes, &(options.bytes_as.unwrap_or_default()))
}
/// Formats supposedly binary data.  Returns an error if the data is not binary.
fn format_bytes(bytes: &[IDLValue], bytes_format: &BytesFormat) -> Result<JsonValue, ()> {
    match bytes_format {
        BytesFormat::Numbers => Ok(JsonValue::Array(
            bytes
                .iter()
                .map(|item| {
                    if let IDLValue::Nat8(value) = item {
                        Ok(JsonValue::Number(serde_json::Number::from(*value)))
                    } else {
                        Err(())
                    }
                })
                .collect::<Result<Vec<JsonValue>, ()>>()?,
        )),
        BytesFormat::Hex => {
            let mut ans = String::with_capacity(bytes.len() * 2);
            for byte in bytes {
                if let IDLValue::Nat8(value) = byte {
                    ans.push_str(nybble2hex(value >> 4));
                    ans.push_str(nybble2hex(value & 0xf));
                } else {
                    return Err(());
                }
            }
            Ok(JsonValue::String(ans))
        }
        #[cfg(feature = "crypto")]
        BytesFormat::Sha256 => {
            let mut hasher = Sha256::new();
            for byte in bytes {
                if let IDLValue::Nat8(value) = byte {
                    hasher.update([*value]);
                } else {
                    return Err(());
                }
            }
            let digest = hasher.finalize();
            Ok(JsonValue::String(format!("Bytes with sha256: {digest:x}")))
        }
    }
}

/// Formats binary data.  Returns an error if the data is not binary.
pub fn format_blob(bytes: &[u8], bytes_format: &BytesFormat) -> JsonValue {
    match bytes_format {
        BytesFormat::Numbers => JsonValue::Array(
            bytes
                .iter()
                .map(|value| JsonValue::Number(serde_json::Number::from(*value)))
                .collect::<Vec<JsonValue>>(),
        ),
        BytesFormat::Hex => {
            let mut ans = String::with_capacity(bytes.len() * 2);
            for value in bytes {
                ans.push_str(nybble2hex(value >> 4));
                ans.push_str(nybble2hex(value & 0xf));
            }
            JsonValue::String(ans)
        }
        #[cfg(feature = "crypto")]
        BytesFormat::Sha256 => {
            let mut hasher = Sha256::new();
            for value in bytes {
                hasher.update([*value]);
            }
            let digest = hasher.finalize();
            JsonValue::String(format!("Bytes with sha256: {digest:x}"))
        }
    }
}

fn nybble2hex(nybble: u8) -> &'static str {
    match nybble {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "a",
        11 => "b",
        12 => "c",
        13 => "d",
        14 => "e",
        15 => "f",
        _ => "?",
    }
}
