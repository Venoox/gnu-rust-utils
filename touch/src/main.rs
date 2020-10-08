use anyhow::Result;
use libc::{time_t, timespec};
use log::debug;
use std::ffi::CString;
use std::io;
use std::os::unix::prelude::*;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
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
    let ct = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let (seconds, nanos) = (ct.as_secs(), ct.as_nanos() as u32);
    set_time(opt.file, FileTime { seconds, nanos })?;
    Ok(())
}

fn set_time(p: PathBuf, time: FileTime) -> io::Result<()> {
    debug!("setting times: {:#?}", time);
    let p = CString::new(p.as_os_str().as_bytes())?;
    let times = [to_timespec(time), to_timespec(time)];
    let rc = unsafe {
        libc::syscall(
            libc::SYS_utimensat,
            libc::AT_FDCWD,
            p.as_ptr(),
            times.as_ptr(),
            0,
        )
    };
    if rc != 0 {
        debug!("result of syscall: {}", rc);
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn to_timespec(ft: FileTime) -> timespec {
    libc::timespec {
        tv_sec: ft.seconds() as time_t,
        tv_nsec: ft.nanoseconds() as _,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FileTime {
    seconds: u64,
    nanos: u32,
}

impl FileTime {
    pub fn seconds(&self) -> u64 {
        self.seconds
    }

    pub fn nanoseconds(&self) -> u32 {
        self.nanos
    }
}
