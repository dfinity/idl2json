use clap::Parser;
use idl2json_cli as lib;
use std::io::{self, Read};

/// Reads IDL from stdin, writes JSON to stdout.
fn main() {
    let args = lib::Args::parse();
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read string from stdin");
    let json_str = lib::main(&args, &buffer).expect("Failed to convert IDL to JSON");
    println!("{json_str}");
}
