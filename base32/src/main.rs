use std::process;
use structopt::StructOpt;
use std::path::PathBuf;
use data_encoding;
use std::fs;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "h", long = "help")]
    help: bool,

    #[structopt(short = "d", long = "decode")]
    decode: bool,

    #[structopt(short = "v", long = "version")]
    version: bool,

    #[structopt(name = "FILE",parse(from_os_str))]
    file_name: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    if opt.file_name.len() != 1 || opt.help == true {
        help();
        process::exit(0);
    }

    if opt.version == true {
        version();
        process::exit(0);
    }

    if opt.decode == true {
        let contents = fs::read_to_string(&opt.file_name[0])
            .expect("Could not read file\n");
        
        let decoded_string = data_encoding::BASE32.decode(contents.as_bytes());
        println!("{:?}",decoded_string);
    } else {
        let contents = fs::read_to_string(&opt.file_name[0])
            .expect("Could not read file\n");
        
        let encoded_string = data_encoding::BASE32.encode(contents.as_bytes());
        println!("{:?}",encoded_string);
    }

}

fn version() {
    print!("
base32 (GNU coreutils writen in rust) v1.0.0

Written by Nick Gkloumpos
    ");
}

fn help() {
    print!("
Usage: base32 [OPTION]... [FILE]
Base32 encode or decode FILE, or standard input, to standard output.

With no FILE, or when FILE is -, read standard input.

Mandatory arguments to long options are mandatory for short options too.
    -d, --decode          decode data
    -i, --ignore-garbage  when decoding, ignore non-alphabet characters
    -w, --wrap=COLS       wrap encoded lines after COLS character (default 76).
                            Use 0 to disable line wrapping

        --help     display this help and exit
        --version  output version information and exit

The data are encoded as described for the base32 alphabet in RFC 4648.
When decoding, the input may contain newlines in addition to the bytes of
the formal base32 alphabet.  Use --ignore-garbage to attempt to recover
from any other non-alphabet bytes in the encoded stream.
    ")
}