use prost::Message;

pub fn serialize_message(msg: impl Message) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    buf
}
