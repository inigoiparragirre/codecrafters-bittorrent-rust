use std::collections::HashMap;
use crate::value::BencodeValue;
use crate::error::{Result, Error};
use std::io::Read;

// #[derive(Debug, PartialEq)]
// pub enum ParseDecode {
//     Integer(i64),
//     Bytes(Vec<u8>),
//     List,
//     Dictionary,
//     End,
// }

pub struct Parser<'a> {
    input: &'a [u8],
    //index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Parser {
            input,
            //index: 0,
        }
    }

    pub fn parse_number(&mut self) -> Result<i64> {
        let mut num_bytes: Vec<u8> = Vec::new();
        loop {
            let mut buf: [u8; 1] = [0; 1];
            self.input.read(&mut buf).map_err(|_| Error::Message("Error reading number".to_string()))?;
            if buf[0] == b'e' {
                break;
            }
            num_bytes.push(buf[0]);
        }
        let num_string = String::from_utf8(num_bytes).map_err(|_| Error::Message("Error converting number to string from_utf8".to_string()))?;
        let number = num_string.parse::<i64>().map_err(|_| Error::Message("Error parsing number".to_string()))?;
        Ok(number)
    }

    pub fn parse_bytes(&mut self, len: i64) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::new();
        for _ in 0..len {
            let mut buf: [u8; 1] = [0; 1];
            self.input.read(&mut buf).map_err(|_| Error::Message("Error reading number".to_string()))?;
            bytes.push(buf[0]);
        }
        Ok(bytes)
    }

    pub fn parse_string_len(&mut self, first: u8) -> Result<i64> {
        let mut num_bytes: Vec<u8> = Vec::new();
        num_bytes.push(first);
        loop {
            let mut buf: [u8; 1] = [0; 1];
            self.input.read(&mut buf).map_err(|_| Error::Message("Error reading number".to_string()))?;
            if buf[0] == b':' {
                break;
            }
            num_bytes.push(buf[0]);
        }
        let num_string = String::from_utf8(num_bytes).map_err(|_| Error::Message("Error converting number to string from_utf8".to_string()))?;
        let number = num_string.parse::<i64>().map_err(|_| Error::Message("Error parsing number".to_string()))?;
        Ok(number)

    }

    pub fn parse_list(&mut self) -> Result<BencodeValue> {
        let mut list = Vec::new();
        loop {
            let decoded_value = self.parse()?;
            match decoded_value {
                BencodeValue::BEnd => {
                    break;
                }
                _ => {
                    list.push(decoded_value);
                }
            }
        }
        Ok(BencodeValue::BList(list))
    }

    pub fn parse_dictionary(&mut self) -> Result<BencodeValue> {
        let mut map = HashMap::new();
        loop {
            let key = self.parse().map_err(|_| Error::Message("Error mapping key input".to_string()))?;
            match key {
                BencodeValue::BString(key_string) => {
                    let value = self.parse()?;
                    map.insert(key_string, value);
                }
                BencodeValue::BEnd => {
                    return Ok(BencodeValue::BDictionary(map));
                }
                _ => {
                    return Err(Error::Message(format!("Invalid key type: {:?}", key)));
                }
            }
        }
    }

    pub fn parse(&mut self) -> Result<BencodeValue> {

        // Read byte character
        let mut buf: [u8; 1] = [0; 1];
        self.input.read(&mut buf).map_err(|_| Error::Message("Error reading input".to_string()))?;
        return match buf[0] {
            b'i' => {
                // Example: "i52e" -> "52"
                let number = self.parse_number()?;
                Ok(BencodeValue::BInteger(number))
            }
            first @ b'0'..=b'9' => {
                // Example: "5:hello" -> "hello"
                // Find the index of the colon
                let string_len = self.parse_string_len(first)?;
                let string = self.parse_bytes(string_len)?;
                Ok(BencodeValue::BString(string))
            }
            b'l' => {
                // Example: "l5:helloi52ee" -> ["hello", 52]
                let decoded_list = self.parse_list()?;

                Ok(decoded_list)
            }
            b'd' => {
                // Example: "d5:helloi52ee" -> {"hello": 52}
                let decoded_dictionary = self.parse_dictionary()?;
                Ok(decoded_dictionary)
            }
            b'e' => {
                Ok(BencodeValue::BEnd)
            }
            _ => {
                Err(Error::Message(format!("Invalid character `{}`", buf[0])))
            }
        }
    }
}

