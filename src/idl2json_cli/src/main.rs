use clap::Parser;
use idl2json_cli as lib;

/// Reads IDL from stdin, writes JSON to stdout.
fn main() {
    let args = lib::Args::parse();
    lib::main(&args).expect("Failed to convert IDL to JSON");
}
