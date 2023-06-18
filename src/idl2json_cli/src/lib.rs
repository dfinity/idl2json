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
use candid::{
    parser::types::{IDLType, IDLTypes},
    IDLArgs, IDLProg,
};
use clap::Parser;
use idl2json::{
    idl2json, idl2json_with_weak_names, idl_args2json_with_weak_names, polyfill, BytesFormat,
    Idl2JsonOptions,
};
use std::{path::PathBuf, str::FromStr};

/// Reads IDL from stdin, writes JSON to stdout.
pub fn main(args: &Args, idl_str: &str) -> anyhow::Result<String> {
    let idl_args: IDLArgs = idl_str
        .parse()
        .with_context(|| anyhow!("Malformed input"))?;
    let idl2json_options = {
        let progs: anyhow::Result<Vec<IDLProg>> = args
            .did
            .iter()
            .map(|did| {
                let did_as_str = std::fs::read_to_string(did)
                    .with_context(|| anyhow!("Could not read did file '{}'.", did.display()))?;
                IDLProg::from_str(&did_as_str)
                    .with_context(|| anyhow!("Failed to parse did file '{}'", did.display()))
            })
            .collect();
        let progs = progs?;

        Idl2JsonOptions {
            prog: progs,
            bytes_as: args.bytes_as,
            ..Idl2JsonOptions::default()
        }
    };
    // Decide what to do
    if args.init {
        // Use the type of the .did file init arg.
        // - If multiple did files are provided, the first is used.
        // - Clap should reject commands without a --did file.
        let idl_types = polyfill::idl_prog::get_init_arg_type(
            idl2json_options
                .prog
                .get(0)
                .context("Please specify which .did file to use.")?,
        )
        .context("Failed to get the service argument from the did file.")?;
        serde_json::to_string(&idl_args2json_with_weak_names(
            &idl_args,
            &idl_types,
            &idl2json_options,
        ))
        .context("Failed to serialize to json")
    } else if let Some(idl_type) = &args.typ {
        if idl_type.trim().starts_with('(') {
            let idl_types = IDLTypes::from_str(idl_type).context("Failed to parse type")?;
            serde_json::to_string(&idl_args2json_with_weak_names(
                &idl_args,
                &idl_types,
                &idl2json_options,
            ))
            .context("Failed to serialize to json")
        } else {
            let idl_type = IDLType::from_str(idl_type).context("Failed to parse type")?;
            convert_all(&idl_args, &Some(idl_type), &idl2json_options)
        }
    } else {
        convert_all(&idl_args, &None, &idl2json_options)
    }
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

/// Converts Candid on stdin to JSON on stdout.
#[derive(Parser, Debug, Default)]
#[clap(name("idl2json"), version = concat!(env!("CARGO_PKG_VERSION"), "\ncandid ", env!("CARGO_CANDID_VERSION")))]
pub struct Args {
    /// A .did file containing type definitions
    #[clap(short, long)]
    did: Vec<PathBuf>,
    /// The name of a type in the provided .did file
    #[clap(short, long)]
    typ: Option<String>,
    /// Use the service init argument type from the did file
    #[clap(short, long, requires("did"))]
    init: bool,
    /// How to display bytes
    #[clap(short, long, arg_enum)]
    bytes_as: Option<BytesFormat>,
}
