extern crate core;

use std::{env};
use serde_json::Value;


mod decode;
// pub mod de;
mod value;
mod torrent;
mod error;

use error::Result;
use clap::Parser;

#[derive(Parser, Debug)]
struct Arguments {
    info: String,
    file_name: String,
}


// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1].as_str();
    let encoded_bytes = &args[2].as_bytes();

    let parser = decode::Parser::new(&buffer);

    match *command {
        "decode" => {
            match parser.parse(*encoded_bytes) {
                Ok((decoded_value, _)) => {
                    println!("{}", decoded_value.to_string());
                    Ok(())
                }
                Err(err) => {
                    println!("Error decoding: {}", err);
                    Err(err)
                }
            }

        }
        "info" => {
            // Get valid string characters
            // let content: &[u8] = &fs::read(&args[2])?;

            match decode::decode_bencoded_value(&args[2].as_bytes(), 0) {
                Ok((decoded_value, _)) => {
                    let url = decoded_value.get("announce").unwrap().clone();
                    let key_url: String = serde_json::from_value(url).unwrap();

                    let length = decoded_value.get("info").unwrap().get("length").unwrap().clone();
                    let key_length: Value = serde_json::from_value(length).unwrap();
                    println!("Tracker URL: {}", key_url);
                    println!("Length: {}", key_length);
                    Ok(())
                }
                Err(err) => {
                    println!("Error decoding info: {}", err);
                    Err(err)
                }
            }


        }
        _ => {
            println!("unknown command: {}", args[1]);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "unknown command")))
        }

    }

}