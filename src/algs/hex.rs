/*

HEX
Encoding works on a string (ascii like "Exam"), returns a string (here would be "4578616d")

*/

use unicode_segmentation::UnicodeSegmentation;
use std::convert::TryInto;


fn remove_whitespace(s: String) -> String {
    s.split_whitespace().collect::<String>()
}


pub fn encode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    let optstr = _opts.unwrap_or("".to_string());
    let data = querystring::querify(&optstr[..]);

    let mut separator = "".to_string();
    let mut value = String::from_utf8(val.to_vec()).unwrap();
    for item in data {
        match &item.0.to_lowercase()[..] {
            "ignore-whitespace" => {
                if item.1 == "true" {
                    value = remove_whitespace(value);
                }
            },
            "separator" => {
                separator = item.1.to_string();
            },
            _ => (),
        }
    }

    let mut result : Vec<char> = hex::encode(value).chars().collect();

    // https://stackoverflow.com/questions/27996430/reversing-a-string-in-rust
    let sep_rev = separator.graphemes(true).rev();

    let mut i = 0i32;
    let mut last_cut = -1i32;
    loop {
        if i >= (result.len()-1).try_into().unwrap() {
            break;
        }

        if i == last_cut + 2 {
            last_cut = i as i32;
            for graph in sep_rev.clone().flat_map(|g| g.chars()) {
                result.insert((i+1).try_into().unwrap(), graph);
                last_cut += 1;
            }
            
        }

        i += 1;
    }

    return Some(result.into_iter().collect::<String>().as_bytes().to_vec());
}

pub fn decode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    let optstr = _opts.unwrap_or("".to_string());
    let data = querystring::querify(&optstr[..]);
    let mut value = String::from_utf8(val.to_vec()).unwrap();
    for item in data {
        match &item.0.to_lowercase()[..] {
            "ignore-whitespace" => {
                if item.1 == "true" {
                    value = value.chars().filter(|c| !c.is_whitespace()).collect();
                }
            },
            // TODO : Support separator for hex decoding
            "separator" => {

            },
            _ => (),
        }
    }
    return Some(hex::decode(value).unwrap());
}