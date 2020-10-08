use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::str::contains;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

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
        .arg(tmp_file_path("test_file"))
        .assert()
        .success();
    assert_eq!(true, Path::new(&tmp_file_path("test_file")).exists());
    fs::remove_file(tmp_file_path("test_file"))?;
    Ok(())
}

fn tmp_file_path<S: Into<String>>(name: S) -> String {
    env::temp_dir()
        .join(Path::new(&name.into()))
        .to_string_lossy()
        .to_string()
}
