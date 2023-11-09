use std::env;
use std::fs::File;
use std::io::Read;
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
        println!("info: {}", args[2]);
        // Attempt to open the file
        let mut file = File::open(&args[2])?;

        // Read the contents of the file
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // Print the contents of the file
        println!("File contents:\n{}", content);


    }

    else {
        println!("unknown command: {}", args[1])
    }
    Ok(())
}
