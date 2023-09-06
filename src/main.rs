extern crate nix_data;
extern crate regex;
extern crate serde;
extern crate serde_json;

use std::fs;

use serde::{Deserialize, Serialize};

use serde_json::{Value, Map};

// TODO: need to get options out of the root object without knowing the key
// #[derive(Serialize, Deserialize, Debug)]
// struct Json { option: _Option }

// #[derive(Serialize, Deserialize, Debug)]
// struct _Option {
//     declarations: Vec<String>,
//     default: _Default,
//     description: Description,
//     loc: Vec<String>,
//     read_only: bool,
//     _type: String,
// }
// #[derive(Serialize, Deserialize, Debug)]
// struct _Default {
//     _type: String,
//     text: String,
// }
// 
// #[derive(Serialize, Deserialize, Debug)]
// struct Description {
//     _type: String,
//     text: String,
// }

fn split_first_layer_keys(obj: &mut Value) {
    if let Value::Object(map) = obj {
        let keys_to_split: Vec<String> = map.keys().filter(|k| k.contains('.')).cloned().collect();
        
        for key in keys_to_split {
            let parts: Vec<&str> = key.split('.').collect();
            let value = map.remove(&key).unwrap();
            
            let mut current_map = map;
            for part in parts.iter().take(parts.len() - 1) {
             &mut current_map = &mut current_map
                    .entry(part.to_string())
                    .or_insert(Value::Object(Default::default()))
                    .as_object_mut()
                    .expect("Expected an object");
            }
            
            current_map.insert(parts.last().unwrap().to_string(), value.clone());
        }
    }
}
fn main() {

    let option_json = fs::read_to_string(nix_data::cache::nixos::nixosoptions().unwrap()).unwrap();

    let mut parsed_json: Value = serde_json::from_str(&option_json.as_str()).unwrap();

    split_first_layer_keys(&mut parsed_json);

    println!("{}", serde_json::to_string_pretty(&parsed_json).unwrap());


}
