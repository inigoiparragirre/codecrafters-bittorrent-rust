use serde_bytes::ByteBuf;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Torrent
{
    pub info: Info,
    pub announce: String,
}

impl Torrent {

    pub fn new() -> Torrent {
        Torrent {
            info: Info {
                name: "".to_string(),
                pieces: ByteBuf::new(),
                piece_length: 0,
                length: 0,
            },
            announce: "".to_string(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    pub piece_length: i64,
    pub length: i64,
}