use std::result::Result as stdResult;
use std::error::Error;
use std::{env};
use clap::Parser;
use crate::value::BencodeValue;
use crate::torrent::Torrent;



mod decode;
mod value;
mod torrent;
mod error;
mod encode;
mod peers;


#[derive(Parser, Debug)]
struct Arguments {
    info: String,
    file_name: String,
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
#[tokio::main]
async fn main() -> stdResult<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1].as_str();

    // Declare the data that we will need to send to the tracker
    let mut info_hash: Vec<u8> = Vec::new();
    let mut torrent = torrent::Torrent::new();
    let peer_id = "00112233445566778899".to_string();


    match *command {
        "decode" => {
            let encoded_bytes = &args[2].as_bytes();
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
        "peers" => {
            // Read the file
            let content: &[u8] = &std::fs::read(&args[2])?;
            read_info(content, &mut info_hash, &mut torrent)?;
            match make_peer_request(info_hash, &torrent, peer_id).await {
                Ok(_) => {
                    Ok(())
                }
                Err(err) => {
                    println!("Error making peer request: {}", err);
                    Err(err.into())
                }
            }
        }
        "info" => {
            // Read the file
            let content: &[u8] = &std::fs::read(&args[2])?;
            read_info(content, &mut info_hash, &mut torrent)
        }
        _ => {
            println!("unknown command: {}", args[1]);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "unknown command").into())
        }
    }
}

fn read_info(content: &[u8], info_hash: &mut Vec<u8>, torrent: &mut Torrent) -> stdResult<(), Box<dyn Error>> {
    let mut parser = decode::Parser::new(&content);
    match parser.parse() {
        Ok(decoded_value) => {
            if let BencodeValue::BDictionary(map) = decoded_value {
                if let Some(url) = map.get("announce".as_bytes()) {
                    let url_string = format!("{}", url);
                    let output = url_string.trim_matches('"'); // Remove quotes
                    torrent.announce = output.to_string();
                    println!("Tracker URL: {}", output);
                }

                if let Some(info) = map.get("info".as_bytes()) {
                    if let BencodeValue::BDictionary(map) = info {
                        if let Some(length) = map.get("length".as_bytes()) {
                            println!("Length: {}", length);
                            torrent.info.length = length.to_string().parse::<i64>().unwrap();
                        }

                        if let Some(length) = map.get("piece length".as_bytes()) {
                            println!("Piece Length: {}", length);
                            torrent.info.piece_length = length.to_string().parse::<i64>().unwrap();
                        }

                        let mut encoder = encode::Encoder::new();
                        encoder.encode(info)?;
                        let hash = encoder.encode_sha1(info_hash);
                        println!("Info Hash: {}", hash);



                        if let Some(pieces_string) = map.get("pieces".as_bytes()) {
                            // Get the bytes string and represent as hexadecimal
                            // Represent hexadecimal hash of each piece
                            if let BencodeValue::BString(pieces_string) = pieces_string {
                                println!("Piece Hashes:");
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

async fn make_peer_request(info_hash: Vec<u8>, torrent: &Torrent, peer_id: String) -> Result<(), Box<dyn Error>> {
    let d = peers::TrackerRequest::default();

    // URL encode the byte string
    let url_encoded = percent_encoding::percent_encode(&info_hash, percent_encoding::NON_ALPHANUMERIC);
    //println!("Encoded Info Hash: {}", url_encoded);

    let tracker_request = peers::TrackerRequest {
        info_hash: url_encoded.to_string(),
        peer_id,
        left: torrent.info.length as u64,
        port: 6881,
        ..d
    };
    // println!("{:#?}", tracker_request);

    // Make request to tracker url
    let post_response =
        reqwest::Client::new()
            .get(&torrent.announce)
            .query(&tracker_request)
            .send()
            .await?
            .text()
            .await?;

    println!("Response peer request: {}", post_response);

    let good_post_response = "d8:intervali60e12:min intervali60e5:peers18:�>RY���!M��>U�!8:completei3e10:incompletei0ee";

    let decoded: peers::TrackerResponse = serde_bencode::from_str(&good_post_response)?;

    println!("{:#?}", decoded);
    Ok(())
}