
/// BencodeValue is an enum that represents all possible values that can be
/// encoded in bencode.


enum BencodeValue {
    BString(Vec<u8>),
    BInteger(i64),
    BList(Vec<BencodeValue>),
    BDictionary(HashMap<Vec<u8>, BencodeValue>),
}



}