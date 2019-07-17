use std::str;

/*

HEX
Encoding works on a string (ascii like "Exam"), returns a string (here would be "4578616d")

*/



pub fn encode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    return Some(hex::encode(str::from_utf8(val).unwrap()).as_bytes().to_vec());
}

pub fn decode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    let optstr = _opts.unwrap_or("".to_string());
    let data = querystring::querify(&optstr[..]);
    let mut value = String::from_utf8(val.to_vec()).unwrap();
    for item in data {
        match item.0 {
            "ignore-whitespace" => {
                if item.1 == "true" {
                    value = value.chars().filter(|c| !c.is_whitespace()).collect();
                }
            },
            _ => (),
        }
    }
    return Some(hex::decode(value).unwrap());
}