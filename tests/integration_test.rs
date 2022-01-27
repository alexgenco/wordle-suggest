use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use anyhow::Result;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn happy_path() -> Result<()> {
    let attempts_file = assert_fs::NamedTempFile::new("attempts.txt")?;

    Command::cargo_bin("wordle-suggest")?
        .assert()
        .success()
        .stdout(predicate::str::contains("\nsales\n"));

    attempts_file.write_str("^s?ales\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", attempts_file.path().to_str().unwrap(), "-n", "50"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\nsural\n"));

    attempts_file.write_str("^s^u?r^al\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", attempts_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::eq("sugar\n"));

    Ok(())
}
