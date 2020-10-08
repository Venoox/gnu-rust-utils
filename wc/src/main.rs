use std::process;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "c", long = "bytes")]
    bytes: bool,

    #[structopt(short = "m", long = "chars")]
    chars: bool,

    #[structopt(short = "l", long = "lines")]
    lines: bool,

    #[structopt(short = "L", long = "max-line-length")]
    max_line: bool,

    #[structopt(short = "w", long = "words")]
    words: bool,

    #[structopt(short = "h", long = "help")]
    help: bool,

    #[structopt(short = "v", long = "version")]
    version: bool,

    #[structopt(name = "FILE",parse(from_os_str))]
    file_name: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    if opt.file_name.len() != 1 {
        help();
    }

    let contents = fs::read_to_string(&opt.file_name[0])
        .expect("Something went wrong when reading the file");

    let word_list = contents.split(" ");
    let line_list = contents.split("\n");
    
    let mut word_count: i128 = 0;
    let mut max_line: usize = 0;
    let mut char_count: usize = 0;
    let mut byte_count: usize = 0;
    let mut newline_count: usize = 0;

    for line in line_list {
        newline_count += 1;
        if line.chars().count() >= max_line {
            max_line = line.chars().count();
        }
    }

    for word in word_list {
        char_count += word.chars().count();
        word_count += 1;
        byte_count += word.len();
    }

    if opt.bytes == true {
        println!("{}",byte_count);
        process::exit(0);
    }

    if opt.chars == true {
        println!("{}",char_count);
        process::exit(0);
    }

    if opt.lines == true {
        println!("{}",newline_count);
        process::exit(0);
    }

    if opt.max_line == true {
        println!("{}",max_line);
        process::exit(0);
    }

    if opt.words == true {
        println!("{}",word_count);
        process::exit(0);
    }

    if opt.help == true {
        help();
    }

    process::exit(0);
}

fn help() {
    println!(
    "Usage: wc [OPTION]... [FILE]...

    Print newline, word, and byte counts for each FILE, and a total line if
    more than one FILE is specified.  A word is a non-zero-length sequence of
    characters delimited by white space.
    
    With no FILE, or when FILE is -, read standard input.
    
    The options below may be used to select which counts are printed, always in
    the following order: newline, word, character, byte, maximum line length.
        -c, --bytes            print the byte counts
        -m, --chars            print the character counts
        -l, --lines            print the newline counts
        -L, --max-line-length  print the maximum display width
        -w, --words            print the word counts
        -h, --help     display this help and exit"
    );
    process::exit(0);
}
