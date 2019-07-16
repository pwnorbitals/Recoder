use std::str;

/*

HEX
Encoding works on a string (ascii like "Exam"), returns a string (here would be "4578616d")

*/

pub fn encode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    return Some(str::from_utf8(val));
}

pub fn decode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    return Some();
}