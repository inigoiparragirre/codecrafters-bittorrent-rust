use serde_bytes::ByteBuf;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Torrent {
    pub info: Info,
    pub announce: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    pub piece_length: i64,
    pub length: i64,
}