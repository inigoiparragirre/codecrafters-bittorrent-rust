


struct Frame {};


impl Decoder for Frame {
    type Item = PeerMessage;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<PeerMessage>, io::Error> {
        if src.len() < 4 {
            return Ok(None);
        }

        let length = BigEndian::read_u32(&src[0..4]) as usize;

        if src.len() < length + 4 {
            return Ok(None);
        }

        let id = src[4];
        let payload = src[5..length + 4].to_vec();

        src.advance(length + 4);

        Ok(Some(PeerMessage {
            length: length as u32,
            id,
            payload,
        }))
    }
}