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
}

// Implement default for TrackerRequest
impl Default for TrackerRequest {
    fn default() -> Self {
        TrackerRequest {
            info_hash: "".to_string(),
            peer_id: "".to_string(),
            port: 0,
            uploaded: 0,
            downloaded: 0,
            left: 0,
            compact: 1,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TrackerResponse {
    interval: u64,
    peers: Vec<Peer>,
}