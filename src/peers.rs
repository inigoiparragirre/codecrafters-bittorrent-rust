use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Peer {
    ip: u32,
    port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerRequest {
    pub info_hash: String,
    pub peer_id: String,
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
    pub compact: u8,
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