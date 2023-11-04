use serde_json;
use std::env;


// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str, index: usize) -> (serde_json::Value, usize) {

    // If encoded_value starts with a i, and ends with an e and inside it's a number
    if encoded_value.starts_with('i') {
        // Example: "i52e" -> "52"
        let end_index = encoded_value.find('e').unwrap();
        let number_string = &encoded_value[1..end_index];
        let number = number_string.parse::<i64>().unwrap();
        return (serde_json::Value::Number(number.into()), number_string.len());
    }
    // If encoded value is a string, it starts with a number, followed by a colon, followed by the string
    else if encoded_value.chars().next().unwrap().is_digit(10) {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        return (serde_json::Value::String(string.to_string()), string.len());
    }
    // If encoded value is a list, it starts with an l, ends with an e, and contains encoded values
    else if encoded_value.starts_with('l') && encoded_value.ends_with('e') {
        // Example: "l5:helloi52ee" -> ["hello", 52]
        let mut list = Vec::new();
        // We need to remove the l and e from the encoded value
        // let inside_encoded_value = &encoded_value[1..encoded_value.len() - 1];
        let mut current_index = index + 1;
        while current_index < encoded_value.len() {
            let current_value = &encoded_value[current_index..];
            // println!("current_value: {}", current_value);
            let (decoded_value, _index) = decode_bencoded_value(current_value, current_index);
            // println!("decoded_value: {}", decoded_value.to_string());
            // match decoded_value {
            //     // Fix current index values for each case
            //     serde_json::Value::Number(_) => current_index += decoded_value.to_string().len() + 2,
            //     serde_json::Value::String(_) => current_index += decoded_value.to_string().len(),
            //     serde_json::Value::Array(_) => current_index += decoded_value.to_string().len() + 1,
            //     _ => panic!("Unhandled decoded value: {}", decoded_value)
            // }

            list.push(decoded_value);

        }
        return (serde_json::Value::Array(list), 0);
    }
    else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // println!("Logs from your program will appear here!");
        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let (decoded_value, _) = decode_bencoded_value(encoded_value, 0);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
