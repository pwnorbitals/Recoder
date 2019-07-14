extern crate sha2;
use sha2::Digest;

pub fn encode(val: &[u8]) -> Option<Vec<u8>> {
    return Some(sha2::Sha256::digest(&val).to_vec());
}

pub fn decode(_val: &[u8]) -> Option<Vec<u8>> {
    return None;
}