extern crate structopt;
use structopt::StructOpt;
use std::io::{self, Write};

mod algs;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "e", long = "encode")]
    encode: Option<String>,

    #[structopt(short = "d", long = "decode")]
    decode: Option<String>,

    #[structopt(name = "STRING")]
    string: String
}


fn encode(alg: String, value: &[u8]) -> Option<Vec<u8>> {   
    return algs::find(alg).unwrap().0(value);
}

fn decode(alg: String, value: &[u8]) -> Option<Vec<u8>> {
    return algs::find(alg).unwrap().1(value);
}

fn main() {
    let opt = Opt::from_args();
    
    if opt.encode.is_some() && opt.decode.is_some() {
        panic!("Cannot decode and encode");
    } else if opt.encode.is_none() && opt.decode.is_none() {
        panic!("Need an action")
    } 

    let mut result: Option<Vec<u8>> = None;
    if opt.encode.is_some() {
        let alg = opt.encode.unwrap();
        result = encode(alg, opt.string.as_bytes());
    }
    else if opt.decode.is_some() {
        let alg = opt.decode.unwrap();
        result = decode(alg, opt.string.as_bytes());
    }

    io::stdout().write_all(&result.expect("Couldn't execute the action")[..]).unwrap();
}