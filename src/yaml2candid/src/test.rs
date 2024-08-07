//! Tests converting from YAML to Candid, especially extreme values of primitive types.
#![allow(clippy::panic)] // Tests are allowed to panic!
use anyhow::Context;
use candid::types::{value::IDLField, Label};
use candid_parser::types::TypeField;
use num_bigint::{BigInt, BigUint};
use pretty_assertions::assert_eq;
use serde_yaml::Number;

use super::{IDLType, IDLValue, Yaml2Candid, YamlValue};

fn assert_conversion_is(
    converter: &Yaml2Candid,
    typ: &IDLType,
    data: &YamlValue,
    expected_result: IDLValue,
) {
    let value = converter
        .convert(typ, data)
        .expect("Failed to convert YAML to Candid.");
    assert_eq!(value, expected_result);
}

fn assert_named_conversion_is(
    converter: &Yaml2Candid,
    typ: &IDLType,
    data: &YamlValue,
    expected_result: IDLValue,
    name: &str,
) {
    let value = converter
        .convert(typ, data)
        .with_context(|| format!("Failed to convert {name}"))
        .expect("Failed to convert YAML to Candid.");
    assert_eq!(value, expected_result, "Unexpected conversion of {name}");
}

fn assert_conversion_fails(converter: &Yaml2Candid, typ: &IDLType, data: &YamlValue) {
    let result = converter.convert(typ, data);
    assert!(
        result.is_err(),
        "Converting {data:?} to {typ:?} should fail."
    );
}

#[test]
fn can_convert_i8() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int8);
    for value in [i8::MIN, i8::MIN + 1, -1, 0, 1, i8::MAX - 1, i8::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Int8(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_i8_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int8);
    for data in [
        YamlValue::from("foo"),
        YamlValue::from(i64::MAX),
        YamlValue::from(0.5),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_u8() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat8);
    for value in [0, 1, u8::MAX - 1, u8::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Nat8(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_u8_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat8);
    for data in [
        YamlValue::from("foo"),
        YamlValue::from(i64::MAX),
        YamlValue::from(0.5),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_i16() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int16);
    for value in [i16::MIN, i16::MIN + 1, -1, 0, 1, i16::MAX - 1, i16::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Int16(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_i16_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int16);
    for data in [
        YamlValue::from("foo"),
        YamlValue::from(i64::MAX),
        YamlValue::from(0.5),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_u16() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat16);
    for value in [0, 1, u16::MAX - 1, u16::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Nat16(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_u16_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat16);
    for data in [
        YamlValue::from("foo"),
        YamlValue::from(i64::MAX),
        YamlValue::from(0.5),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_i32() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int32);
    for value in [i32::MIN, i32::MIN + 1, -1, 0, 1, i32::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Int32(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_i32_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int32);
    for data in [
        YamlValue::from("foo"),
        YamlValue::from(i64::MAX),
        YamlValue::from(0.5),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_u32() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat32);
    for value in [0, 1, u32::MAX - 1, u32::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Nat32(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_u32_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat32);
    for data in [
        YamlValue::from("foo"),
        YamlValue::from(i64::MAX),
        YamlValue::from(0.5),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_i64() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int64);
    for value in [i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Int64(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_i64_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int64);
    for data in [
        YamlValue::from("foo"),
        YamlValue::from(u64::MAX),
        YamlValue::from(0.5),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_u64() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat64);
    for value in [0, 1, u64::MAX - 1, u64::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Nat64(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_u64_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat64);
    for data in [
        YamlValue::from("foo"),
        YamlValue::from(-1),
        YamlValue::from(0.5),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_small_unsigned_ints() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat);
    for value in [0, 1, u64::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Nat(candid::Nat(BigUint::from(*value)));
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn can_convert_large_unsigned_ints_from_strings() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat);
    for value in ["0", "1", "123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Nat(candid::Nat(value.parse().expect("Test error: String used in test is not a valid BigUint.")));
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_nat_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat);
    for data in [
        YamlValue::from("foo"),
        YamlValue::from(-1),
        YamlValue::from(0.5),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_small_ints() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int);
    for value in [i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Int(candid::Int(BigInt::from(*value)));
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn can_convert_large_ints_from_strings() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int);
    for value in ["-123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890","-1","0", "1", "123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Int(candid::Int(value.parse().expect("Test error: String used in test is not a valid BigInt.")));
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_int_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int);
    for data in [
        YamlValue::from("foo"),
        YamlValue::Null,
        YamlValue::from(0.5),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_f32() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Float32);
    for value in [0f32, -0.125, 0.5, f32::MAX, f32::MIN].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Float32(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_f32_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Float32);
    for data in [YamlValue::Null, YamlValue::from("FOO")].iter() {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_f64() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Float64);
    for value in [0f64, -0.125, 0.5, f64::MAX, f64::MIN].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Float64(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_f64_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Float64);
    for data in [YamlValue::from("FOO"), YamlValue::Null].iter() {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_bool() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Bool);
    for value in [true, false].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Bool(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_bool_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Bool);
    for data in [
        YamlValue::from(0),
        YamlValue::from(1),
        YamlValue::from("FOO"),
        YamlValue::from("true"),
        YamlValue::from("false"),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_null() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Null);
    let data = YamlValue::Null;
    let expected_result = IDLValue::Null;
    assert_conversion_is(&converter, &typ, &data, expected_result);
}

#[test]
fn conversion_to_null_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Null);
    for data in [
        YamlValue::from(0),
        YamlValue::from(1),
        YamlValue::from("FOO"),
        YamlValue::Bool(false),
        YamlValue::Bool(true),
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_string() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Text);
    for value in ["", "foo", "bar", "baz"].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Text(value.to_string());
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_string_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Text);
    for data in [YamlValue::from(0), YamlValue::Bool(false), YamlValue::Null].iter() {
        assert_conversion_fails(&converter, &typ, data);
    }
}

#[test]
fn can_convert_reserved() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Reserved);
    for data in [YamlValue::Null, YamlValue::from("foo"), YamlValue::from(6)].iter() {
        let expected_result = IDLValue::Reserved;
        assert_conversion_is(&converter, &typ, data, expected_result);
    }
}

#[test]
fn conversion_to_empty_should_always_fail() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Empty);
    for data in [
        YamlValue::from(0),
        YamlValue::from(1),
        YamlValue::from("FOO"),
        YamlValue::Bool(true),
        YamlValue::Bool(false),
        YamlValue::Number(Number::from(8)),
        YamlValue::Null,
    ]
    .iter()
    {
        assert_conversion_fails(&converter, &typ, data);
    }
}

struct TestVec {
    description: &'static str,
    typ: IDLType,
    data: YamlValue,
    expected_result: IDLValue,
}

#[test]
fn can_convert() {
    let converter = Yaml2Candid::default();
    let test_vectors = vec![
        TestVec {
            description: "Vector of u8s",
            typ: IDLType::VecT(Box::new(IDLType::PrimT(
                candid_parser::types::PrimType::Int8,
            ))),
            data: YamlValue::Sequence(vec![
                YamlValue::from(1),
                YamlValue::from(2),
                YamlValue::from(3),
            ]),
            expected_result: IDLValue::Vec(vec![
                IDLValue::Int8(1),
                IDLValue::Int8(2),
                IDLValue::Int8(3),
            ]),
        },
        TestVec {
            description: "hex encoded blob",
            typ: IDLType::VecT(Box::new(IDLType::PrimT(
                candid_parser::types::PrimType::Nat8,
            ))),
            data: YamlValue::from("0x010203090a1000"),
            expected_result: IDLValue::Vec(vec![
                IDLValue::Nat8(1),
                IDLValue::Nat8(2),
                IDLValue::Nat8(3),
                IDLValue::Nat8(9),
                IDLValue::Nat8(10),
                IDLValue::Nat8(16),
                IDLValue::Nat8(0),
            ]),
        },
        TestVec {
            description: "base64 encoded blob",
            typ: IDLType::VecT(Box::new(IDLType::PrimT(
                candid_parser::types::PrimType::Nat8,
            ))),
            data: YamlValue::from("base64,AQIDCQoQAA=="),
            expected_result: IDLValue::Vec(vec![
                IDLValue::Nat8(1),
                IDLValue::Nat8(2),
                IDLValue::Nat8(3),
                IDLValue::Nat8(9),
                IDLValue::Nat8(10),
                IDLValue::Nat8(16),
                IDLValue::Nat8(0),
            ]),
        },
        TestVec {
            description: "Some(5) in canonical form",
            typ: IDLType::OptT(Box::new(IDLType::PrimT(
                candid_parser::types::PrimType::Int8,
            ))),
            data: YamlValue::Sequence(vec![YamlValue::from(5)]),
            expected_result: IDLValue::Opt(Box::new(IDLValue::Int8(5))),
        },
        TestVec {
            description: "Record",
            typ: IDLType::RecordT(vec![TypeField {
                label: Label::Named("Foo".to_string()),
                typ: IDLType::PrimT(candid_parser::types::PrimType::Int8),
            }]),
            data: YamlValue::Mapping(
                [(YamlValue::from("Foo"), YamlValue::from(8))]
                    .into_iter()
                    .collect(),
            ),
            expected_result: IDLValue::Record(vec![IDLField {
                id: Label::Named("Foo".to_string()),
                val: IDLValue::Int8(8),
            }]),
        },
        TestVec {
            description: "Record containing None in canonical form",
            typ: IDLType::RecordT(vec![TypeField {
                label: Label::Named("Foo".to_string()),
                typ: IDLType::OptT(Box::new(IDLType::PrimT(
                    candid_parser::types::PrimType::Int8,
                ))),
            }]),
            data: YamlValue::Mapping(
                [(YamlValue::from("Foo"), YamlValue::Sequence(vec![]))]
                    .into_iter()
                    .collect(),
            ),
            expected_result: IDLValue::Record(vec![IDLField {
                id: Label::Named("Foo".to_string()),
                val: IDLValue::None,
            }]),
        },
        TestVec {
            description: "Record containing None in conventional form (omission)",
            typ: IDLType::RecordT(vec![TypeField {
                label: Label::Named("Foo".to_string()),
                typ: IDLType::OptT(Box::new(IDLType::PrimT(
                    candid_parser::types::PrimType::Int8,
                ))),
            }]),
            data: YamlValue::Mapping([].into_iter().collect()),
            expected_result: IDLValue::Record(vec![IDLField {
                id: Label::Named("Foo".to_string()),
                val: IDLValue::None,
            }]),
        },
        TestVec {
            description: "Record containing Some(8) in canonical form",
            typ: IDLType::RecordT(vec![TypeField {
                label: Label::Named("Foo".to_string()),
                typ: IDLType::OptT(Box::new(IDLType::PrimT(
                    candid_parser::types::PrimType::Int8,
                ))),
            }]),
            data: YamlValue::Mapping(
                [(
                    YamlValue::from("Foo"),
                    YamlValue::Sequence(vec![YamlValue::from(8)]),
                )]
                .into_iter()
                .collect(),
            ),
            expected_result: IDLValue::Record(vec![IDLField {
                id: Label::Named("Foo".to_string()),
                val: IDLValue::Opt(Box::new(IDLValue::Int8(8))),
            }]),
        },
        TestVec {
            description: "Record containing Some(8) in conventional form",
            typ: IDLType::RecordT(vec![TypeField {
                label: Label::Named("Foo".to_string()),
                typ: IDLType::OptT(Box::new(IDLType::PrimT(
                    candid_parser::types::PrimType::Int8,
                ))),
            }]),
            data: YamlValue::Mapping(
                [(YamlValue::from("Foo"), YamlValue::from(8))]
                    .into_iter()
                    .collect(),
            ),
            expected_result: IDLValue::Record(vec![IDLField {
                id: Label::Named("Foo".to_string()),
                val: IDLValue::Opt(Box::new(IDLValue::Int8(8))),
            }]),
        },
        TestVec {
            description: "Record containing Some([8]) in conventional form",
            typ: IDLType::RecordT(vec![TypeField {
                label: Label::Named("Foo".to_string()),
                typ: IDLType::OptT(Box::new(IDLType::VecT(Box::new(IDLType::PrimT(
                    candid_parser::types::PrimType::Int8,
                ))))),
            }]),
            data: YamlValue::Mapping(
                [(
                    YamlValue::from("Foo"),
                    YamlValue::Sequence(vec![YamlValue::from(8)]),
                )]
                .into_iter()
                .collect(),
            ),
            expected_result: IDLValue::Record(vec![IDLField {
                id: Label::Named("Foo".to_string()),
                val: IDLValue::Opt(Box::new(IDLValue::Vec(vec![IDLValue::Int8(8)]))),
            }]),
        },
        TestVec {
            description: "Record containing Some([5,6]) in conventional form",
            typ: IDLType::RecordT(vec![TypeField {
                label: Label::Named("Foo".to_string()),
                typ: IDLType::OptT(Box::new(IDLType::VecT(Box::new(IDLType::PrimT(
                    candid_parser::types::PrimType::Int8,
                ))))),
            }]),
            data: YamlValue::Mapping(
                [(
                    YamlValue::from("Foo"),
                    YamlValue::Sequence(vec![YamlValue::from(5), YamlValue::from(6)]),
                )]
                .into_iter()
                .collect(),
            ),
            expected_result: IDLValue::Record(vec![IDLField {
                id: Label::Named("Foo".to_string()),
                val: IDLValue::Opt(Box::new(IDLValue::Vec(vec![
                    IDLValue::Int8(5),
                    IDLValue::Int8(6),
                ]))),
            }]),
        },
        TestVec {
            description: "Function pointer",
            typ: IDLType::FuncT(candid_parser::types::FuncType {
                modes: vec![],
                args: vec![IDLType::PrimT(candid_parser::types::PrimType::Int8)],
                rets: vec![IDLType::PrimT(candid_parser::types::PrimType::Int8)],
            }),
            data: YamlValue::Sequence(vec![
                YamlValue::from("2vxsx-fae"),
                YamlValue::from("my_fun"),
            ]),
            expected_result: IDLValue::Func(candid::Principal::anonymous(), "my_fun".to_string()),
        },
    ];

    for TestVec {
        description,
        typ,
        data,
        expected_result,
    } in test_vectors.into_iter()
    {
        assert_named_conversion_is(&converter, &typ, &data, expected_result, description);
    }
}

/// A conversion that should fail.
struct ErrorTestVec {
    typ: IDLType,
    data: YamlValue,
    expected_error: &'static str,
}
impl ErrorTestVec {
    pub fn should_fail(&self) {
        let Self {
            typ,
            data,
            expected_error,
        } = self;
        let converter = Yaml2Candid::default();
        let result = converter.convert(typ, data);
        if let Err(e) = result {
            assert!(e.to_string().contains(expected_error))
        } else {
            panic!("Converting {data:?} to {typ:?} should fail.");
        }
    }
}

#[test]
fn unsupported_blob_encoding_should_fail() {
    ErrorTestVec{
        typ: IDLType::VecT(Box::new(IDLType::PrimT(
            candid_parser::types::PrimType::Nat8,
        ))),
        data: YamlValue::from("010203090a1000"),
        expected_error: "Unknown encoding for byte vector as string starting: 010203  Please prefix string encoded byte vectors"
    }.should_fail();
}

#[test]
fn unsupported_blob_type_should_fail() {
    ErrorTestVec {
        typ: IDLType::VecT(Box::new(IDLType::PrimT(
            candid_parser::types::PrimType::Nat8,
        ))),
        data: YamlValue::from(9),
        expected_error: "Expected vector of bytes encoded as one of",
    }
    .should_fail();
}
