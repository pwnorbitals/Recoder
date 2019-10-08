mod md5;
mod base64;
mod sha1;
mod sha256;
mod url;
mod hex;
mod caesar;


pub type EncDecFct = fn (&[u8], Option<String>) -> Option<Vec<u8>>;

pub fn find(alg:String) -> Option<(&'static EncDecFct, &'static EncDecFct)> {

    for val in ALG_TABLE {
        for alg_str in val.0 {
            if alg_str == &alg.to_lowercase() {
                return Some( (&val.1, &val.2) );
            }
        }
    }

    return None;
}



static ALG_TABLE: &'static [(&[&'static str], EncDecFct, EncDecFct)] = &[
    (&["md5", "md-5"], md5::encode, md5::decode),
    (&["sha1", "sha-1"], sha1::encode, sha1::decode),
    (&["base64", "b64", "base-64", "b-64"], base64::encode, base64::decode),
    (&["urlencode", "url", "urlencoding"], url::encode, url::decode),
    (&["sha256", "sha2", "sha-256", "sha-2"], sha256::encode, sha256::decode),
    (&["hex"], hex::encode, hex::decode),
    (&["caesar", "offset", "rot", "off", "rotate"], caesar::encode, caesar::decode),

    /* TODO :
        bin
        dec
        oct


    */
];