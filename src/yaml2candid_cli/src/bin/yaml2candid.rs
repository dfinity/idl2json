use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use yaml2candid::Yaml2Idl;

/// Converts YAML to Candid
#[derive(Parser, Debug)]
#[clap(name("yaml2candid"))]
struct Args {
    /// Path to the interface definition `.did` file.
    #[clap(short, long, required = true)]
    did: String,

    /// The name of the IDL type.
    #[clap(short, long, required = true)]
    typ: String,

    /// The path to the YAML file to be converted.  (Default: stdin)
    #[clap(short, long)]
    yml: Option<String>,
}

fn main() {
    let args = Args::parse();
    let converter = Yaml2Idl::from_did_file(&args.did).unwrap();
    let yaml_str = file2string(args.yml);
    let candid = converter.convert_str(&args.typ, &yaml_str).unwrap();
    println!("{candid}")
}

fn file2string(filename: Option<String>) -> String {
    let mut contents: Vec<u8> = Vec::new();
    let mut reader: Box<dyn BufRead> = match filename {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
    };
    reader.read_to_end(&mut contents).unwrap();
    String::from_utf8(contents).unwrap()
}
