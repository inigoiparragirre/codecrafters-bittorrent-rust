use linked_hash_map::LinkedHashMap;
use sha1::{Sha1, Digest};
use crate::value::BencodeValue;
use crate::error::{Result};

pub struct Encoder {
    buf: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            buf: Vec::new(),
        }
    }

    /// Encode info bytes to a sha1 hash (both hex and byte string representation)
    pub fn encode_sha1(&self, info_hash: &mut String) -> String {
        let mut hasher = Sha1::new();
        hasher.update(&self.buf);
        let result = hasher.finalize();

        // Convert the result to a hex string and return as output
        let mut hex_string = String::new();
        for byte in result {
            info_hash.push_str(&format!("{}", byte));
            hex_string.push_str(&format!("{:02x}", byte));
        }
        hex_string
    }

    pub fn encode(&mut self, input: &BencodeValue) -> Result<()> {
        match input {
            BencodeValue::BString(msg) => {
                self.encode_string(msg)?;
            }
            BencodeValue::BInteger(code) => {
                self.encode_integer(*code)?;
            }
            BencodeValue::BList(list) => {
                self.encode_list(list)?;
            }
            BencodeValue::BDictionary(map) => {
                self.encode_dictionary(map)?;
            }
            BencodeValue::BEnd => self.buf.push(b'e'),
        }
        Ok(())
    }

    pub fn encode_dictionary(&mut self, map: &LinkedHashMap<Vec<u8>, BencodeValue>) -> Result<()> {
        self.buf.push(b'd');
        // Add a comma after the item
        for (key, value) in map {
            self.encode_string(key)?;
            self.encode(value)?;
        }
        self.buf.push(b'e');
        Ok(())
    }

    pub fn encode_list(&mut self, list: &Vec<BencodeValue>) -> Result<()> {
        self.buf.push(b'l');
        // Add a comma after the item
        for item in list {
            self.encode(item)?;
        }
        self.buf.push(b'e');
        Ok(())
    }

    pub fn encode_string(&mut self, bytes: &Vec<u8>) -> Result<()> {
        let length = bytes.len();
        // Add each byte to the buffer
        length.to_string().as_bytes().iter().for_each(|b| self.buf.push(*b));
        self.buf.push(b':');
        bytes.iter().for_each(|b| self.buf.push(*b));
        Ok(())
    }

    pub fn encode_integer(&mut self, integer: i64) -> Result<()> {
        self.buf.push(b'i');
        integer.to_string().as_bytes().iter().for_each(|b| self.buf.push(*b));
        self.buf.push(b'e');
        Ok(())
    }
}