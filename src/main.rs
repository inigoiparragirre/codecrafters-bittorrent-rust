// mod decode;
use serde_json;
use std::env;

mod decode;

use clap::Parser;

#[derive(Parser, Debug)]
struct Arguments {
    info: String,
    file_name: String,
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
        let (decoded_value, _) = crate::decode::decode_bencoded_value(encoded_value, 0);
        println!("{}", decoded_value.to_string());
    } else if command == "info" {
        println!("info: {}", args[2]);
    }

    else {
        println!("unknown command: {}", args[1])
    }
}
