use std::{env};
use anyhow::{Context, Result};
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
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1].as_str();

    // Declare the data that we will need to send to the tracker
    let mut info_hash = [0; 20];
    let mut torrent = Torrent::new();
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
                    Err(err)
                }
            }
        }
        "peers" => {
            // Read the file
            let content: &[u8] = &std::fs::read(&args[2])?;
            read_info(content, &mut info_hash, &mut torrent)?;
            match make_peer_request(&info_hash, &torrent, peer_id).await {
                Ok(_) => {
                    Ok(())
                }
                Err(err) => {
                    println!("Error making peer request: {}", err);
                    Err(err)
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

fn read_info(content: &[u8], info_hash: &mut [u8; 20], torrent: &mut Torrent) -> Result<()> {
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

async fn make_peer_request(info_hash: &[u8; 20], torrent: &Torrent, peer_id: String) -> Result<()> {
    let d = peers::TrackerRequest::default();
    const PORT: u16 = 6881;

    // URL encode the byte string
    let tracker_request = peers::TrackerRequest {
        peer_id,
        left: torrent.info.length as u64,
        port: PORT,
        ..d
    };
    // This cannot be urlencoded by serialize, it goes apart
    let encoded_info_hash = peers::url_encode(info_hash);
    // println!("{:#?}", tracker_request);
    let encoded_request = serde_urlencoded::to_string(tracker_request).context("Error encoding tracker request")?;
    // println!("Encoded request: {}", encoded_request);

    // Make request to tracker url
    let url = format!("{}?{}&info_hash={}", torrent.announce, encoded_request, encoded_info_hash);
    println!("URL: {}", url);
    let client = reqwest::Client::new().get(url);
    let result_response = client
        .send()
        .await?;
    let response_bytes = result_response.bytes().await?;

    // println!("Response bytes: {:?}", result_response.as_bytes());
    //let test_bytes = [100, 56, 58, 99, 111, 109, 112, 108, 101, 116, 101, 105, 49, 49, 101, 49, 48, 58, 105, 110, 99, 111, 109, 112, 108, 101, 116, 101, 105, 49, 101, 56, 58, 105, 110, 116, 101, 114, 118, 97, 108, 105, 49, 56, 48, 48, 101, 49, 50, 58, 109, 105, 110, 32, 105, 110, 116, 101, 114, 118, 97, 108, 105, 49, 56, 48, 48, 101, 53, 58, 112, 101, 101, 114, 115, 55, 50, 58, 106, 72, 239, 191, 189, 0, 239, 191, 189, 13, 239, 191, 189, 119, 61, 239, 191, 189, 26, 239, 191, 189, 2, 7, 239, 191, 189, 20, 239, 191, 189, 239, 191, 189, 71, 239, 191, 189, 0, 29, 239, 191, 189, 239, 191, 189, 37, 48, 74, 20, 239, 191, 189, 239, 191, 189, 82, 239, 191, 189, 239, 191, 189, 239, 191, 189, 26, 239, 191, 189, 72, 239, 191, 189, 28, 2, 239, 191, 189, 86, 45, 67, 239, 191, 189, 74, 239, 191, 189, 103, 239, 191, 189, 90, 239, 191, 189, 221, 178, 114, 66, 55, 239, 191, 189, 70, 239, 191, 189, 96, 69, 53, 20, 239, 191, 189, 239, 191, 189, 96, 239, 191, 189, 195, 129, 27, 239, 191, 189, 96, 101];

    let decoded: peers::TrackerResponseSuccess = serde_bencode::from_bytes(&response_bytes).context("Error decoding serde response")?;
    //println!("{:#?}", decoded);

    let peers = decoded.peers.0;

    for peer in peers {
        println!("{}", peer);
    }

    // match result_response {
    //     Ok(response) => {
    //         println!("Response peer request: {}", response);
    //         let decoded: peers::TrackerResponse = serde_bencode::from_str(&response).context("Error decoding serde response")?;
    //         println!("Decoded response: {:#?}", response);
    //         match decoded {
    //             peers::TrackerResponse::Success(success) => {
    //                 println!("Success: {:#?}", success);
    //             }
    //             peers::TrackerResponse::Error(error) => {
    //                 println!("Error: {:#?}", error);
    //             }
    //         }
    //     }
    //     Err(err) => {
    //         println!("Error making request: {}", err.to_string());
    //     }
    // }


    // let good_post_response = r#"d8:completei1e10:downloadedi1e10:incompletei1e8:intervali1800e12:min intervali900e5:peers12:
    // let decoded: peers::TrackerResponse = serde_bencode::from_str(&good_post_response)?;
    //

    Ok(())
}