use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Peer {
    ip: u32,
    port: u16,
}

#[derive(Debug, Deserialize)]
pub struct TrackerRequest {
    info_hash: String,
    peer_id: String,
    port: u16,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    compact: u8,
    // no_peer_id: u8,
    // event: String,
    // ip: String,
    // numwant: u32,
    // key: String,
    // trackerid: String,
}

#[derive(Debug, Serialize)]
pub struct TrackerResponse {
    interval: u64,
    peers: Vec<Peer>,
}