

enum ParseResult {
    Integer(i64),
    Bytes(Vec<u8>),
    List,
    Dictionary,
    End,
}