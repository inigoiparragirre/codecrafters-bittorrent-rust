use linked_hash_map::LinkedHashMap;
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
    BDictionary(LinkedHashMap<Vec<u8>, BencodeValue>),

    /// End value: only for detecting the close of encoding
    BEnd,
}

impl From<String> for BencodeValue {
    fn from(s: String) -> BencodeValue {
        BencodeValue::BString(s.into_bytes())
    }
}

impl From<LinkedHashMap<Vec<u8>, BencodeValue>> for BencodeValue {
    fn from(v: LinkedHashMap<Vec<u8>, BencodeValue>) -> BencodeValue {
        BencodeValue::BDictionary(v)
    }
}

impl fmt::Display for BencodeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // We output the string as a quoted string
            BencodeValue::BString(msg) => write!(f, r#""{}""#, String::from_utf8(msg.clone()).unwrap()),
            BencodeValue::BInteger(code) => write!(f, "{}", code),
            BencodeValue::BList(list) => {
                let mut output = String::new();
                // Add a comma after the item
                for item in list {
                    output.push_str(&format!("{},", item));
                }
                let output_str = output.trim_end_matches(","); // Remove the last comma
                write!(f, r#"[{}]"#, output_str)
            }
            BencodeValue::BDictionary(map) => {
                let mut output = String::new();
                // Add a comma after the item
                for (key, value) in map {
                    output.push_str(&format!(r#""{}":{},"#, String::from_utf8(key.clone()).unwrap(), value));
                }
                let output_str = output.trim_end_matches(","); // Remove the last comma
                write!(f, r#"{{{}}}"#, output_str)
            }
            // Handle other variants if needed
            _ => write!(f, "Not implemented"),
        }
    }
}

