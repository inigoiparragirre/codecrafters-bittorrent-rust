use std::collections::HashMap;
use std::fmt;
use serde::{de, Deserializer};
use serde_bytes::ByteBuf;

/// BencodeValue is an enum that represents all possible values that can be
/// encoded in bencode.


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

struct BencodeValueVisitor;

impl<'de> de::Visitor<'de> for BencodeValueVisitor {

    type Value = BencodeValue;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any valid BEncode value")
    }

    fn visit_seq<V>(self, mut access: V) -> Result<BencodeValue, V::Error>
        where
            V: de::SeqAccess<'de>,
    {
        let mut seq = Vec::new();
        while let Some(e) = access.next_element()? {
            seq.push(e);
        }
        Ok(BencodeValue::BList(seq))
    }

    fn visit_map<V>(self, mut access: V) -> Result<BencodeValue, V::Error>
        where
            V: de::MapAccess<'de>,
    {
        let mut map = HashMap::new();
        while let Some((k, v)) = access.next_entry::<ByteBuf, _>()? {
            map.insert(k.into_vec(), v);
        }
        Ok(BencodeValue::BDictionary(map))
    }


}


impl<'de> de::Deserialize<'de> for BencodeValue {


    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BencodeValueVisitor)
    }
}


