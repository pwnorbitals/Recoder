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

    #[structopt(short = "o", long = "options")]
    options: Option<String>,

    #[structopt(name = "STRING")]
    string: Option<String>
}

fn main() {
    // Parse arguments
    let opt = Opt::from_args();

    // Take input
    let input = match opt.string {
        // From command-line
        Some(data) => data.as_bytes().to_vec(),   

        // From stdin
        _ => {
            let mut buf : Vec<u8> = vec![];             
            io::stdin().read_to_end(&mut buf).unwrap(); 
            buf
        }
    };

    // Compute the result
    let result = match (opt.encode, opt.decode) {
        // Encode
        (Some(alg), None) => algs::find(alg).unwrap().0(&input[..], opt.options),

        // Decode
        (None, Some(alg)) =>  algs::find(alg).unwrap().1(&input[..], opt.options),

        // None or both ?!
        _ => panic!("Bad action"),
    };

    // Print the result
    io::stdout().write_all(&result.expect("Couldn't execute the action")[..]).unwrap();
}
