//! Code that should be pushed upstream.

/// Polyfills for the candid IDLProg struct.
pub mod idl_prog {
    use candid::{
        parser::types::{Dec, IDLType},
        IDLProg,
    };

    /// Polyfill for IDLProg.get(key)
    pub fn get(prog: &IDLProg, key: &str) -> Option<IDLType> {
        prog.decs.iter().find_map(|x| {
            if let Dec::TypD(y) = x {
                if y.id == key {
                    return Some(y.typ.clone());
                }
            }
            None
        })
    }
}
