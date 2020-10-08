use anyhow::Result;
use filetime::FileTime;
use log::debug;
use std::fs::OpenOptions;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<()> {
    pretty_env_logger::init_timed();
    debug!("starting touch");
    let opt = Opt::from_args();
    debug!("got args: {:#?}", opt);
    create_if_not_exists(&opt.file)?;
    set_time(opt.file, FileTime::now())?;
    Ok(())
}

fn create_if_not_exists(file: &PathBuf) -> Result<()> {
    OpenOptions::new().create(true).write(true).open(file)?;
    Ok(())
}

fn set_time(p: PathBuf, time: FileTime) -> Result<()> {
    filetime::set_file_times(p, time, time)?;
    Ok(())
}
