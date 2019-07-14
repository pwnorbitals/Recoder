extern crate urlencoding;
use std::str;


pub fn encode(val: &[u8]) -> Option<Vec<u8>> {
    return Some(urlencoding::encode(str::from_utf8(&val).unwrap()).as_bytes().to_vec());
}

pub fn decode(val: &[u8]) -> Option<Vec<u8>> {
    return Some(urlencoding::decode(str::from_utf8(&val).unwrap()).as_bytes().to_vec());
}