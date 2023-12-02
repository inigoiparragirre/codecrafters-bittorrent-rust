use serde_derive::{Deserialize, Serialize};
use crate::peers::addr::Address;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerRequest {
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
    pub peers: Address,
}
