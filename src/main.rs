use std::result::Result as stdResult;
use std::error::Error;
use std::{env};
use clap::Parser;
use crate::value::BencodeValue;


mod decode;
mod value;
mod torrent;
mod error;
mod encode;


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


    match *command {
        "decode" => {
            let mut parser = decode::Parser::new(&encoded_bytes);
            match parser.parse() {
                Ok(decoded_value) => {
                    println!("{}", decoded_value);
                    Ok(())
                }
                Err(err) => {
                    println!("Error decoding: {}", err);
                    Err(err.into())
                }
            }
        }
        "info" => {
            // Read the file
            let content: &[u8] = &std::fs::read(&args[2])?;
            let mut parser = decode::Parser::new(&content);
            match parser.parse() {
                Ok(decoded_value) => {
                    if let BencodeValue::BDictionary(map) = decoded_value {
                        if let Some(url) = map.get("announce".as_bytes()) {
                            let url_string = format!("{}", url);
                            let output = url_string.trim_matches('"'); // Remove quotes
                            println!("Tracker URL: {}", output);
                        }

                        if let Some(info) = map.get("info".as_bytes()) {
                            if let BencodeValue::BDictionary(map) = info {
                                if let Some(length) = map.get("length".as_bytes()) {
                                    println!("Length: {}", length);
                                }

                                if let Some(length) = map.get("piece length".as_bytes()) {
                                    println!("Piece Length: {}", length);
                                }

                                let mut encoder = encode::Encoder::new();
                                encoder.encode(info)?;
                                let hash = encoder.encode_hex();
                                println!("Info Hash: {}", hash);

                                if let Some(pieces_string) = map.get("pieces".as_bytes()) {
                                    // Get the bytes string and represent as hexadecimal
                                    // Represent hexadecimal hash of each piece
                                    if let BencodeValue::BString(pieces_string) = pieces_string {
                                        let mut remaining_hash_data = &pieces_string[..];
                                        while !remaining_hash_data.is_empty() {
                                            let (hash, rest) = remaining_hash_data.split_at(20);
                                            remaining_hash_data = rest;
                                            let hash_in_hex = hex::encode(hash);
                                            println!("{}", hash_in_hex);

                                        }
                                    }


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