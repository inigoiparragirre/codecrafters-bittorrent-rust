use std::fmt;
use serde::de::Visitor;
use serde::Deserializer;
use serde_bytes::Deserialize;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Peer {
    ip: u32,
    port: u16,
}

struct PeerVisitor;

impl<'de> Visitor<'de> for PeerVisitor {
    type Value = Peer;


    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a peer")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        let ip = u32::from_be_bytes([v[0], v[1], v[2], v[3]]);
        let port = u16::from_be_bytes([v[4], v[5]]);
        Ok(Peer { ip, port })
    }
}

impl<'de> Deserialize<'de> for Peer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PeerVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerRequest {
    pub peer_id: String,
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
    pub compact: u8,
}

pub fn url_encode(info_hash: &[u8; 20]) -> String {
    let mut url_encoded = String::new();
    for byte in info_hash {
        url_encoded.push_str(&format!("%{:02x}", byte));
    }
    url_encoded

}

// Implement default for TrackerRequest
impl Default for TrackerRequest {
    fn default() -> Self {
        TrackerRequest {
            peer_id: "".to_string(),
            port: 0,
            uploaded: 0,
            downloaded: 0,
            left: 0,
            compact: 1,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TrackerResponseSuccess {
    pub interval: i64,
    #[serde(rename = "min interval")]
    pub min_interval: i64,
    pub incomplete: i64,
    pub complete: i64,
    //#[serde(deserialize_with = "deserialize_peer_list")]
    pub peers: Vec<Peer>,
}

#[derive(Debug, Deserialize)]
pub struct TrackerResponseError {
    pub failure_reason: String,
}

#[derive(Debug, Deserialize)]
pub enum TrackerResponse {
    Success(TrackerResponseSuccess),
    Error(TrackerResponseError),
}

// fn deserialize_peer_list<'de, D>(deserializer: D) -> Result<Vec<Peer>, D::Error>
//     where
//         D: serde::Deserializer<'de>,
// {
//
//     println!("Deserializing peer list: ",);
//
//     let bytes = <Vec<u8>>::deserialize(deserializer)?;
//     let bytes_length = bytes.len();
//     println!("Bytes length: {}", bytes_length);
//
//     println!("Bytes peer list: {:?}", bytes);
//     let mut peers = Vec::new();
//     for i in 0..bytes.len() / 6 {
//         let ip = u32::from_be_bytes([bytes[i * 6], bytes[i * 6 + 1], bytes[i * 6 + 2], bytes[i * 6 + 3]]);
//         let port = u16::from_be_bytes([bytes[i * 6 + 4], bytes[i * 6 + 5]]);
//         peers.push(Peer { ip, port });
//     }
//     println!("Peers: {:?}", peers);
//     Ok(peers)
// }

