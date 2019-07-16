use std::str;

/*

HEX
Encoding works on a string (ascii like "Exam"), returns a string (here would be "4578616d")

*/

pub fn encode(val: &[u8]) -> Option<Vec<u8>> {
    return Some(hex::encode(str::from_utf8(val).unwrap()).as_bytes().to_vec());
}

pub fn decode(val: &[u8]) -> Option<Vec<u8>> {
    return Some(hex::decode(val).unwrap());
}