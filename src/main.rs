use std::{env, fs};
use std::error::Error;
use std::ops::Add;
use serde_json::Value;

mod decode;

use clap::Parser;

#[derive(Parser, Debug)]
struct Arguments {
    info: String,
    file_name: String,
}


// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        println!("Logs from your program will appear here!");
        let encoded_value = &args[2];
        let (decoded_value, _) = decode::decode_bencoded_value(encoded_value, 0);
        // This is necessary to print for the tests
        println!("{}", decoded_value.to_string());
    } else if command == "info" {

        // Get valid string characters
        let encoded_torrent_content: String = String::from_utf8_lossy(&fs::read(&args[2])?).parse()?;
        // Remove pieces
        let clean_encoded_content = encoded_torrent_content.split("6:pieces").collect::<Vec<&str>>()[0];
        let clean_string = String::from(clean_encoded_content);
        let content = clean_string.add("ee");

        let (decoded_value, _) = decode::decode_bencoded_value(&content, 0);
        let url = decoded_value.get("announce").unwrap().clone();
        let key_url: String = serde_json::from_value(url).unwrap();

        let length = decoded_value.get("info").unwrap().get("length").unwrap().clone();
        let key_length: Value = serde_json::from_value(length).unwrap();
        println!("Tracker URL: {}", key_url);
        println!("Length: {}", key_length);


    }

    else {
        println!("unknown command: {}", args[1])
    }
    Ok(())
}