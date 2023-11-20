
struct BAccess;

impl BAccess {
    fn new() -> Self {
        BAccess {}
    }
}




pub struct BDeserializer<'de> {

    input: &'de Vec<u8>,
}

impl<'de> BDeserializer<'de> {

    pub fn from_bytes(input: &'de Vec<u8>) -> Self {
        Deserializer { input }
    }
}

impl<'de> BDeserializer<'de> {
    fn new(input: &'de Vec<u8>) -> Self {
        Deserializer { input }
    }

    fn decode(&self) -> Result<ParseResult, &'static str> {
        let c = self.input[0];
        match c {
            'i' => Ok(ParseResult::Integer(self.decode_integer()?)),
            'l' => Ok(ParseResult::List),
            'd' => Ok(ParseResult::Dictionary),
            'e' => Ok(ParseResult::End),
            '0'..='9' => Ok(ParseResult::Bytes(self.decode_bytes()?)),
            _ => Err(Error::InvalidValue(format!(
                    "Invalid character `{}`",
                    c as char
                )))
        }
        Ok(c)
    }

    fn decode_bytes(&self) {

    }

    fn decode_integer(&self) {

    }
}


impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.parse()? {
            'd' => self.deserialize_map(visitor),
            'l' => self.deserialize_seq(visitor),
            '0'..='9' => self.deserialize_u64(visitor),
            'e' => self.deserialize_i64(visitor),

            _ => Err(Error::Syntax),
        }
    }
}
