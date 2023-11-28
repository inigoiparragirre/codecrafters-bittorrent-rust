use std::io::Read;


use crate::decode;
use crate::torrent::Torrent;
use crate::de::{from_bytes};
use crate::decode::ParseDecode;



fn main() {
    // let stdin = io::stdin();
    let mut buffer = Vec::new();
    // let mut handle = stdin.lock();
    // Read torrent from file
    let mut handle = std::fs::File::open("example.torrent").unwrap();


    match handle.read_to_end(&mut buffer) {
        Ok(_) => match from_bytes::<Torrent>(&buffer) {
            Ok(t) => render_torrent(&t),
            Err(e) => println!("ERROR: {e:?}"),
        },
        Err(e) => println!("ERROR: {e:?}"),
    }
}


fn render_torrent(torrent: &Torrent) {
    println!("name:\t\t{}", torrent.info.name);
    println!("announce:\t{:?}", torrent.announce);
}






