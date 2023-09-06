// Parse nixos options from options.json / adjust formatting
extern crate nix_data;
extern crate serde_json;
use serde_json::{Map, Value};
use std::fs;

// Function to split keys in the first layer of a JSON object
fn split_first_layer_keys(json_object: &mut Value) {
    // Check if the input is an object
    if let Value::Object(object_map) = json_object {
        // Collect keys containing '.' into a vector
        let keys_to_split: Vec<String> = object_map.keys().filter(|key| key.contains('.')).cloned().collect();

        // Iterate through keys to split
        for key_to_split in keys_to_split {
            // Split key by '.' into parts
            let key_parts: Vec<&str> = key_to_split.split('.').collect();
            // Remove the original key and get its value
            let value_to_move = object_map.remove(&key_to_split).unwrap();

            // Create a mutable reference to the current map
            let mut current_map: &mut Map<String, Value> = object_map;

            // Iterate through the parts, creating nested objects as needed
            for part in key_parts.iter().take(key_parts.len() - 1) {
                // Insert a new nested object if it doesn't exist, or get a reference to the existing one
                current_map = current_map
                    .entry(part.to_string())
                    .or_insert_with(|| Value::Object(Default::default()))
                    .as_object_mut()
                    .expect("Expected an object");
            }

            // Insert the value into the last nested object
            current_map.insert(key_parts.last().unwrap().to_string(), value_to_move);
        }
    }
}

fn main() {
    // Read the contents of a file into a JSON string
    let json_str = fs::read_to_string(nix_data::cache::nixos::nixosoptions().unwrap()).unwrap();

    // Parse the JSON string into a mutable Value
    let mut parsed_json: Value =
        serde_json::from_str(json_str.as_str()).expect("Failed to parse JSON");

    // Split keys only in the first layer of the JSON object
    split_first_layer_keys(&mut parsed_json);

    // Print the modified JSON in a pretty format
    println!("{}", serde_json::to_string_pretty(&parsed_json).unwrap());
}
