//! Code for manipulating candid types.
use candid::{
    parser::types::{IDLType, PrimType, TypeField},
    types::internal::{Field as InternalField, Type as InternalType},
};

/// Deriving CandidType on a RustType provides
/// the Candid type, however only as an internal
/// type incompatible with IDLType.  Let's convert.
///
/// There may be an existing conversion function
/// but I cannot see it.
#[allow(clippy::unimplemented)]
pub fn internal_candid_type_to_idl_type(internal_type: &InternalType) -> IDLType {
    match internal_type {
        InternalType::Null => IDLType::PrimT(PrimType::Null),
        InternalType::Bool => IDLType::PrimT(PrimType::Null),
        InternalType::Nat => IDLType::PrimT(PrimType::Nat),
        InternalType::Int => IDLType::PrimT(PrimType::Int),
        InternalType::Nat8 => IDLType::PrimT(PrimType::Nat8),
        InternalType::Nat16 => IDLType::PrimT(PrimType::Nat16),
        InternalType::Nat32 => IDLType::PrimT(PrimType::Nat32),
        InternalType::Nat64 => IDLType::PrimT(PrimType::Nat64),
        InternalType::Int8 => IDLType::PrimT(PrimType::Int8),
        InternalType::Int16 => IDLType::PrimT(PrimType::Int16),
        InternalType::Int32 => IDLType::PrimT(PrimType::Int32),
        InternalType::Int64 => IDLType::PrimT(PrimType::Int64),
        InternalType::Float32 => IDLType::PrimT(PrimType::Float32),
        InternalType::Float64 => IDLType::PrimT(PrimType::Float64),
        InternalType::Text => IDLType::PrimT(PrimType::Text),
        InternalType::Reserved => IDLType::PrimT(PrimType::Reserved),
        InternalType::Empty => IDLType::PrimT(PrimType::Empty),
        InternalType::Knot(_) => unimplemented!(),
        InternalType::Var(_) => unimplemented!(),
        InternalType::Unknown => unimplemented!(),
        InternalType::Opt(boxed_type) => {
            IDLType::OptT(Box::new(internal_candid_type_to_idl_type(boxed_type)))
        }
        InternalType::Vec(items) => {
            IDLType::VecT(Box::new(internal_candid_type_to_idl_type(items)))
        }
        InternalType::Record(fields) => {
            IDLType::RecordT(fields.iter().map(internal_field_type_to_idl_type).collect())
        }
        InternalType::Variant(fields) => {
            IDLType::VariantT(fields.iter().map(internal_field_type_to_idl_type).collect())
        }
        InternalType::Func(_) => unimplemented!(),
        InternalType::Service(_) => unimplemented!(),
        InternalType::Class(_, _) => unimplemented!(),
        InternalType::Principal => IDLType::PrincipalT,
    }
}

fn internal_field_type_to_idl_type(field: &InternalField) -> TypeField {
    TypeField {
        label: field.id.clone(),
        typ: internal_candid_type_to_idl_type(&field.ty),
    }
}
