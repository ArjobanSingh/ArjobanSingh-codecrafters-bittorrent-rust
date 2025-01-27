use serde_json::{self};
use std::env;

// Available if you need it!
// use serde_bencode

enum DecodedType {
    String(String),
    Number(i64),
    List(Vec<DecodedType>),
}

fn get_colon_idx_and_num(encoded_value: &str) -> (usize, usize) {
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number: usize = number_string.parse().unwrap();
    (colon_index, number)
}

fn decode_string(encoded_value: &str) -> DecodedType {
    // Example: "5:hello" -> "hello"
    let (colon_index, number) = get_colon_idx_and_num(encoded_value);
    let string = &encoded_value[colon_index + 1..=colon_index + number as usize];
    DecodedType::String(string.to_string())
}

fn decode_int(encoded_value: &str) -> DecodedType {
    let i_idx = encoded_value.find('i').unwrap();
    let e_idx = encoded_value.find('e').unwrap();
    let str = &encoded_value[i_idx + 1..e_idx];
    DecodedType::Number(str.parse().unwrap())
}

fn decode_list(encoded_value: &str) -> (DecodedType, usize) {
    // Remove first(l) and last(e) char
    let mut iter = encoded_value.chars().peekable();
    iter.next(); // move iterator to skip 'l'

    let mut iter_idx: usize = 1;
    let mut result: Vec<DecodedType> = Vec::new();

    while let Some(&value) = iter.peek() {
        match value {
            val if val.is_digit(10) => {
                let str_chunk = &encoded_value[iter_idx..];

                let (colon_index, number_of_chars) = get_colon_idx_and_num(str_chunk);
                result.push(decode_string(str_chunk));

                let end_idx = colon_index + number_of_chars;

                // Eg: "l5:worldi54ee" -> "5:worldi54e";
                for _ in 0..=end_idx {
                    iter.next();
                    iter_idx += 1;
                }
            }
            val if val == 'i' => {
                let str_chunk = &encoded_value[iter_idx..];
                result.push(decode_int(str_chunk));
                let end_idx = str_chunk.find('e').unwrap();

                for _ in 0..=end_idx {
                    iter.next();
                    iter_idx += 1;
                }
            }
            val if val == 'l' => {
                let (nested_result, end_idx) = decode_list(&encoded_value[iter_idx..]);
                result.push(nested_result);
                for _ in 0..end_idx {
                    iter.next();
                    iter_idx += 1;
                }
            }
            val if val == 'e' => {
                iter_idx += 1;
                break;
            }
            _ => {
                println!(
                    "What is failed value: {} and encoded: {}",
                    value, encoded_value
                );
                panic!("Unhandled encoded value inside list: {}", value);
            }
        }
    }

    (DecodedType::List(result), iter_idx)
}

fn decode_type_to_serde_json(decoded_type: &DecodedType) -> serde_json::Value {
    match decoded_type {
        DecodedType::String(val) => serde_json::Value::String(val.to_string()),
        DecodedType::Number(val) => (*val).into(),
        DecodedType::List(list) => {
            let vec: Vec<serde_json::Value> = list
                .iter()
                .map(|decoded_type| decode_type_to_serde_json(decoded_type))
                .collect();
            vec.into()
        },
    }
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    let decoded_type = match encoded_value.chars().next().unwrap() {
        val if val.is_digit(10) => decode_string(encoded_value),
        val if val == 'i' => decode_int(encoded_value),
        val if val == 'l' => decode_list(encoded_value).0,
        _ => panic!("Unhandled encoded value: {}", encoded_value),
    };

    decode_type_to_serde_json(&decoded_type)
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
