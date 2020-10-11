use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read, Stdin};
use std::path::Path;

/// Print the first 10 lines of each FILE to standard output.
/// With more than one FILE, precede each with a header giving the file name.
/// With no FILEs: read standard input
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Christohper Morton <sonro@gmx.com>")]
struct Opts {
    /// Files to read
    files: Vec<String>,

    /// Print the first NUM lines instead of the first 10
    #[clap(short = 'n', long, default_value = "10")]
    lines: usize,

    /// Print the first NUM bytes instead of each file
    #[clap(short, long)]
    bytes: Option<usize>,

    /// Always output file name
    #[clap(short, long)]
    verbose: bool,

    /// Never output file name
    #[clap(short, long)]
    quiet: bool,
}

fn main() {
    let opts = Opts::parse();

    match opts.files.len() {
        0 => StdinPrinter::new(&opts).print_stdin(),
        1 => FilePrinter::new(&opts).print_single_file(),
        _ => FilePrinter::new(&opts).print_multi_files(),
    }
}

struct FilePrinter<'a> {
    func: fn(&mut BufReader<File>, usize),
    verbose: bool,
    count: usize,
    files: &'a Vec<String>,
}

impl<'a> FilePrinter<'a> {
    fn new(opts: &'a Opts) -> Self {
        let (func, count) = get_func_and_count(opts);
        Self {
            func,
            count,
            verbose: opts.verbose || !(opts.files.len() == 1 || opts.quiet),
            files: &opts.files,
        }
    }

    fn print_single_file(self) {
        if self.verbose {
            print_path(&self.files[0]);
        }

        self.print_file(&self.files[0]);
    }

    fn print_multi_files(&self) {
        let pre_print = match self.verbose {
            true => |f| print_path(f),
            false => |_| (),
        };

        let n = self.files.len() - 1;
        let files_till_last = self.files.iter().take(n);

        for file in files_till_last {
            pre_print(file);
            self.print_file(file);
            println!();
        }

        pre_print(&self.files[n]);
        self.print_file(&self.files[n]);
    }

    fn print_file(&self, path: &str) {
        if let Some(file) = open_file(path) {
            let mut reader = BufReader::new(file);
            (self.func)(&mut reader, self.count);
        }
    }
}

struct StdinPrinter {
    func: fn(&mut BufReader<Stdin>, usize),
    count: usize,
}

impl StdinPrinter {
    fn new(opts: &Opts) -> Self {
        let (func, count) = get_func_and_count(opts);
        Self { func, count }
    }

    fn print_stdin(&self) {
        let mut reader = BufReader::new(stdin());
        (self.func)(&mut reader, self.count);
    }
}

#[inline]
fn print_lines<T: Read>(reader: &mut BufReader<T>, lines: usize) {
    for line in reader.lines().take(lines) {
        println!("{}", line.unwrap());
    }
}

#[inline]
fn print_bytes<T: Read>(reader: &mut BufReader<T>, bytes: usize) {
    for byte in reader.bytes().take(bytes) {
        print!("{}", byte.unwrap());
    }
}

#[inline]
fn print_path(path: &str) {
    println!("==> {} <==", path);
}

fn open_file(name: &str) -> Option<File> {
    let path = Path::new(name);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            println!("head: error reading '{}': {}", &name, err);
            return None;
        }
    };

    if path.is_dir() {
        println!("head: error reading '{}': Is directory", &name);
        return None;
    }

    Some(file)
}

fn get_func_and_count<T: Read>(opts: &Opts) -> (fn(&mut BufReader<T>, usize), usize) {
    let count;
    let func = match opts.bytes {
        None => {
            count = opts.lines;
            print_lines
        }
        Some(b) => {
            count = b;
            print_bytes
        }
    };

    (func, count)
}
