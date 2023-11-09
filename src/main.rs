use std::{env, fs};
use std::error::Error;

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
        // println!("Logs from your program will appear here!");
        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let (decoded_value, _) = crate::decode::decode_bencoded_value(encoded_value, 0);
        // This is necessary to print for the tests
        println!("{}", decoded_value.to_string());
    } else if command == "info" {

        // Get valid string characters
        let encoded_torrent_content: String = String::from_utf8_lossy(&fs::read(&args[2])?).parse()?;
        let (decoded_value, _) = crate::decode::decode_bencoded_value(&encoded_torrent_content, 0);

        println!("{}", decoded_value.to_string());




    }

    else {
        println!("unknown command: {}", args[1])
    }
    Ok(())
}