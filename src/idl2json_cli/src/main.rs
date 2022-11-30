use idl2json_cli as lib;

/// Reads IDL from stdin, writes JSON to stdout.
fn main() {
    lib::main().expect("Failed to convert IDL to JSON");
}
