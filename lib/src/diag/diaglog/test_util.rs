use deku::reader::Reader;
use std::io::Cursor;

pub fn unhexlify(hexlified_bytes: &str) -> (usize, Reader<Cursor<Vec<u8>>>) {
    let byte_len = hexlified_bytes.len() / 2;
    let bytes = (0..hexlified_bytes.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hexlified_bytes[i..i + 2], 16).unwrap())
        .collect();
    (byte_len, Reader::new(Cursor::new(bytes)))
}
