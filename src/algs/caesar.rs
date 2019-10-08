/*

CAESAR
Encoding works on a string, returns a string

*/

static DEFAULT_OFFSET : i32 = 13;

fn exec_rot(val: &[u8], off: i32) -> Option<Vec<u8>> {
    return Some(val.into_iter().map(|c| {
        let base = match c {
            65 ... 90 => 65,
            97 ... 122 => 97,
            _ => c.clone() as i32
        };

        let cur_char = *c as i32;             // current position in ASCII table
        // println!("{:?}", cur_char);
        let cur_pos = cur_char - base;        // current position in alphabet
        // println!("{:?}", cur_pos);
        let offset = off % 26;                // offset in alphabet
        // println!("{:?}", offset);
        let new_pos = cur_pos + offset;       // new position in alphabet (can overflow !)
        // println!("{:?}", new_pos);
        let new_pos_checked = new_pos.abs() % 26;   // new position without overflow
        // println!("{:?}", new_pos_checked);
        let result = new_pos_checked + base;  // new position in ASCII table
        // println!("{:?}", result);

        return result as u8;
    }).collect());
}

pub fn encode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    let optstr = _opts.unwrap_or("".to_string());
    let data = querystring::querify(&optstr[..]);

    let mut offset = DEFAULT_OFFSET;
    for item in data {
        match &item.0.to_lowercase()[..] {
            "offset" => {
                offset = item.1.parse::<i32>().unwrap();
            },
            _ => (),
        }
    }
    
    return exec_rot(val, offset);
}

pub fn decode(val: &[u8], _opts: Option<String>) -> Option<Vec<u8>> {
    let optstr = _opts.unwrap_or("".to_string());
    let data = querystring::querify(&optstr[..]);

    let mut offset = DEFAULT_OFFSET;
    for item in data {
        match &item.0.to_lowercase()[..] {
            "offset" => {
                offset = item.1.parse::<i32>().unwrap();
            },
            _ => (),
        }
    }

    return exec_rot(val, -1 * offset);
}
