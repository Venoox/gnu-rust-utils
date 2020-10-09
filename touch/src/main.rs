use anyhow::Result;
use filetime::FileTime;
use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use std::process;

fn main() -> Result<()> {
    let path = env::args().skip(1).next().unwrap_or_else(|| {
        print_usage();
        process::exit(1);
    });
    create_if_not_exists(&path)?;
    set_time(path, FileTime::now())?;
    Ok(())
}

fn print_usage() {
    println!(r#"usage"#);
}

fn create_if_not_exists<P: AsRef<Path>>(file: &P) -> Result<()> {
    OpenOptions::new().create(true).write(true).open(file)?;
    Ok(())
}

fn set_time<P: AsRef<Path>>(p: P, time: FileTime) -> Result<()> {
    filetime::set_file_times(p, time, time)?;
    Ok(())
}
