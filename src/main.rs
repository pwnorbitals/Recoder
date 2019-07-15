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

    let input = match opt.string {
        Some(data) => data.as_bytes().to_vec(),
        _ => {
            let mut buf : Vec<u8> = vec![];
            io::stdin().read_to_end(&mut buf).unwrap();
            buf
        }
    };

    let result = match (opt.encode, opt.decode) {
        (Some(alg), None) => algs::find(alg).unwrap().0(&input[..]),
        (None, Some(alg)) =>  algs::find(alg).unwrap().1(&input[..]),
        _ => panic!("Bad action"),
    };


    io::stdout().write_all(&result.expect("Couldn't execute the action")[..]).unwrap();
}
