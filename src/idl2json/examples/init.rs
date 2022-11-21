use core::str;
use std::str::FromStr;
use candid::{IDLProg, parser::{types::{Dec, IDLType}, value::IDLValue}, Decode};
use hex::FromHex;
use idl2json::{idl2json, idl2json_with_weak_names};

/// Converts some sample bytes
fn main() {
   let type_name = "InternetIdentityInit";
   let prog = {
      let did_as_str = std::fs::read_to_string("../../samples/internet_identity.did").expect("Could not read did file");
      IDLProg::from_str(&did_as_str).expect("Failed to parse did")
   };
   let idl_type = prog
                    .decs
                    .iter()
                    .find_map(|x| {
                        if let Dec::TypD(y) = x {
                            if y.id == type_name {
                                return Some(y.typ.clone());
                            }
                        }
                        None
                    }).expect("Failed to get idltype");
   let idl_type = IDLType::OptT(Box::new(idl_type));
   println!("Type: {:?}\n\n", &idl_type);
   let hex_bytes = "4449444C056E016C02C488BFD70102F7F5CBFB07046E036D7B6E780100010120F691F269DD66AA4FC44E6916AEFEE03BB7FEB821AEF43467526974F470CD4B07010010A5D4E8000000";
   //let buffer = <[u8; 12]>::from_hex(hex_bytes);
   let buffer = [68,73,68,76,5,110,1,108,2,196,136,191,215,1,2,247,245,203,251,7,4,110,3,109,123,110,120,1,0,1,1,32,246,145,242,105,221,102,170,79,196,78,105,22,174,254,224,59,183,254,184,33,174,244,52,103,82,105,116,244,112,205,75,7,1,0,16,165,212,232,0,0,0];
   println!("data: {:?}\n\n", &buffer);
   //let idl_value: (IDLValue) = buffer[..].parse().expect("Could not parse bytes");
   let idl_value = Decode!(&buffer[..], IDLValue).expect("Failed to parse buffer");
   println!("Value: {:?}\n\n", idl_value);
   println!("Untyped conversion: {:?}\n\n", idl2json(&idl_value));

   println!("Typed conversion: {}\n\n", serde_json::to_string(&idl2json_with_weak_names(&idl_value, &idl_type)).expect("Failed to stringify"));
}