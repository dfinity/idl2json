use anyhow::{anyhow, Context};
use candid::{parser::types::IDLType, IDLArgs, IDLProg};
use clap::Parser;
use idl2json::{idl2json, idl2json_with_weak_names, polyfill, Idl2JsonOptions};
use std::{
    io::{self, Read},
    path::PathBuf,
    str::FromStr,
};

/// Reads IDL from stdin, writes JSON to stdout.
pub fn main(args: &Args) -> anyhow::Result<()> {
    let idl_args = {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .with_context(|| anyhow!("Failed to read string from stdin"))?;
        let idl_args: IDLArgs = buffer.parse().expect("Malformed input");
        idl_args
    };
    let idl_value = idl_args
        .args
        .get(0)
        .ok_or_else(|| anyhow!("No value in input"))?;

    let idl2json_options = Idl2JsonOptions::default();

    let json_value = if let (Some(did), Some(typ)) = (&args.did, &args.typ) {
        let idl_type: IDLType = {
            let prog = {
                let did_as_str = std::fs::read_to_string(&did)
                    .with_context(|| anyhow!("Could not read did file '{}'.", did.display()))?;
                IDLProg::from_str(&did_as_str)
                    .with_context(|| anyhow!("Failed to parse did file '{}'", did.display()))?
            };
            polyfill::idl_prog::get(&prog, typ).ok_or_else(|| {
                anyhow!("Type '{typ}' not found in .did file '{}'.", did.display())
            })?
        };
        idl2json_with_weak_names(idl_value, &idl_type, &idl2json_options)
    } else {
        idl2json(idl_value, &idl2json_options)
    };
    println!(
        "{}",
        serde_json::to_string(&json_value).expect("Cannot get it out")
    );

    Ok(())
}

/// Converts Candid on stdin to JSON on stdout.
#[derive(Parser, Debug)]
#[clap(name("idl2json"), version = concat!(env!("CARGO_PKG_VERSION"), "\ncandid ", env!("CARGO_CANDID_VERSION")))]
pub struct Args {
    /// A .did file containing type definitions
    #[clap(short, long)]
    did: Option<PathBuf>,
    /// The name of a type in the provided .did file
    #[clap(short, long)]
    typ: Option<String>,
}
