

// #[derive(Debug, Deserialize)]
// pub struct Torrent {
//     info: Info,
//     #[serde(default)]
//     announce: Option<String>,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct Info {
//     pub name: String,
//     pub pieces: ByteBuf,
//     #[serde(rename = "piece length")]
//     pub piece_length: i64,
//     #[serde(default)]
//     pub length: Option<i64>,
// }