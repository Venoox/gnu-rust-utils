use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;

fn file_lines(file_name: &str) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(err) => return Err(err)
    };
    let buf_reader = io::BufReader::new(file);
    Ok(buf_reader.lines())
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    
    for file_name in args.iter() {
        if file_name == "-" {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Can't read from stdin!");
            print!("{}", input);
        }
        else {
            match file_lines(file_name) {
                Ok(lines) => {
                    for line in lines {
                        if let Ok(string) = line {
                            println!("{}", string);
                        }
                    }
                },
                Err(err) => {
                    eprintln!("{}", err);
                    return;
                }
            }
        }
    }
}
