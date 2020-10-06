use std::process;
use structopt::StructOpt;
use std::env;
// use std::path::Path;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "h", long = "help")]
    help: bool,

    #[structopt(short = "P", long = "physical")]
    physical: bool,

    #[structopt(short = "L", long = "logical")]
    logical: bool,

    #[structopt(short = "v", long = "version")]
    version: bool,
}

fn main() {
    let opt = Opt::from_args();
    
    // Printing help message
    if opt.help == true {
        println!("
        pwd: pwd [-LP]
        Print the name of the current working directory.
        
        Options:
        -L	print the value of $PWD if it names the current working
                directory
        -P	print the physical directory, without any symbolic links
        
        By default, `pwd' behaves as if `-L' were specified.
        
        Exit Status:
        Returns 0 unless an invalid option is given or the current directory
        cannot be read.
    ");
        process::exit(0);
    }

    // If physical argument is false pwd assumes the user needs the logical path
    if opt.physical == true {
        let path = env::current_dir().expect("Insufficient permissions");
        println!("{}", path.display());
        process::exit(0);
    } else {
        let path = env::current_dir()
            .expect("Insufficient permissions")
            .canonicalize().expect("Insufficient permissions");
        println!("{}", path.display());
        process::exit(0);
    }

}
