extern crate md5;

/*

MD5
Hashing works on a byte array, returns a byte array

*/

pub fn encode(val: &[u8]) -> Option<Vec<u8>> {
    return Some(md5::compute(val).to_vec());
}

pub fn decode(_val: &[u8]) -> Option<Vec<u8>> {
    return None;
}