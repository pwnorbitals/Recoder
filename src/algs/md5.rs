extern crate md5;

pub fn encode(val: &[u8]) -> Option<Vec<u8>> {
    return Some(format!("{:x}", md5::compute(val)).as_bytes().to_vec());
}

pub fn decode(_val: &[u8]) -> Option<Vec<u8>> {
    return None;
}