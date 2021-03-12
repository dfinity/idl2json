use candid::parser::value::IDLValue;

fn main() {
    use candid::IDLArgs;
    // Candid values represented in text format
    let text_value = r#"
(
  opt record {
    id = opt record { id = 1 };
    ballots = vec {
      record {
        10_393_729_187_455_219_830;
        record { vote = 0; voting_power = 15_117_295_952_643_213_369 };
      };
      record {
        49;
        record { vote = 1; voting_power = 151_172_959_526_432_132 };
      };
      record { 0; record { vote = 1; voting_power = 201_540_946_954_225_587 } };
      record {
        50;
        record { vote = 0; voting_power = 151_172_959_526_432_132 };
      };
    };
    reject_cost_doms = 100_000_000;
    proposal_timestamp_seconds = 1_615_390_834;
    reward_event_round = 0;
    failed_timestamp_seconds = 0;
    proposal = opt record {
      url = "";
      action = opt variant {
        ExternalUpdate = record {
          update_type = 5;
          payload = blob "DIDL\01l\07\f2\ad\d0\b1\03q\be\b2\94\c2\03q\c3\d7\f3\b3\06q\c9\ef\8e\c5\09q\d4\cb\8b\ab\0cq\a0\d8\ef\ae\0cq\f1\81\93\b1\0fq\01\00@a1d71312400d163a1bec704259adbaff1fbbb904b2dab7403b8de30f02ce946a\00@e4b6cc7d77af4c8e02e7db58e2a71d481b446e612d3fbd2e38e072b7c236348e(f4fc0e60c04184d9939eddcc467c6d5404e74203\00lhttps://download.dfinity.systems/ic/f4fc0e60c04184d9939eddcc467c6d5404e74203/x86_64-linux/nodemanager.tar.gzkhttps://download.dfinity.systems/ic/f4fc0e60c04184d9939eddcc467c6d5404e74203/x86_64-linux/ic-replica.tar.gz";
        }
      };
      summary = "<proposal created from initialization>";
    };
    proposer = opt record { id = 49 };
    tally_at_decision_time = null;
    executed_timestamp_seconds = 0;
  },
)
"#;

    // Parse text format into IDLArgs for serialization
    let args: IDLArgs = text_value.parse().expect("");

    println!("{:?}", args);
    println!("{}", serde_json::to_string(&idl_to_serde(&args.args[0])).expect(""));
}

// Better way to do this:  https://github.com/sfackler/serde-transcode/blob/master/src/lib.rs
use serde_json::value::Value as JsonValue;
fn idl_to_serde(idl: &IDLValue) -> JsonValue {
    match idl {
      IDLValue::Bool(bool) => JsonValue::Bool(*bool),
      IDLValue::Null => JsonValue::Null,
      IDLValue::Text(s) => JsonValue::String(s.clone()),
      IDLValue::Number(s) => JsonValue::String(s.clone()), // Unspecified number type
      IDLValue::Float64(f) => JsonValue::Number(serde_json::Number::from_f64(*f).expect("A float's a float")),
      IDLValue::Opt(value) => JsonValue::Array(vec![idl_to_serde(value)]),
      IDLValue::Vec(value) => JsonValue::Array(value.iter().map(idl_to_serde).collect()),
      IDLValue::Record(value) => JsonValue::Object(value.iter().map(|field| (format!("{}", field.id), idl_to_serde(&field.val))).collect()),
      IDLValue::Variant(field, _index) => JsonValue::Object(vec![(format!("{}", field.id), idl_to_serde(&field.val))].into_iter().collect()),
      IDLValue::Principal(p) => JsonValue::String( format!("{}", p) ),
      IDLValue::Service(p) => JsonValue::String( format!("{}", p) ),
      IDLValue::Func(p,c) => JsonValue::Object(vec![ ("principal".to_string(), JsonValue::String(format!("{}", p))), ("code".to_string(), JsonValue::String(c.to_string())) ].into_iter().collect()),
      IDLValue::None => JsonValue::Array(vec![]),
      IDLValue::Int(i) => JsonValue::String(format!("{}", i)),
      IDLValue::Nat(i) => JsonValue::String(format!("{}", i)),
      IDLValue::Nat8(i) => JsonValue::String(format!("{}", i)),
      IDLValue::Nat16(i) => JsonValue::String(format!("{}", i)),
      IDLValue::Nat32(i) => JsonValue::String(format!("{}", i)),
      IDLValue::Nat64(i) => JsonValue::String(format!("{}", i)),
      IDLValue::Int8(i) => JsonValue::String(format!("{}", i)),
      IDLValue::Int16(i) => JsonValue::String(format!("{}", i)),
      IDLValue::Int32(i) => JsonValue::String(format!("{}", i)),
      IDLValue::Int64(i) => JsonValue::String(format!("{}", i)),
      IDLValue::Float32(f) => JsonValue::Number(serde_json::Number::from_f64(*f as f64).expect("A float's a float")),
      IDLValue::Reserved => panic!("Unimplemented: {:?}", idl),
    }
}
