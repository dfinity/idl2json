//! Library for converting YAML to IDL values.
#![warn(missing_docs)]
use anyhow::Context;
use candid::parser::types::{Dec, IDLType};
use candid::parser::value::{IDLField, IDLValue, VariantValue};
use candid::IDLProg;
use serde_yaml::Value as YamlValue;
use std::path::Path;

/// Converts YAML to IDL using a given did file.
pub struct Yaml2Idl {
    /// The types that the converter supports, defined in an IDLProg interface definition.
    pub prog: IDLProg,
}
impl Yaml2Idl {
    /// Utility that creates a yaml2idl converter from the did file at the given path.
    ///
    /// # Arguments
    /// * `path` - the path to a candid interface `.did` file.
    ///
    /// # Return value
    /// A `Yaml2Idl` converter for the provided `.did` file
    pub fn from_did_file<P: AsRef<Path> + std::fmt::Display>(path: P) -> anyhow::Result<Self> {
        let did_file = std::fs::read_to_string(&path).with_context(|| {
            let absolute_path = if path.as_ref().is_absolute() {
                path.as_ref().to_path_buf()
            } else {
                std::env::current_dir().unwrap_or_default().join(path)
            };
            format!("Could not open did file: {:?}", absolute_path)
        })?;
        let prog: IDLProg = did_file.parse().unwrap();
        Ok(Yaml2Idl { prog })
    }
    /// Converts a YAML string to a named IDL type.
    ///
    /// # Arguments
    /// * `type_name` - The name of a type in the converter's `.did` file.
    /// * `yaml_str` - The YAML to convert
    ///
    /// # Return value
    /// An IDL string of the requested type, populated with the data in the YAML file.
    ///
    /// # Errors
    /// This function will return an error if:
    ///
    /// * `yaml_str` is not valid YAML.
    pub fn convert_str(&self, type_name: &str, yaml_str: &str) -> anyhow::Result<String> {
        let yaml_value: YamlValue = serde_yaml::from_str(yaml_str).unwrap();
        let converted = self.convert(&IDLType::VarT(type_name.to_string()), &yaml_value).unwrap();
        Ok(format!("{:?}", converted))
    }
    /// Converts a YAML value into an IDL value.
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
        match (typ, data) {
            (IDLType::VarT(name), val) => {
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
                    .unwrap();
                self.convert(&typ, val)
            }
            (IDLType::PrimT(candid::parser::types::PrimType::Nat8), YamlValue::Number(number)) => {
                Ok(IDLValue::Nat8(
                    u8::try_from(number.as_u64().unwrap()).unwrap(),
                ))
            }
            (IDLType::PrimT(candid::parser::types::PrimType::Nat16), YamlValue::Number(number)) => {
                Ok(IDLValue::Nat16(
                    u16::try_from(number.as_u64().unwrap()).unwrap(),
                ))
            }
            (IDLType::PrimT(candid::parser::types::PrimType::Nat32), YamlValue::Number(number)) => {
                Ok(IDLValue::Nat32(
                    u32::try_from(number.as_u64().unwrap()).unwrap(),
                ))
            }
            (IDLType::PrimT(candid::parser::types::PrimType::Nat64), YamlValue::Number(number)) => {
                Ok(IDLValue::Nat64(number.as_u64().unwrap()))
            }
            (IDLType::PrimT(candid::parser::types::PrimType::Text), YamlValue::String(value)) => {
                Ok(IDLValue::Text(value.to_string()))
            }
            (IDLType::PrincipalT, YamlValue::String(value)) => Ok(IDLValue::Principal(
                candid::Principal::from_text(value).unwrap(),
            )),
            (IDLType::RecordT(fields), YamlValue::Mapping(mapping)) => Ok(IDLValue::Record(
                fields
                    .iter()
                    .map(|field| {
                        let id = field.label.clone();
                        let mapping_key = field.label.to_string();
                        let val = mapping.get(&mapping_key);
                        let val = match (&field.typ, val) {
                            (IDLType::OptT(typ), Some(val)) => self.convert(typ, val).unwrap(),
                            (IDLType::OptT(_typ), None) => IDLValue::None,
                            (typ, Some(val)) => self.convert(typ, val).unwrap(),
                            (_typ, None) => panic!("Missing key: {}", &mapping_key),
                        };
                        IDLField { id, val }
                    })
                    .collect(),
            )),
            (IDLType::VecT(typ), YamlValue::Sequence(values)) => Ok(IDLValue::Vec(
                values
                    .iter()
                    .map(|val| self.convert(typ, val).unwrap())
                    .collect(),
            )),
            (IDLType::VariantT(types), YamlValue::Mapping(value)) => {
                Ok(types
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
                                val: self.convert(&typ.typ, val).unwrap(),
                            };
                            IDLValue::Variant(VariantValue(Box::new(field), numerical_key))
                        })
                    })
                    .unwrap_or_else(|| {
                        panic!("Could not find matching type:\n{:?}\n\n{:?}", types, value)
                    }))
            }
            (typ, data) => {
                panic!("Unsupported pair:\n{:?}\n\n{:?}", typ, data);
            }
        }
    }
}
