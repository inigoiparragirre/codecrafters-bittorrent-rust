use std::{env};
use std::net::{SocketAddr, TcpStream};
use anyhow::{Context, Result};
use clap::Parser;
use crate::value::BencodeValue;
use crate::torrent::Torrent;
use std::io::{Read, Write};
use peers::Handshake;
use tracker::{TrackerResponseSuccess, TrackerRequest};
use crate::peers::addr::Address;


mod decode;
mod value;
mod torrent;
mod error;
mod encode;
mod peers;
mod tracker;


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
        "info" => {
            // Read the file
            let content: &[u8] = &std::fs::read(&args[2])?;
            read_info(content, &mut info_hash, &mut torrent, true)
        }
        "peers" => {
            // Read the file
            let content: &[u8] = &std::fs::read(&args[2])?;
            read_info(content, &mut info_hash, &mut torrent, true)?;
            make_peer_request(&info_hash, &torrent, peer_id).await.context("Error making peer request")?;
            Ok(())

        }
        "handshake" => {
            // Read the file
            let content: &[u8] = &std::fs::read(&args[2])?;
            let socket_addr = args[3].parse::<SocketAddr>().context("Error parsing socket address")?;
            read_info(content, &mut info_hash, &mut torrent, false)?;
            make_handshake(&info_hash, &socket_addr).context("Error making handshake")?;
            Ok(())

        }
        "download_piece" => {
            // Once the handshake is complete, we send a bitfield message


            // We send an interested message
            todo!("Send interested message");
            // Wait to receive an unchocke message back
            todo!("Wait to receive unchoke message");

            todo!("Break the piece into blocks of 16kiB");

            todo!("Wait for a piece message for each block requested");

            todo!("Check the integrity of each piece block");

            Ok(())
        }
        _ => {
            println!("unknown command: {}", args[1]);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "unknown command").into())
        }
    }
}

fn make_handshake(info_hash: &[u8;20], socket_addr: &SocketAddr) -> Result<()> {
    if let Ok(mut stream) = TcpStream::connect(socket_addr) {
        let handshake = Handshake::new(*info_hash, *b"00112233445566778899");
        let serialized_bytes = bincode::serialize(&handshake).expect("Serialization failed for handshake");
        //println!("Serialized: {:?}", serialized_bytes);
        stream.write_all(&serialized_bytes).expect("Error writing to stream");

        // Read data from the stream
        let mut buffer = [0; 256];
        stream.read(&mut buffer).expect("Error reading from stream");
        let handshake_response: Handshake = bincode::deserialize(&buffer).expect("Error deserializing handshake");
        println!("Peer ID: {}", handshake_response.peer_id.iter().map(|b| format!("{:02x}", b)).collect::<String>());
        Ok(())
    }
    else {
        println!("Error connecting to socket address");
        anyhow::bail!("Error connecting to socket address");
    }
}

fn read_info(content: &[u8], info_hash: &mut [u8; 20], torrent: &mut Torrent, print: bool) -> Result<()> {
    let mut parser = decode::Parser::new(&content);
    match parser.parse() {
        Ok(decoded_value) => {
            if let BencodeValue::BDictionary(map) = decoded_value {
                if let Some(url) = map.get("announce".as_bytes()) {
                    let url_string = format!("{}", url);
                    let output = url_string.trim_matches('"'); // Remove quotes
                    torrent.announce = output.to_string();
                    if print {println!("Tracker URL: {}", output);}
                }

                if let Some(info) = map.get("info".as_bytes()) {
                    if let BencodeValue::BDictionary(map) = info {
                        if let Some(length) = map.get("length".as_bytes()) {
                            if print {println!("Length: {}", length);}
                            torrent.info.length = length.to_string().parse::<i64>().unwrap();
                        }

                        if let Some(length) = map.get("piece length".as_bytes()) {
                            if print {println!("Piece Length: {}", length);}
                            torrent.info.piece_length = length.to_string().parse::<i64>().unwrap();
                        }

                        let mut encoder = encode::Encoder::new();
                        encoder.encode(info)?;
                        let hash = encoder.encode_sha1(info_hash);
                        if print {println!("Info Hash: {}", hash);}


                        if let Some(pieces_string) = map.get("pieces".as_bytes()) {
                            // Get the bytes string and represent as hexadecimal
                            // Represent hexadecimal hash of each piece
                            if let BencodeValue::BString(pieces_string) = pieces_string {
                                if print {println!("Piece Hashes:");}
                                let mut remaining_hash_data = &pieces_string[..];
                                while !remaining_hash_data.is_empty() {
                                    let (hash, rest) = remaining_hash_data.split_at(20);
                                    remaining_hash_data = rest;
                                    let hash_in_hex = hex::encode(hash);
                                    if print {println!("{}", hash_in_hex);}
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
    let d = TrackerRequest::default();
    const PORT: u16 = 6881;

    // URL encode the byte string
    let tracker_request = TrackerRequest {
        peer_id,
        left: torrent.info.length as u64,
        port: PORT,
        ..d
    };
    // This cannot be urlencoded by serialize, it goes apart
    let encoded_info_hash = url_encode(info_hash);
    // println!("{:#?}", tracker_request);
    let encoded_request = serde_urlencoded::to_string(tracker_request).context("Error encoding tracker request")?;
    // println!("Encoded request: {}", encoded_request);

    // Make request to tracker url
    let url = format!("{}?{}&info_hash={}", torrent.announce, encoded_request, encoded_info_hash);
    println!("URL: {}", url);
    let client = reqwest::Client::new().get(url);
    let response_bytes = client
        .send()
        .await?
        .bytes().await?;
    let TrackerResponseSuccess {
        peers: Address(list_peers), ..
    } = serde_bencode::from_bytes(&response_bytes).context("Error decoding serde response")?;

    for peer in list_peers {
        println!("{}", peer);
    }

    Ok(())
}

pub fn url_encode(info_hash: &[u8; 20]) -> String {
    let mut url_encoded = String::new();
    for byte in info_hash {
        url_encoded.push_str(&format!("%{:02x}", byte));
    }
    url_encoded
}