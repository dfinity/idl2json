//! Tests converting from YAML to Candid.
use super::{Yaml2Candid, IDLType, IDLValue, YamlValue};

fn assert_conversion_is(converter: &Yaml2Candid, typ: &IDLType, data: &YamlValue, expected_result: IDLValue) {
    let result = converter.convert(typ, data).unwrap();
    assert_eq!(result, expected_result);
}


#[cfg(test)]
fn can_convert_i8() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int8);
    for value in [i8::MIN, -1, 0, 1, i8::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Int8(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[cfg(test)]
fn can_convert_u8() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat8);
    for value in [0, 1, u8::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Nat8(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[cfg(test)]
fn can_convert_i16() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Int16);
    for value in [i16::MIN, -1, 0, 1, i16::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Int16(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}

#[cfg(test)]
fn can_convert_u16() {
    let converter = Yaml2Candid::default();
    let typ = IDLType::PrimT(candid_parser::types::PrimType::Nat16);
    for value in [0, 1, u16::MAX].iter() {
        let data = YamlValue::from(*value);
        let expected_result = IDLValue::Nat16(*value);
        assert_conversion_is(&converter, &typ, &data, expected_result);
    }
}