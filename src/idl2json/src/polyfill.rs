//! Code that should be pushed upstream.

/// Polyfills for the candid IDLProg struct.
pub mod idl_prog {
    use candid_parser::{
        types::{Dec, IDLType, IDLTypes},
        IDLProg,
    };

    /// Gets a type defined in a program declarations section.
    #[deprecated(since = "0.8.6", note = "Please use `get_type()` instead.")]
    pub fn get(prog: &IDLProg, key: &str) -> Option<IDLType> {
        get_type(prog, key)
    }

    /// Gets a type defined in a program declarations section.
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

    /// Gets the arguments for creating a service.
    ///
    /// This will return None if the prog contains no service aka actor of type ClassT.
    pub fn get_init_arg_type(prog: &IDLProg) -> Option<IDLTypes> {
        if let Some(IDLType::ClassT(args, _)) = &prog.actor {
            Some(IDLTypes { args: args.clone() })
        } else {
            None
        }
    }
}
