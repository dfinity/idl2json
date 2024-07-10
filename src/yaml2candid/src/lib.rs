//! Library for converting YAML to IDL values.
#![warn(missing_docs)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
use anyhow::{anyhow, bail, Context};
use candid::{
    types::value::{IDLField, IDLValue, VariantValue},
    Principal,
};
use candid_parser::{
    types::{Dec, IDLType},
    IDLProg,
};
use num_bigint::{BigInt, BigUint};
use serde_yaml::Value as YamlValue;
use std::path::Path;
#[cfg(test)]
mod test;

/// Converts YAML to Candid using a given did file.
pub struct Yaml2Candid {
    /// The types that the converter supports, defined in an IDLProg interface definition.
    pub prog: IDLProg,
}
impl Default for Yaml2Candid {
    /// Creates a Yaml2Candid converter with no types.
    fn default() -> Self {
        Yaml2Candid {
            prog: IDLProg {
                decs: vec![],
                actor: None,
            },
        }
    }
}
impl Yaml2Candid {
    /// Utility that creates a Yaml2Candid converter from the did file at the given path.
    ///
    /// # Arguments
    /// * `path` - the path to a candid interface `.did` file.
    ///
    /// # Return value
    /// A `Yaml2Candid` converter for the provided `.did` file
    pub fn from_did_file<P: AsRef<Path> + std::fmt::Display>(path: P) -> anyhow::Result<Self> {
        let did_file = std::fs::read_to_string(&path).with_context(|| {
            let absolute_path = if path.as_ref().is_absolute() {
                path.as_ref().to_path_buf()
            } else {
                std::env::current_dir().unwrap_or_default().join(path)
            };
            format!("Could not open did file: {:?}", absolute_path)
        })?;
        let prog: IDLProg = did_file.parse()?;
        Ok(Yaml2Candid { prog })
    }
    /// Converts a YAML string to a named IDL type.
    ///
    /// # Arguments
    /// * `type_name` - The name of a type in the converter's `.did` file.
    /// * `yaml_str` - The YAML to convert
    ///
    /// # Return value
    /// An Candid string of the requested type, populated with the data in the YAML file.
    ///
    /// # Errors
    /// This function will return an error if:
    ///
    /// * `yaml_str` is not valid YAML.
    pub fn convert_str(&self, type_name: &str, yaml_str: &str) -> anyhow::Result<String> {
        let yaml_value: YamlValue = serde_yaml::from_str(yaml_str)?;
        let converted = self.convert(&IDLType::VarT(type_name.to_string()), &yaml_value)?;
        Ok(converted.to_string())
    }
    /// Converts a YAML value into a Candid value of the given IDL type.
    ///
    /// Note: The converter will ignore additional fields in the YAML file.
    ///
    /// # Arguments
    /// * `typ` - the IDL type to convert to.
    /// * `data` - the YAML value.
    ///
    /// # Panics
    /// * The IDL type contains primitives that this converter does not yet support.
    /// * YAML types do not match the IDL types.
    /// * `type_name` is not defined in the converter's `.did` file.
    pub fn convert(&self, typ: &IDLType, data: &YamlValue) -> anyhow::Result<IDLValue> {
        match typ {
            IDLType::VarT(name) => {
                let typ = self
                    .prog
                    .decs
                    .iter()
                    .find_map(|x| {
                        if let Dec::TypD(y) = x {
                            if y.id == *name {
                                return Some(y.typ.clone());
                            }
                        }
                        None
                    })
                    .ok_or_else(|| anyhow!("Could not find a type called {name:?}"))?;
                self.convert(&typ, data)
            }
            IDLType::PrimT(prim_t) => match prim_t {
                candid_parser::types::PrimType::Int8 => match data {
                    YamlValue::Number(number) => Ok(IDLValue::Int8(i8::try_from(
                        number
                            .as_i64()
                            .with_context(|| "Could not parse number as i64: {number:?}")?,
                    )?)),
                    _ => bail!("Please express this value as a number: {data:?}"),
                },
                candid_parser::types::PrimType::Nat8 => match data {
                    YamlValue::Number(number) => Ok(IDLValue::Nat8(u8::try_from(
                        number
                            .as_u64()
                            .with_context(|| "Could not parse number as u64: {number:?}")?,
                    )?)),
                    _ => bail!("Please express this value as a number: {data:?}"),
                },
                candid_parser::types::PrimType::Int16 => match data {
                    YamlValue::Number(number) => Ok(IDLValue::Int16(i16::try_from(
                        number
                            .as_i64()
                            .with_context(|| "Could not parse number as i64: {number:?}")?,
                    )?)),
                    _ => bail!("Please express this value as a number: {data:?}"),
                },
                candid_parser::types::PrimType::Nat16 => match data {
                    YamlValue::Number(number) => Ok(IDLValue::Nat16(u16::try_from(
                        number
                            .as_u64()
                            .with_context(|| "Could not parse number as u64: {number:?}")?,
                    )?)),
                    _ => bail!("Please express this value as a number: {data:?}"),
                },
                candid_parser::types::PrimType::Int32 => match data {
                    YamlValue::Number(number) => Ok(IDLValue::Int32(i32::try_from(
                        number
                            .as_i64()
                            .with_context(|| "Could not parse number as i64: {number:?}")?,
                    )?)),
                    _ => bail!("Please express this value as a number: {data:?}"),
                },
                candid_parser::types::PrimType::Nat32 => match data {
                    YamlValue::Number(number) => Ok(IDLValue::Nat32(u32::try_from(
                        number
                            .as_u64()
                            .with_context(|| "Could not parse number as u64: {number:?}")?,
                    )?)),
                    _ => bail!("Please express this value as a number: {data:?}"),
                },
                candid_parser::types::PrimType::Int64 => match data {
                    YamlValue::Number(number) => Ok(IDLValue::Int64(
                        number
                            .as_i64()
                            .with_context(|| "Could not parse number as i64: {number:?}")?,
                    )),
                    _ => bail!("Please express this value as a number: {data:?}"),
                },
                candid_parser::types::PrimType::Nat64 => match data {
                    YamlValue::Number(number) => {
                        Ok(IDLValue::Nat64(number.as_u64().with_context(|| {
                            "Could not parse number as u64: {number:?}"
                        })?))
                    }
                    _ => bail!("Please express this value as a number: {data:?}"),
                },
                // Nat is a bigint, so can support arbitrarily large numbers.  In JSON we express large numbers as strings but numbers should be accepted as well.
                candid_parser::types::PrimType::Nat => Ok(IDLValue::Nat(match data {
                    YamlValue::Number(number) => candid::types::number::Nat::from(
                        number
                            .as_u64()
                            .with_context(|| "Could not parse number as u64: {number:?}")?,
                    ),
                    YamlValue::String(value) => candid::types::number::Nat::from(
                        value
                            .parse::<BigUint>()
                            .with_context(|| format!("Could not parse value as BigUint: {value:?}"))?,
                    ),
                    _ => bail!("Please express natural numbers (unsigned ints) as numbers (e.g. 5) or decimal strings (e.g. \"123\"): {data:?}"),
                })),
                // Int is a bigint, so can support arbitrarily large numbers.  In JSON we express large numbers as strings but numbers should be accepted as well.
                candid_parser::types::PrimType::Int => Ok(IDLValue::Int(match data {
                    YamlValue::Number(number) => candid::types::number::Int::from(
                        number
                            .as_i64()
                            .with_context(|| "Could not parse number as i64: {number:?}")?,
                    ),
                    YamlValue::String(value) => candid::types::number::Int::from(
                        value
                            .parse::<BigInt>()
                            .with_context(|| format!("Could not parse value as BigInt: {value:?}"))?,
                    ),
                    _ => bail!("Please express integers as numbers (e.g. 5) or decimal strings (e.g. \"123\"): {data:?}"),
                })),
                candid_parser::types::PrimType::Float32 => match data {
                    YamlValue::Number(number) => Ok(IDLValue::Float32(
                        number
                            .as_f64().map(|val| val as f32)
                            .with_context(|| "Could not parse number as f64: {number:?}")?,
                    )),
                    _ => bail!("Please express this value as a number: {data:?}"),
                },
                candid_parser::types::PrimType::Float64 => match data {
                    YamlValue::Number(number) => Ok(IDLValue::Float64(
                        number
                            .as_f64()
                            .with_context(|| "Could not parse number as f64: {number:?}")?,
                    )),
                    _ => bail!("Please express this value as a number: {data:?}"),
                },
                candid_parser::types::PrimType::Bool => match data {
                    YamlValue::Bool(value) => Ok(IDLValue::Bool(*value)),
                    _ => bail!("Please express this value as a boolean: {data:?}"),
                },
                candid_parser::types::PrimType::Null => match data {
                    YamlValue::Null => Ok(IDLValue::Null),
                    _ => bail!("Please express this value as null: {data:?}"),
                },
                candid_parser::types::PrimType::Text => match data {
                    YamlValue::String(value) => Ok(IDLValue::Text(value.to_string())),
                    _ => bail!("Please express this value as a string: {data:?}"),
                },
                candid_parser::types::PrimType::Reserved => Ok(IDLValue::Reserved),
                candid_parser::types::PrimType::Empty => bail!("Cannot create an Empty type; by definition the Empty type can never occur.  https://internetcomputer.org/docs/current/references/candid-ref#type-empty"),
            },
            IDLType::RecordT(fields) => if let YamlValue::Mapping(mapping) = data {Ok(IDLValue::Record(
                fields
                    .iter()
                    .map(|field| {
                        let id = field.label.clone();
                        let mapping_key = field.label.to_string();
                        let val = mapping.get(&mapping_key);
                        let converted_val = if let Some(val) = val {
                            self.convert(&field.typ, val)?
                        } else if let IDLType::OptT(_) = &field.typ {
                            IDLValue::None
                        } else {
                            bail!("Missing key: {}", &mapping_key)
                        };
                        Ok(IDLField { id, val: converted_val })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            ))} else {
                bail!("Expected a mapping for record type, got: {data:?}")
            },
            IDLType::VecT(typ) => if let YamlValue::Sequence(values) = data { Ok(IDLValue::Vec(
                values
                    .iter()
                    .map(|val| self.convert(typ, val))
                    .collect::<Result<Vec<_>, _>>()?,
            ))} else {
                bail!("Expected a sequence for vec type, got: {data:?}")
            },
            IDLType::VariantT(types) => if let YamlValue::Mapping(value) = data {
                types
                    .iter()
                    .find_map(|typ| {
                        let key = typ.label.to_string();
                        // Note: This lookup can be extended:
                        // - Handle non-string keys
                        // - Start from the one value in Mapping and iterate over types to find a match, rather than iterating
                        //   over types and repeatedly doing the presumably more expensive lookup in mapping.
                        value.get(&key).map(|val| {
                            // u64 represents the index from the type, defaults to 0 when parsing, only used for serialization
                            let numerical_key = u64::from(typ.label.get_id());
                            let field = IDLField {
                                id: typ.label.clone(),
                                val: self.convert(&typ.typ, val).with_context(|| {
                                    format!("Failed to convert variant of type {key}")
                                })?,
                            };
                            Ok(IDLValue::Variant(VariantValue(
                                Box::new(field),
                                numerical_key,
                            )))
                        })
                    })
                    .unwrap_or_else(|| {
                        bail!("Could not find matching type:\n{:?}\n\n{:?}", types, value)
                    })
            } else {
                bail!("Expected a mapping for variant type, got: {data:?}")
            },
            IDLType::OptT(typ) => if let YamlValue::Sequence(values) = data {
                // This is complicated.
                // The standard specifies this conversion: https://internetcomputer.org/docs/current/references/candid-ref#corresponding-javascript-values-9
                // None -> []
                // Some(x) -> [x]
                // Conventionally in JSON:
                // None -> undefined (omitted field)
                // Some(x) -> x
                // When converting we will attempt the standards-compilant conversion first and then fall back to the conventional conversion.
                // TODO: It may be a good idea to add a flag to control this behavior: standards-compliant vs conventional vs guess!
                match values.len() {
                    0 => Ok(IDLValue::None),
                    1 => {
                        // This could either be a standards-compliant use of representing Some(a) as [a] OR it could be the conventional use of Some(a_vec) as a_vec.
                        let correct_conversion = self.convert(typ, &values[0]);
                        match correct_conversion {
                            Ok(val) => Ok(IDLValue::Opt(Box::new(val))),
                            err => {
                                // Try the conventional, technically incorrect, interpretation
                                eprintln!("The recommended (but rarely used) way to represent an optional value is to use an empty sequence for None and a sequence with one element for Some.  Guessing you meant opt vec ...");
                                let conventional_conversion = self.convert(typ, data);
                                if let Ok(val) = conventional_conversion {
                                    Ok(IDLValue::Opt(Box::new(val)))
                                } else {
                                    err
                                }
                            }
                        }}
                    _ => {
                        // TODO: Find a good way of formalizing the common practice of representing None in JSON&YAML as an omitted element and Some(x) as a x being present.
                        eprintln!("The recommended (but rarely used) way to represent an optional value is to use a sequence with one or zero elements.  Guessing you meant opt vec ...");
                        Ok(IDLValue::Opt(Box::new(self.convert(typ, data)?)))
                    }
                }
            } else {
                eprintln!("The recommended (but rarely used) way to represent an optional value is to use a sequence with one or zero elements.");
                Ok(IDLValue::Opt(Box::new(self.convert(typ, data)?)))
            },
            IDLType::PrincipalT => Ok(IDLValue::Principal(Self::parse_principal(data)?)),
            IDLType::FuncT(_func) => {
                // See: https://internetcomputer.org/docs/current/references/candid-ref#type-func---
                match data {
                    YamlValue::Sequence(values) if values.len() == 2 => {
                        let principal = Self::parse_principal(&values[0])?;
                        let name = Self::parse_string(&values[1])?;
                        Ok(IDLValue::Func(principal, name))
                    },
                    _ => bail!("Expected a sequence of length 2 (principal, name) for func type, got: {data:?}"),
                }
            }
            IDLType::ServT(_bindings) => {
                // See: https://internetcomputer.org/docs/current/references/candid-ref#type-service-
                todo!("Conversion of service type is not supported yet.")
            }
            IDLType::ClassT(_, _) => {
                // Not defined here: https://internetcomputer.org/docs/current/references/candid-ref
                todo!("Conversion of class type is not supported yet.")
            }
        }
    }
    /// Gets a string from a YAML value.
    fn parse_string(data: &YamlValue) -> anyhow::Result<String> {
        match data {
            YamlValue::String(value) => Ok(value.to_string()),
            _ => bail!("Expected a string, got: {data:?}"),
        }
    }
    /// Gets a principal from a YAML value.
    fn parse_principal(data: &YamlValue) -> anyhow::Result<Principal> {
        match data {
            YamlValue::String(value) => Ok(Principal::from_text(value)?),
            _ => bail!("Expected a string for principal type, got: {data:?}"),
        }
    }
}
