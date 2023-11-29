use std::fmt::{self, Display};
use thiserror::Error;
use serde::{de, ser};


#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum BencodeError {
    Message(String),
    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`. These
    // are specific to the format, in this case JSON.
    Eof,
    Syntax,
    // InvalidType,
}

impl ser::Error for BencodeError {
    fn custom<T: Display>(msg: T) -> Self {
        BencodeError::Message(msg.to_string())
    }
}

impl de::Error for BencodeError {
    fn custom<T: Display>(msg: T) -> Self {
        BencodeError::Message(msg.to_string())
    }
}

impl Display for BencodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BencodeError::Message(msg) => formatter.write_str(msg),
            BencodeError::Eof => formatter.write_str("unexpected end of input"),
            _ => formatter.write_str("unknown error"),
        }
    }
}


