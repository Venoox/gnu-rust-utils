use std::env;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut buf: &str = &args.join(" ");
    if buf.is_empty() {
        buf = "y";
    }
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    loop {
        writeln!(stdout_handle, "{}", buf).ok();
    }
}
