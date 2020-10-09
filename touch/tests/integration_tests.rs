use anyhow::Result;
use assert_cmd::prelude::*;
use filetime::FileTime;
use predicates::str::contains;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_help_message_when_no_argument_passed() -> Result<()> {
    Command::cargo_bin("touch")?
        .assert()
        .failure()
        .stderr(contains("USAGE"));
    Ok(())
}

#[test]
fn test_run_with_not_existing_file() -> Result<()> {
    Command::cargo_bin("touch")?
        .arg(tmp_file_path("test_file1"))
        .assert()
        .success();
    assert_eq!(true, Path::new(&tmp_file_path("test_file1")).exists());
    fs::remove_file(tmp_file_path("test_file1"))?;
    Ok(())
}

fn tmp_file_path<S: Into<String>>(name: S) -> String {
    env::temp_dir()
        .join(Path::new(&name.into()))
        .to_string_lossy()
        .to_string()
}

#[test]
fn test_run_with_existing_file() -> Result<()> {
    File::create(tmp_file_path("test_file2"))?;
    set_time_from_past(tmp_file_path("test_file2"))?;

    let (orig_mtime, orig_atime) = times(tmp_file_path("test_file2"))?;

    Command::cargo_bin("touch")?
        .arg(tmp_file_path("test_file2"))
        .assert()
        .success();

    let (curr_mtime, curr_atime) = times(tmp_file_path("test_file2"))?;

    assert!(orig_atime < curr_atime);
    assert!(orig_mtime < curr_mtime);
    fs::remove_file(tmp_file_path("test_file2"))?;
    Ok(())
}

fn set_time_from_past<S: Into<String>>(filepath: S) -> Result<()> {
    let d = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let seconds = d.as_secs() - 180;
    let ft = FileTime::from_unix_time(seconds as i64, 0);
    filetime::set_file_times(filepath.into(), ft, ft)?;
    Ok(())
}

fn times<S: Into<String>>(path: S) -> Result<(FileTime, FileTime)> {
    let meta = fs::metadata(tmp_file_path(path))?;
    let mtime = FileTime::from_last_modification_time(&meta);
    let atime = FileTime::from_last_modification_time(&meta);
    Ok((mtime, atime))
}
