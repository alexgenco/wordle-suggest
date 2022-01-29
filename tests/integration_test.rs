use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::{fixture::FileWriteStr, NamedTempFile};
use predicates::{
    boolean::{NotPredicate, PredicateBooleanExt},
    ord::eq,
    prelude::predicate::str::contains,
    str::ContainsPredicate,
};
use std::process::Command;

#[test]
fn happy_path() -> Result<()> {
    Command::cargo_bin("wordle-suggest")?
        .assert()
        .success()
        .stdout(
            // Words with repeated characters excluded by default on first guess
            contains("tones\n").and(excludes("\nsales\n")),
        );

    let (path, file) = tmp_file("attempts.txt")?;

    file.write_str("s^al?es\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path, "-n", "50"])
        .assert()
        .success()
        .stdout(
            // After first guess, repeated characters are returned by default
            contains("\nshell\n"),
        );

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path, "-r", "unique"])
        .assert()
        .success()
        .stdout(contains("\nscale\n").and(
            // Repeated characters are disallowed with explicit `-r unique`
            excludes("\nshell\n"),
        ));

    file.write_str("s^u^r?a^l\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path])
        .assert()
        .success()
        .stdout(eq("sugar\n"));

    Ok(())
}

fn tmp_file(basename: &str) -> Result<(String, NamedTempFile)> {
    let file = assert_fs::NamedTempFile::new(basename)?;
    let path = file.to_string_lossy().into();

    Ok((path, file))
}

fn excludes(s: &str) -> NotPredicate<ContainsPredicate, str> {
    contains(s).not()
}
