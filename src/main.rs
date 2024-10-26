use serde_json::{self};
use std::env;

// Available if you need it!
// use serde_bencode

fn decode_string(encoded_value: &str) -> String {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index + 1..=colon_index + number as usize];
    string.to_string()
}

fn decode_int(encoded_value: &str) -> i64 {
    let e_idx = encoded_value.find('e').unwrap();
    let str = &encoded_value[1..e_idx];
    str.parse().unwrap()
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    let decoded: serde_json::Value = match encoded_value.chars().next().unwrap() {
        val if val.is_digit(10) => serde_json::Value::String(decode_string(encoded_value)),
        val if val == 'i' => decode_int(encoded_value).into(),
        _ => panic!("Unhandled encoded value: {}", encoded_value),
    };

    decoded
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
