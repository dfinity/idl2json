use candid::IDLArgs;
use std::io::{self, Read};
use idl2json::idl_to_serde;

/// Reads IDL from stdin, writes JSON to stdout.
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: IDLArgs = buffer.parse().expect("Malformed input");
    println!(
        "{}",
        serde_json::to_string(&idl_to_serde(&args.args[0])).expect("Cannot get it out")
    );

    Ok(())
}
