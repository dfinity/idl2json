//! Tests converting from YAML to Candid.
use num_bigint::{BigInt, BigUint};

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
    for data in [YamlValue::from("foo"), YamlValue::from(i64::MAX)].iter() {
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
    for data in [YamlValue::from("foo"), YamlValue::from(i64::MAX)].iter() {
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
    for data in [YamlValue::from("foo"), YamlValue::from(i64::MAX)].iter() {
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
    for data in [YamlValue::from("foo"), YamlValue::from(i64::MAX)].iter() {
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
    for data in [YamlValue::from("foo"), YamlValue::from(i64::MAX)].iter() {
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
    for data in [YamlValue::from("foo"), YamlValue::from(i64::MAX)].iter() {
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
    for data in [YamlValue::from("foo"), YamlValue::from(u64::MAX)].iter() {
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
    for data in [YamlValue::from("foo"), YamlValue::from(-1)].iter() {
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
        let expected_result = IDLValue::Nat(candid::Nat(value.parse().unwrap()));
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_nat_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat64);
    for data in [YamlValue::from("foo"), YamlValue::from(-1)].iter() {
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
        let expected_result = IDLValue::Int(candid::Int(value.parse().unwrap()));
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[test]
fn conversion_to_int_should_fail_for_some_inputs() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat64);
    for data in [YamlValue::from("foo"), YamlValue::Null].iter() {
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
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat64);
    for data in [YamlValue::from(f64::MAX), YamlValue::from("FOO")].iter() {
        assert_conversion_fails(&converter, &typ, data);
    }
}
