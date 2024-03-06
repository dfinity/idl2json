//! Code for manipulating candid types.
use candid::types::internal::{Field as InternalField, Type as InternalType, TypeInner};
use candid_parser::types::{IDLType, PrimType, TypeField};

/// Deriving CandidType on a RustType provides
/// the Candid type, however only as an internal
/// type incompatible with IDLType.  Let's convert.
///
/// There may be an existing conversion function
/// but I cannot see it.
#[allow(clippy::unimplemented)]
pub fn internal_candid_type_to_idl_type(internal_type: &InternalType) -> IDLType {
    match internal_type.as_ref() {
        TypeInner::Null => IDLType::PrimT(PrimType::Null),
        TypeInner::Bool => IDLType::PrimT(PrimType::Null),
        TypeInner::Nat => IDLType::PrimT(PrimType::Nat),
        TypeInner::Int => IDLType::PrimT(PrimType::Int),
        TypeInner::Nat8 => IDLType::PrimT(PrimType::Nat8),
        TypeInner::Nat16 => IDLType::PrimT(PrimType::Nat16),
        TypeInner::Nat32 => IDLType::PrimT(PrimType::Nat32),
        TypeInner::Nat64 => IDLType::PrimT(PrimType::Nat64),
        TypeInner::Int8 => IDLType::PrimT(PrimType::Int8),
        TypeInner::Int16 => IDLType::PrimT(PrimType::Int16),
        TypeInner::Int32 => IDLType::PrimT(PrimType::Int32),
        TypeInner::Int64 => IDLType::PrimT(PrimType::Int64),
        TypeInner::Float32 => IDLType::PrimT(PrimType::Float32),
        TypeInner::Float64 => IDLType::PrimT(PrimType::Float64),
        TypeInner::Text => IDLType::PrimT(PrimType::Text),
        TypeInner::Reserved => IDLType::PrimT(PrimType::Reserved),
        TypeInner::Empty => IDLType::PrimT(PrimType::Empty),
        TypeInner::Knot(_) => unimplemented!(),
        TypeInner::Var(_) => unimplemented!(),
        TypeInner::Unknown => unimplemented!(),
        TypeInner::Opt(boxed_type) => {
            IDLType::OptT(Box::new(internal_candid_type_to_idl_type(boxed_type)))
        }
        TypeInner::Vec(items) => IDLType::VecT(Box::new(internal_candid_type_to_idl_type(items))),
        TypeInner::Record(fields) => {
            IDLType::RecordT(fields.iter().map(internal_field_type_to_idl_type).collect())
        }
        TypeInner::Variant(fields) => {
            IDLType::VariantT(fields.iter().map(internal_field_type_to_idl_type).collect())
        }
        TypeInner::Func(_) => unimplemented!(),
        TypeInner::Service(_) => unimplemented!(),
        TypeInner::Class(_, _) => unimplemented!(),
        TypeInner::Principal => IDLType::PrincipalT,
        TypeInner::Future => unimplemented!(),
    }
}

fn internal_field_type_to_idl_type(field: &InternalField) -> TypeField {
    TypeField {
        label: (*field.id).clone(),
        typ: internal_candid_type_to_idl_type(&field.ty),
    }
}
