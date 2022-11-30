use candid::IDLArgs;
use clap::Parser;
use idl2json::{idl2json, Idl2JsonOptions};
use std::io::{self, Read};

/// Reads IDL from stdin, writes JSON to stdout.
pub fn main() -> io::Result<()> {
    let _args = Args::parse();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let idl_args: IDLArgs = buffer.parse().expect("Malformed input");
    println!(
        "{}",
        serde_json::to_string(&idl2json(&idl_args.args[0], &Idl2JsonOptions::default()))
            .expect("Cannot get it out")
    );

    Ok(())
}

/// Converts Candid on stdin to JSON on stdout.
#[derive(Parser, Debug)]
#[clap(name("yaml2candid"), version = concat!(env!("CARGO_PKG_VERSION"), "\ncandid ", env!("CARGO_CANDID_VERSION")))]
struct Args {}
