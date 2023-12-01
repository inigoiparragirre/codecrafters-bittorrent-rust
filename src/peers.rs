use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Handshake {
    pub length: u8,
    pub p_str: [u8; 19],
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}


impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Handshake {
        Handshake {
            length: 19,
            p_str: *b"BitTorrent protocol",
            reserved: [0; 8],
            info_hash,
            peer_id,
        }
    }
}

pub mod peer {
    use std::fmt;
    use std::net::{Ipv4Addr, SocketAddrV4};
    use serde::{Deserialize, Deserializer};
    use serde::de::Visitor;

    #[derive(Debug, Clone)]
    pub struct Peers(pub Vec<SocketAddrV4>);

    impl<'de> Deserialize<'de> for Peers {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
        {
            deserializer.deserialize_bytes(PeerVisitor)
        }
    }

    struct PeerVisitor;

    impl<'de> Visitor<'de> for PeerVisitor {
        type Value = Peers;


        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a peer")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
        {
            if v.len() % 6 != 0 {
                return Err(E::custom(format!("bytes length error for peers {}", v.len())));
            }
            Ok(Peers(
                v.chunks_exact(6)
                    .map(|slice_6| {
                        SocketAddrV4::new(
                            Ipv4Addr::new(slice_6[0], slice_6[1], slice_6[2], slice_6[3]),
                            u16::from_be_bytes([slice_6[4], slice_6[5]]),
                        )
                    })
                    .collect(),
            ))
        }
    }


}

use peer::Peers;

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
    pub peers: Peers,
}



