use tokio::net::TcpStream;
use std::net::SocketAddr;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use crate::value::BencodeValue;
use crate::torrent::Torrent;


use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_util::codec::{Framed};
use peers::Handshake;
use tracker::{TrackerResponseSuccess, TrackerRequest};
use crate::frame::MessageDecoder;
use crate::peers::addr::Address;
use futures_util::{StreamExt, SinkExt};
use bytes::{BytesMut, BufMut};
use crate::peers::{PeerMessage, PeerMessageType};

const BLOCK_SIZE: i64 = 16 * 1024;

mod decode;
mod value;
mod torrent;
mod error;
mod encode;
mod peers;
mod tracker;
mod frame;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[clap(rename_all = "snake_case")]
enum Commands {
    Decode {
        encoded_value: String,
    },
    Info {
        file: String,
    },
    Peers {
        file: String,
    },
    Handshake {
        file: String,
        socket_addr: String,
    },
    DownloadPiece {
        #[clap(short, long)]
        output: PathBuf,
        file: PathBuf,
        piece_index: u32,

    },
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
#[tokio::main]
async fn main() -> Result<()> {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Declare the data that we will need to send to the tracker
    let mut info_hash = [0; 20];
    let mut torrent = Torrent::new();
    let peer_id = "00112233445566778899".to_string();


    match cli.command {
        Commands::Decode { encoded_value } => {
            let encoded_bytes = encoded_value.as_bytes();
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
        Commands::Info {
            file,
        } => {
            // Read the file
            let content: &[u8] = &std::fs::read(file)?;
            read_info(content, &mut info_hash, &mut torrent, true)
        }
        Commands::Peers {
            file,
        } => {
            // Read the file
            let content: &[u8] = &std::fs::read(file)?;
            read_info(content, &mut info_hash, &mut torrent, true)?;
            make_peer_request(&info_hash, &torrent, peer_id).await.context("Error making peer request")?;
            Ok(())
        }
        Commands::Handshake {
            file,
            socket_addr,
        } => {
            // Read the file
            let content: &[u8] = &std::fs::read(file)?;
            let socket_addr = socket_addr.parse::<SocketAddr>().context("Error parsing socket address")?;
            read_info(content, &mut info_hash, &mut torrent, false)?;
            let mut stream = TcpStream::connect(socket_addr).await?;
            make_handshake(&mut stream, &info_hash).await.context("Error making handshake")?;
            Ok(())
        }
        Commands::DownloadPiece {
            output: _output,
            file,
            piece_index,
        } => {
            // Read the file
            let content: &[u8] = &std::fs::read(file)?;

            read_info(content, &mut info_hash, &mut torrent, false)?;
            let peers = make_peer_request(&info_hash, &torrent, peer_id).await.context("Error making peer request")?;
            let mut stream = TcpStream::connect(&peers[..]).await?;
            make_handshake(&mut stream, &info_hash).await.context("Error making handshake")?;

            //
            let mut framed = Framed::new(stream, MessageDecoder);
            let message_bitfield =  framed.next().await.expect("Expect a bitfield message").context("Error reading bitfield message")?;
            assert_eq!(message_bitfield.id, PeerMessageType::Bitfield);

            // Send an interested message
            framed.send(PeerMessage {
                id: PeerMessageType::Interested,
                length: 1,
                payload: vec![],
            }).await.expect("Error sending interested message");

            let unchoke = framed.next().await.expect("Expect a unchoke").context("Error reading unchoke")?;
            assert_eq!(unchoke.id, PeerMessageType::Unchoke);

            let length = torrent.info.length;
            println!("Length: {}", length);
            let piece_length = torrent.info.piece_length;
            println!("Piece length: {}", piece_length);

            let piece_size = torrent.info.piece_length;

            let nblocks = (piece_size + (BLOCK_SIZE - 1)) / BLOCK_SIZE;
            println!("Number of blocks: {}", nblocks);
            println!("Block size: {}", BLOCK_SIZE);

            for block in 0..nblocks {
                let offset = block * BLOCK_SIZE;
                let length = std::cmp::min(BLOCK_SIZE, piece_size - offset);
                let mut payload = BytesMut::with_capacity(12);

                // Print data
                println!("Request block: {}, Offset: {}, Length: {}", block, offset, length);

                // Add data to request payload
                payload.put(&piece_index.to_be_bytes()[..]);
                payload.put(&offset.to_be_bytes()[..]);
                payload.put(&length.to_be_bytes()[..]);

                let request = PeerMessage {
                    id: PeerMessageType::Request,
                    length: 13,
                    payload: payload.to_vec(),
                };
                framed.send(request).await.with_context(|| format!("Error sending request for block {}", block))?;

                let piece = framed
                    .next()
                    .await
                    .expect("peer always sends a piece")
                    .context("Piece message was invalid")?;
                assert_eq!(piece.id, PeerMessageType::Piece);
                assert!(!piece.payload.is_empty());

                println!("Piece payload length: {}", piece.payload.len());

                // Split the payload bytes to get the index, offset, and data
                let (index_bytes, rest) = piece.payload.split_at(4);
                let (offset_bytes, data) = rest.split_at(4);

                assert_eq!(u32::from_be_bytes([index_bytes[0], index_bytes[1], index_bytes[2], index_bytes[3]]), piece_index);
                assert_eq!(u32::from_be_bytes([offset_bytes[0], offset_bytes[1], offset_bytes[2], offset_bytes[3]]), offset as u32);
                assert_eq!(data.len(), length as usize);





            }






            Ok(())

            // Wait to receive an unchocke message back
            // todo!("Wait to receive unchoke message");
            //
            // todo!("Break the piece into blocks of 16kiB");
            //
            // todo!("Wait for a piece message for each block requested");
            //
            // todo!("Check the integrity of each piece block");
            //
            // Ok(())
        }
    }
}

async fn make_handshake(stream: &mut TcpStream, info_hash: &[u8; 20]) -> Result<()> {
    let handshake = Handshake::new(*info_hash, *b"00112233445566778899");

    let handshake_bytes_size = std::mem::size_of::<Handshake>();
    println!("Handshake size: {}", handshake_bytes_size);
    let serialized_bytes = bincode::serialize(&handshake).expect("Serialization failed for handshake");
    //println!("Serialized: {:?}", serialized_bytes);
    stream.write_all(&serialized_bytes).await.expect("Error writing to stream");


    // Read the current data from the stream
    let mut reader = BufReader::new(stream);
    let received: Vec<u8> = reader.fill_buf().await.expect("Error reading from stream").to_vec();
    println!("Received length: {}", received.len());
    let handshake_response: Handshake = bincode::deserialize(&received).expect("Error deserializing handshake");
    println!("Peer ID: {}", handshake_response.peer_id.iter().map(|b| format!("{:02x}", b)).collect::<String>());
    assert_eq!(handshake_response.p_str, *b"BitTorrent protocol");
    assert_eq!(handshake_response.length, 19);
    // Consume the buffer
    reader.consume(received.len());
    Ok(())
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
                    if print { println!("Tracker URL: {}", output); }
                }

                if let Some(info) = map.get("info".as_bytes()) {
                    if let BencodeValue::BDictionary(map) = info {
                        if let Some(length) = map.get("length".as_bytes()) {
                            if print { println!("Length: {}", length); }
                            torrent.info.length = length.to_string().parse::<i64>().unwrap();
                        }

                        if let Some(length) = map.get("piece length".as_bytes()) {
                            if print { println!("Piece Length: {}", length); }
                            torrent.info.piece_length = length.to_string().parse::<i64>().unwrap();
                        }

                        let mut encoder = encode::Encoder::new();
                        encoder.encode(info)?;
                        let hash = encoder.encode_sha1(info_hash);
                        if print { println!("Info Hash: {}", hash); }


                        if let Some(pieces_string) = map.get("pieces".as_bytes()) {
                            // Get the bytes string and represent as hexadecimal
                            // Represent hexadecimal hash of each piece
                            if let BencodeValue::BString(pieces_string) = pieces_string {
                                if print { println!("Piece Hashes:"); }
                                let mut remaining_hash_data = &pieces_string[..];
                                while !remaining_hash_data.is_empty() {
                                    let (hash, rest) = remaining_hash_data.split_at(20);
                                    remaining_hash_data = rest;
                                    let hash_in_hex = hex::encode(hash);
                                    if print { println!("{}", hash_in_hex); }
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

async fn make_peer_request(info_hash: &[u8; 20], torrent: &Torrent, peer_id: String) -> Result<Vec<SocketAddr>> {
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

    for peer in list_peers.clone() {
        println!("{}", peer);
    }

    Ok(list_peers)
}

pub fn url_encode(info_hash: &[u8; 20]) -> String {
    let mut url_encoded = String::new();
    for byte in info_hash {
        url_encoded.push_str(&format!("%{:02x}", byte));
    }
    url_encoded
}