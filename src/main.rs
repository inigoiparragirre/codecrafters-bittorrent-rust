extern crate core;

use std::result::Result as stdResult;
use std::error::Error;
use std::{env};


mod decode;
// pub mod de;
mod value;
mod torrent;
mod error;


use clap::Parser;
use crate::value::BencodeValue;

#[derive(Parser, Debug)]
struct Arguments {
    info: String,
    file_name: String,
}


// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() -> stdResult<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1].as_str();
    let encoded_bytes = &args[2].as_bytes();
    let mut parser = decode::Parser::new(&encoded_bytes);

    match *command {
        "decode" => {
            match parser.parse() {
                Ok(decoded_value) => {
                    println!("Decode: {}", decoded_value);
                    Ok(())
                }
                Err(err) => {
                    println!("Error decoding: {}", err);
                    Err(err.into())
                }
            }
        }
        "info" => {
            // Get valid string characters
            // let content: &[u8] = &fs::read(&args[2])?;
            match parser.parse() {
                Ok(decoded_value) => {
                    if let BencodeValue::BDictionary(map) = decoded_value {
                        if let Some(url) = map.get("announce".as_bytes()) {
                            println!("Tracker URL: {}", url);
                        }

                        if let Some(info) = map.get("info".as_bytes()) {
                            if let BencodeValue::BDictionary(map) = info {
                                if let Some(length) = map.get("length".as_bytes()) {
                                    println!("Length: {}", length);
                                }

                            }
                        }

                    }
                    Ok(())
                }
                Err(err) => {
                    println!("Error decoding info: {}", err);
                    Err(err.into())
                }
            }
        }
        _ => {
            println!("unknown command: {}", args[1]);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "unknown command")))
        }
    }
}