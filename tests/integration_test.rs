use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn happy_path() -> Result<()> {
    let guesses_file = assert_fs::NamedTempFile::new("guesses.txt")?;

    Command::cargo_bin("wordle-suggest")?
        .assert()
        .success()
        .stdout(
            // Words with repeated characters excluded by default on first guess
            predicate::str::contains("\nmares\n").and(predicate::str::contains("\nsales\n").not()),
        );

    guesses_file.write_str("^s?ales\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", guesses_file.path().to_str().unwrap(), "-n", "50"])
        .assert()
        .success()
        .stdout(
            // After first guess, repeated characters are returned by default
            predicate::str::contains("\nsorra\n"),
        );

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", guesses_file.path().to_str().unwrap(), "-r", "unique"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\nsoapy\n").and(
            // Repeated characters are disallowed with explicit `-r unique`
            predicate::str::contains("\nsorra\n").not(),
        ));

    guesses_file.write_str("^s^u?r^al\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", guesses_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::eq("sugar\n"));

    Ok(())
}
