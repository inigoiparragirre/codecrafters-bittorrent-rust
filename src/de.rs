use serde::{de, forward_to_deserialize_any};
use serde::de::Visitor;
use crate::decode::{ParseDecode};
use crate::error::{Error, Result};

struct BAccess<'a, 'de> {
    de: &'a mut BDeserializer<'de>,
    len: Option<usize>,
}

impl<'de: 'a, 'a> BAccess<'a, 'de> {
    fn new(de: &'a mut BDeserializer<'de>, len: Option<usize>) -> Self {
        BAccess {de, len}
    }
}

impl<'a, 'de> de::MapAccess<'de> for BAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where
            K: de::DeserializeSeed<'de>,
    {
        match self.de.decode()? {
            ParseDecode::End => Ok(None),
            _r => {
                //self.de.next = Some(r);
                Ok(Some(seed.deserialize(&mut *self.de)?))
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where
            V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

impl<'a, 'de> de::SeqAccess<'de> for BAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>> {
        let res = match self.de.decode()? {
            ParseDecode::End => Ok(None),
            _r => {
                // self.de.next = Some(r);
                Ok(Some(seed.deserialize(&mut *self.de)?))
            }
        };
        if let Some(l) = self.len {
            let l = l - 1;
            self.len = Some(l);
            if l == 0 && ParseDecode::End != self.de.decode()? {
                return Err(Error::Message("expected `e`".to_string()));
            }
        }
        res
    }
}

pub struct BDeserializer<'de> {
    input: &'de [u8],
}

impl<'de> BDeserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        BDeserializer { input }
    }

    fn decode(&self) -> Result<ParseDecode> {
        let c = self.input[0];
        match c {
            b'i' => Ok(ParseDecode::Integer(self.decode_integer()?)),
            b'l' => Ok(ParseDecode::List),
            b'd' => Ok(ParseDecode::Dictionary),
            b'e' => Ok(ParseDecode::End),
            b'0'..=b'9' => Ok(ParseDecode::Bytes(self.decode_bytes()?)),
            _ => Err(Error::Message(format!(
                    "Invalid character `{}`",
                    c as char
                )))
        }
        // Ok(c)
    }

    fn decode_bytes(&self) -> Result<Vec<u8>> {
        // Review it
        let mut i = 1;
        while self.input[i] != b':' {
            i += 1;
        }
        let s = std::str::from_utf8(&self.input[1..i]).unwrap();
        let length = s.parse::<usize>().unwrap();
        let bytes = &self.input[i + 1..i + 1 + length];
        Ok(bytes.to_vec())

    }

    fn decode_integer(&self) -> Result<i64> {
        let mut i = 1;
        while self.input[i] != b'e' {
            i += 1;
        }
        let s = std::str::from_utf8(&self.input[1..i]).unwrap();
        Ok(s.parse::<i64>().unwrap())

    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut BDeserializer<'de> {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.decode()? {
            ParseDecode::Dictionary => visitor.visit_map(BAccess::new(self, None)),
            ParseDecode::List => visitor.visit_seq(BAccess::new(self, None)),
            ParseDecode::Integer(num) => visitor.visit_i64(num),
            ParseDecode::Bytes(b) => visitor.visit_bytes(b.as_ref()),
            ParseDecode::End => visitor.visit_unit(),
        }
    }


    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }

    // fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    //     where
    //         V: Visitor<'de>,
    // {
    //     visitor.visit_u64();
    // }
}

// This basic deserializer supports only `from_str`.
// pub fn from_str<'a, T>(s: &'a str) -> Result<T>
//     where
//         T: Deserialize<'a>,
// {
//     let mut deserializer = Deserializer::from_str(s);
//     let t = T::deserialize(&mut deserializer)?;
//     if deserializer.input.is_empty() {
//         Ok(t)
//     } else {
//         Err(Error::TrailingCharacters)
//     }
// }

pub fn from_bytes<'de, T>(b: &'de [u8]) -> Result<T>
    where
        T: de::Deserialize<'de>,
{
    de::Deserialize::deserialize(&mut BDeserializer::from_bytes(b))
}

