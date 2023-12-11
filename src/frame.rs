use bytes::{Buf, BytesMut};
use tokio::io;
use tokio_util::codec::{Decoder, Encoder};
use crate::peers::PeerMessage;

const MAX: usize = 8 * 1024 * 1024;

pub struct MessageDecoder;


impl Encoder<String> for MessageDecoder {
    type Error = std::io::Error;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // Don't send a string if it is longer than the other end will
        // accept.
        if item.len() > MAX {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Frame of length {} is too large.", item.len())
            ));
        }

        // Convert the length into a byte array.
        // The cast to u32 cannot overflow due to the length check above.
        let len_slice = u32::to_be_bytes(item.len() as u32);

        // Reserve space in the buffer.
        dst.reserve(4 + item.len());

        // Write the length and string to the buffer.
        dst.extend_from_slice(&len_slice);
        dst.extend_from_slice(item.as_bytes());
        Ok(())
    }
}

impl Decoder for MessageDecoder {
    type Item = PeerMessage;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<PeerMessage>, io::Error> {
        if src.len() < 4 {
            return Ok(None);
        }

        // Read length marker.
        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&src[..4]);
        let length = u32::from_be_bytes(length_bytes) as usize;

        if src.len() < 4 + length {
            // The full string has not yet arrived.
            //
            // We reserve more space in the buffer. This is not strictly
            // necessary, but is a good idea performance-wise.
            src.reserve(4 + length - src.len());

            // We inform the Framed that we need more bytes to form the next
            // frame.
            return Ok(None);
        }

        let id = src[4];
        match id {
            0 => println!("choke"),
            1 => println!("unchoke"),
            2 => println!("interested"),
            3 => println!("not interested"),
            4 => println!("have"),
            5 => println!("bitfield"),
            6 => println!("request"),
            7 => println!("piece"),
            8 => println!("cancel"),
            9 => println!("port"),
            _ => println!("unknown"),
        }
        let payload = src[5..length + 4].to_vec();

        src.advance(length + 4);

        Ok(Some(PeerMessage {
            length: length as u32,
            id,
            payload,
        }))
    }
}