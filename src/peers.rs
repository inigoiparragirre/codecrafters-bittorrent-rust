use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    ip: u32,
    port: u16,
}

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerResponseSuccess {
    pub interval: i64,
    #[serde(rename = "min interval")]
    pub min_interval: i64,
    pub incomplete: i64,
    pub complete: i64,
    pub peers: Vec<Peer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerResponseError {
    pub failure_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TrackerResponse {
    Success(TrackerResponseSuccess),
    Error(TrackerResponseError),
}