//! Code that should be pushed upstream.

/// Polyfills for the candid IDLProg struct.
pub mod idl_prog {
    use candid::{
        parser::types::{Dec, IDLType, IDLTypes},
        IDLProg,
    };

    /// Deprecated; please use `get_type(..)` instead.
    /// TODO: Use the rust deprecated syntax
    pub fn get(prog: &IDLProg, key: &str) -> Option<IDLType> {
        get_type(prog, key)
    }

    /// Polyfill for IDLProg.get(key), that gets the type of the given name.
    pub fn get_type(prog: &IDLProg, key: &str) -> Option<IDLType> {
        prog.decs.iter().find_map(|x| {
            if let Dec::TypD(y) = x {
                if y.id == key {
                    return Some(y.typ.clone());
                }
            }
            None
        })
    }

    /// Gets the arguments for creating a service
    pub fn get_service_arg(_prog: &IDLProg) -> Option<IDLTypes> {
        unimplemented!()
    }
    /// Gets the arguments and return values of a service method.
    ///
    /// Note: In a canister .did file there is typically a service section containing these service methods.
    pub fn get_fn_type(_fn_name: &str, _prog: &IDLProg) -> Option<(IDLTypes, IDLTypes)> {
        unimplemented!()
    }
}