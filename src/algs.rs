mod md5;
mod base64;
mod sha1;
mod sha256;
mod url;


pub type EncDecFct = fn (&[u8]) -> Option<Vec<u8>>;

pub fn find(alg:String) -> Option<(&'static EncDecFct, &'static EncDecFct)> {

    for val in ALG_TABLE {
        for alg_str in val.0 {
            if alg_str == &alg {
                return Some((&val.1, &val.2));
            }
        }
    }

    return None;
}



static ALG_TABLE: &'static [(&[&'static str], EncDecFct, EncDecFct)] = &[
    (&["md5", "md-5", "MD5"], md5::encode, md5::decode),
    (&["sha1", "sha-1", "SHA1"], sha1::encode, sha1::decode),
    (&["base64", "b64", "BASE64", "B64"], base64::encode, base64::decode),
    (&["urlencode", "url", "urlencoding", "URL"], url::encode, url::decode),
    (&["sha256", "SHA256", "sha2"], sha256::encode, sha256::decode),

    /* TODO :
        ascii_hex
        ascii_bin
        ascii_dec
        ascii_oct
        utf-8
        offset
        cesar


    */
];