/*

BASE64
Encoding works on a byte array, returns a string

*/

pub fn encode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    return Some(base64::encode(&val).as_bytes().to_vec());
}

pub fn decode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    return Some(base64::decode(&val).unwrap());
}