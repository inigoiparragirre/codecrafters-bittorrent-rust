use std::collections::HashMap;
use std::fmt;



/// BencodeValue is an enum that represents all possible values that can be
/// encoded in bencode.
#[derive(Debug, PartialEq)]
pub enum BencodeValue {
    /// An array of bytes
    BString(Vec<u8>),

    /// Integer value
    BInteger(i64),

    /// List of bencode values
    BList(Vec<BencodeValue>),

    /// Dictionary of bencode values
    BDictionary(HashMap<Vec<u8>, BencodeValue>),
}

impl From<String> for BencodeValue {
    fn from(s: String) -> BencodeValue {
        BencodeValue::BString(s.into_bytes())
    }
}

impl From<HashMap<Vec<u8>, BencodeValue>> for BencodeValue {
    fn from(v: HashMap<Vec<u8>, BencodeValue>) -> BencodeValue {
        BencodeValue::BDictionary(v)
    }
}

impl fmt::Display for BencodeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // We output the string as a quoted string
            BencodeValue::BString(msg) => write!(f, r#""{}""#, String::from_utf8(msg.clone()).unwrap()),
            BencodeValue::BInteger(code) => write!(f, "{}", code),
            // Handle other variants if needed
            _ => write!(f, "Not implemented"),
        }
    }
}

