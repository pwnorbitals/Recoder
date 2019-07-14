extern crate base64;

pub fn encode(val: &[u8]) -> Option<Vec<u8>> {
    return Some(base64::encode(&val).as_bytes().to_vec());
}

pub fn decode(val: &[u8]) -> Option<Vec<u8>> {
    return Some(base64::decode(&val).unwrap());
}