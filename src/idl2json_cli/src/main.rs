use candid::IDLArgs;
use idl2json::{idl2json, Idl2JsonOptions};
use std::io::{self, Read};

/// Reads IDL from stdin, writes JSON to stdout.
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: IDLArgs = buffer.parse().expect("Malformed input");
    println!(
        "{}",
        serde_json::to_string(&idl2json(&args.args[0], &Idl2JsonOptions::default())).expect("Cannot get it out")
    );

    Ok(())
}
