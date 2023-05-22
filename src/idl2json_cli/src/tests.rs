use super::{main, Args};
use anyhow::{anyhow, Context};

#[test]
fn simple_conversion_should_be_correct() {
   struct TestVector {
     stdin: &'static str, stdout: &'static str
   }
   let args = Args::default();
    let vectors = [
     TestVector{ stdin: "()", stdout: "\n" }
     TestVector{ stdin: "( record {} )", stdout: "{}\n" }
     TestVector{ stdin: "( record {}, record {} )", stdout: "{}\n{}\n" }
  ];
  for vector in vectors {
      let out = main(&args, vector.stdin).map_err(|e| anyhow!("Failed to parse: {} due to: {e}", vector.stdin)).unwrap();
    assert_eq!(vector.stdout, &out)
  }
}
