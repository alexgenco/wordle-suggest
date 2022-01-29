use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::{prelude::*, NamedTempFile};
use predicates::prelude::*;
use std::process::Command;

#[test]
fn happy_path() -> Result<()> {
    Command::cargo_bin("wordle-suggest")?
        .assert()
        .success()
        .stdout(
            // Words with repeated characters excluded by default on first guess
            predicate::str::contains("\nmares\n").and(predicate::str::contains("\nsales\n").not()),
        );

    let (path, file) = tmp_file("attempts.txt")?;

    file.write_str("^s?ales\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path, "-n", "50"])
        .assert()
        .success()
        .stdout(
            // After first guess, repeated characters are returned by default
            predicate::str::contains("\nsorra\n"),
        );

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path, "-r", "unique"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\nsoapy\n").and(
            // Repeated characters are disallowed with explicit `-r unique`
            predicate::str::contains("\nsorra\n").not(),
        ));

    file.write_str("^s^u?r^al\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path])
        .assert()
        .success()
        .stdout(predicate::eq("sugar\n"));

    Ok(())
}

fn tmp_file(basename: &str) -> Result<(String, NamedTempFile)> {
    let file = assert_fs::NamedTempFile::new(basename)?;
    let path = file.to_string_lossy().into();

    Ok((path, file))
}
