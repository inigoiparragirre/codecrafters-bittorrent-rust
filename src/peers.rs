use serde_derive::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize)]
pub enum PeerMessageType {
    Choke = 0,
    Unchoke = 1,
    Interested = 2,
    NotInterested = 3,
    Have = 4,
    Bitfield = 5,
    Request = 6,
    Piece = 7,
    Cancel = 8,
}

#[derive(Debug)]
#[repr(C)]
pub struct PeerMessage {
    pub length: u32,
    pub id: u8,
    pub payload: Vec<u8>,
}

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

pub mod addr {
    use std::fmt;
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    use serde::{Deserialize, Deserializer};
    use serde::de::Visitor;

    #[derive(Debug, Clone)]
    pub struct Address(pub Vec<SocketAddr>);

    impl<'de> Deserialize<'de> for Address {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
        {
            deserializer.deserialize_bytes(PeerVisitor)
        }
    }

    struct PeerVisitor;

    impl<'de> Visitor<'de> for PeerVisitor {
        type Value = Address;


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
            Ok(Address(
                v.chunks_exact(6)
                    .map(|slice_6| {
                        SocketAddr::V4(SocketAddrV4::new(
                            Ipv4Addr::new(slice_6[0], slice_6[1], slice_6[2], slice_6[3]),
                            u16::from_be_bytes([slice_6[4], slice_6[5]]),
                        ))
                    })
                    .collect(),
            ))
        }
    }
}




