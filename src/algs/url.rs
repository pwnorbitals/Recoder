use std::str;

/*

URL ENCODING
Encoding works on an utf-8 string, returns an utf-8 string

*/


pub fn encode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    return Some(urlencoding::encode(str::from_utf8(&val).unwrap()).as_bytes().to_vec());
}

pub fn decode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    return Some(urlencoding::decode(str::from_utf8(&val).unwrap()).as_bytes().to_vec());
}