use std::collections::HashMap;
use crate::value::BencodeValue;
use crate::error::{Result, Error};
use std::result::Result as stdResult;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum ParseDecode {
    Integer(i64),
    Bytes(Vec<u8>),
    List,
    Dictionary,
    End,
}

pub struct Parser<'a> {
    input: &'a [u8],
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Parser {
            input,
            index: 0,
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

    pub fn parse_string_len(&mut self) -> Result<i64> {
        let mut num_bytes: Vec<u8> = Vec::new();
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
            b'0'..=b'9' => {
                // Example: "5:hello" -> "hello"
                // Find the index of the colon
                let string_len = self.parse_string_len()?;
                let string = self.parse_bytes(string_len)?;
                Ok(BencodeValue::BString(string))
            }
            b'l' => {
                // Example: "l5:helloi52ee" -> ["hello", 52]
                let mut list = Vec::new();
                let decoded_value = self.parse()?;
                list.push(decoded_value);

                Ok(BencodeValue::BList(list))
            }
            b'd' => {
                // Example: "d5:helloi52ee" -> {"hello": 52}
                let mut map = HashMap::new();
                let key = self.parse()?;
                match key {
                    BencodeValue::BString(key_string) => {
                        let value = self.parse()?;
                        map.insert(String::from_utf8(key_string).map_err(|_| Error::Message("Error utf8 for map key input".to_string()))?, value);
                    }
                    _ => {
                        return Err(Error::Message(format!("Invalid key type: {:?}", key)));
                    }
                }

                Ok(BencodeValue::BDictionary(map))
            }
            _ => {
                Err(Error::Message(format!("Invalid character `{}`", buf[0])))
            }
        }
    }
}

#[allow(dead_code)]
pub fn decode_bencoded_value(encoded_value: &[u8], _index: usize) -> stdResult<(serde_json::Value, usize), Box<dyn std::error::Error>> {

    // If encoded_value starts with a i, and ends with an e and inside it's a number
    if encoded_value[0] == b'i' {
        // Example: "i52e" -> "52"
        // collect digits until we find an e
        let number_string = encoded_value.to_vec()[1..].iter().take_while(|c| **c != b'e').map(|c| *c as char).collect::<String>();
        let number = number_string.parse::<i64>().unwrap();
        Ok((serde_json::Value::Number(number.into()), number_string.len() + 2))
    }
    // If encoded value is a string, it starts with a number, followed by a colon, followed by the string
    else if encoded_value[0].is_ascii_digit() {
        // Example: "5:hello" -> "hello"

        // Find the index of the colon
        let number_string = encoded_value.iter().take_while(|c| **c != b':').map(|c| *c as char).collect::<String>();
        let number = number_string.parse::<i64>().unwrap();
        let string = encoded_value.iter().skip(number_string.len() + 1).take(number as usize).map(|c| *c as char).collect::<String>();


        Ok((serde_json::Value::String(string.to_string()), string.len() + number_string.len() + 1))
    }
    // If encoded value is a list, it starts with an l, ends with an e, and contains encoded values
    else if encoded_value.starts_with(b"l") {
        // Example: "l5:helloi52ee" -> ["hello", 52]
        // Example: "lli4eei5ee" -> [[4], 5]
        let mut list = Vec::new();
        // We need to remove the l and e from the encoded value
        // let inside_encoded_value = &encoded_value[1..encoded_value.len() - 1];
        let mut current_index = 1;
        while current_index < encoded_value.len() - 1 {
            if encoded_value[current_index] == b'e' {
                break;
            }
            let current_value = &encoded_value[current_index..encoded_value.len() - 1];
            match decode_bencoded_value(current_value, current_index) {
                Ok((decoded_value, value_index)) => {
                    current_index += value_index;
                    list.push(decoded_value);
                }
                Err(_) => {
                    break;
                }
            }
            // let Ok((decoded_value, item_index)) = decode_bencoded_value(current_value, current_index);
            // current_index += item_index;
            // list.push(decoded_value);
        }
        Ok((serde_json::Value::Array(list), current_index + 1))
    } else if encoded_value.starts_with(b"d") {
        // Example: "d5:helloi52ee" -> {"hello": 52}
        let mut map = serde_json::Map::new();
        // We need to remove the d and e from the encoded value
        // let inside_encoded_value = &encoded_value[1..encoded_value.len() - 1];
        let mut current_index = 1;
        while current_index < encoded_value.len() - 1 {
            let current_value = &encoded_value[current_index..encoded_value.len() - 1];
            match decode_bencoded_value(current_value, current_index) {
                Ok((decoded_value, value_index)) => {
                    // Use from_value to get key string without quotes
                    let key = serde_json::from_value(decoded_value.clone()).unwrap();
                    // println!("Key: {}", key);
                    current_index += value_index;
                    if encoded_value[current_index] == b'e' {
                        break;
                    }
                    // let current_value = &encoded_value[current_index..encoded_value.len() - 1];
                    current_index += value_index;
                    map.insert(key, decoded_value);
                }
                Err(_) => {
                    break;
                }
            }


            // let Ok((decoded_value, value_index)) = decode_bencoded_value(current_value, current_index);
            // current_index += value_index;
            // map.insert(key, decoded_value);
        }
        Ok((serde_json::Value::Object(map), current_index + 1))
    } else {
        panic!("Unhandled encoded value: {}", encoded_value.to_vec().iter().map(|c| *c as char).collect::<String>());
    }
}




