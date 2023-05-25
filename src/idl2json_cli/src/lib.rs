//! Command line library for converting candid to JSON.
#![warn(missing_docs)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::unimplemented)]

#[cfg(test)]
mod tests;

use anyhow::{anyhow, Context};
use candid::parser::value::IDLValue;
use candid::{parser::types::IDLType, IDLArgs, IDLProg};
use clap::Parser;
use idl2json::{idl2json, idl2json_with_weak_names, polyfill, Idl2JsonOptions};
use std::{path::PathBuf, str::FromStr};

/// Reads IDL from stdin, writes JSON to stdout.
pub fn main(args: &Args, idl_str: &str) -> anyhow::Result<String> {
    let idl_args: IDLArgs = idl_str
        .parse()
        .with_context(|| anyhow!("Malformed input"))?;
    let idl2json_options = Idl2JsonOptions::default();
    let idl_type = get_idl_type(args).context("Failed to determine optional type")?;
    convert_all(&idl_args, &idl_type, &idl2json_options)
}

/// Candid typically comes as a tuple of values.  This converts a single value in such a tuple.
fn convert_one(
    idl_value: &IDLValue,
    idl_type: &Option<IDLType>,
    idl2json_options: &Idl2JsonOptions,
) -> anyhow::Result<String> {
    let json_value = if let Some(idl_type) = idl_type {
        idl2json_with_weak_names(idl_value, idl_type, idl2json_options)
    } else {
        idl2json(idl_value, idl2json_options)
    };
    serde_json::to_string(&json_value).with_context(|| anyhow!("Cannot print to stderr"))
}

/// Candid typically comes as a tuple of values.  This converts all such tuples
fn convert_all(
    idl_args: &IDLArgs,
    idl_type: &Option<IDLType>,
    idl2json_options: &Idl2JsonOptions,
) -> anyhow::Result<String> {
    let json_structures: anyhow::Result<Vec<String>> = idl_args
        .args
        .iter()
        .map(|idl_value| convert_one(idl_value, idl_type, idl2json_options))
        .collect();
    Ok(json_structures?.join("\n"))
}

/// Get the IDL type, if specified
fn get_idl_type(args: &Args) -> anyhow::Result<Option<IDLType>> {
    if let (Some(did), Some(typ)) = (&args.did, &args.typ) {
        let idl_type: IDLType = {
            let prog = {
                let did_as_str = std::fs::read_to_string(did)
                    .with_context(|| anyhow!("Could not read did file '{}'.", did.display()))?;
                IDLProg::from_str(&did_as_str)
                    .with_context(|| anyhow!("Failed to parse did file '{}'", did.display()))?
            };
            polyfill::idl_prog::get(&prog, typ).ok_or_else(|| {
                anyhow!("Type '{typ}' not found in .did file '{}'.", did.display())
            })?
        };
        Ok(Some(idl_type))
    } else {
        Ok(None)
    }
}

/// Converts Candid on stdin to JSON on stdout.
#[derive(Parser, Debug, Default)]
#[clap(name("idl2json"), version = concat!(env!("CARGO_PKG_VERSION"), "\ncandid ", env!("CARGO_CANDID_VERSION")))]
pub struct Args {
    /// A .did file containing type definitions
    #[clap(short, long)]
    did: Option<PathBuf>,
    /// The name of a type in the provided .did file
    #[clap(short, long)]
    typ: Option<String>,
}
