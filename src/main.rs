extern crate structopt;
use structopt::StructOpt;
use std::io::{self, Write, Read};

mod algs;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "e", long = "encode")]
    encode: Option<String>,

    #[structopt(short = "d", long = "decode")]
    decode: Option<String>,

    #[structopt(name = "STRING")]
    string: Option<String>
}

fn main() {
    let opt = Opt::from_args();


    let mut input : Vec<u8> = vec![];
    if opt.string.is_none() {
        io::stdin().read_to_end(&mut input).unwrap();
    } else {
        input = opt.string.unwrap().as_bytes().to_vec();
    }

    let result = match (opt.encode, opt.decode) {
        (Some(alg), None) => algs::find(alg).unwrap().0(&input[..]),
        (None, Some(alg)) =>  algs::find(alg).unwrap().1(&input[..]),
        _ => panic!("Bad action"),
    };


    io::stdout().write_all(&result.expect("Couldn't execute the action")[..]).unwrap();
}
