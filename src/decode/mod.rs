
    #[allow(dead_code)]
    pub fn decode_bencoded_value(encoded_value: &str, _index: usize) -> (serde_json::Value, usize) {

        // If encoded_value starts with a i, and ends with an e and inside it's a number
        if encoded_value.starts_with('i') {
            // Example: "i52e" -> "52"
            let end_index = encoded_value.find('e').unwrap();
            let number_string = &encoded_value[1..end_index];
            let number = number_string.parse::<i64>().unwrap();
            (serde_json::Value::Number(number.into()), number_string.len() + 2)
        }
        // If encoded value is a string, it starts with a number, followed by a colon, followed by the string
        else if encoded_value.chars().next().is_some_and(|c| c.is_digit(10)) {
            // Example: "5:hello" -> "hello"
            let colon_index = encoded_value.find(':').unwrap();
            let number_string = &encoded_value[..colon_index];
            let number = number_string.parse::<i64>().unwrap();
            let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
            (serde_json::Value::String(string.to_string()), string.len() + number_string.len() + 1)
        }
        // If encoded value is a list, it starts with an l, ends with an e, and contains encoded values
        else if encoded_value.starts_with('l') {
            // Example: "l5:helloi52ee" -> ["hello", 52]
            // Example: "lli4eei5ee" -> [[4], 5]
            let mut list = Vec::new();
            // We need to remove the l and e from the encoded value
            // let inside_encoded_value = &encoded_value[1..encoded_value.len() - 1];
            let mut current_index = 1;
            while current_index < encoded_value.len() - 1 {
                if encoded_value.chars().nth(current_index).unwrap() == 'e' {
                    break;
                }
                let current_value = &encoded_value[current_index..encoded_value.len() - 1];
                let (decoded_value, item_index) = decode_bencoded_value(current_value, current_index);
                current_index += item_index;
                list.push(decoded_value);
            }
            (serde_json::Value::Array(list), current_index + 1)
        } else if encoded_value.starts_with('d') && encoded_value.ends_with('e') {
            // Example: "d5:helloi52ee" -> {"hello": 52}
            let mut map = serde_json::Map::new();
            // We need to remove the d and e from the encoded value
            // let inside_encoded_value = &encoded_value[1..encoded_value.len() - 1];
            let mut current_index = 1;
            while current_index < encoded_value.len() - 1 {
                let current_value = &encoded_value[current_index..encoded_value.len() - 1];
                let (decoded_key, key_index) = decode_bencoded_value(current_value, current_index);
                // Use from_value to get key string without quotes
                let key = serde_json::from_value(decoded_key).unwrap();
                current_index += key_index;
                if encoded_value.chars().nth(current_index).unwrap() == 'e' {
                    break;
                }
                let current_value = &encoded_value[current_index..encoded_value.len() - 1];
                let (decoded_value, value_index) = decode_bencoded_value(current_value, current_index);
                current_index += value_index;
                map.insert(key, decoded_value);
            }
            (serde_json::Value::Object(map), current_index + 1)
        } else {
            panic!("Unhandled encoded value: {}", encoded_value)
        }
    }




