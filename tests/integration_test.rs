use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::{fixture::FileWriteStr, NamedTempFile};
use predicates::{
    boolean::{NotPredicate, PredicateBooleanExt},
    function::function,
    ord::{eq, gt},
    prelude::predicate::str::contains,
    str::ContainsPredicate,
    Predicate,
};
use std::process::Command;

#[test]
fn happy_path() -> Result<()> {
    Command::cargo_bin("wordle-suggest")?
        .assert()
        .success()
        .stdout(
            // Words with repeated characters and plurals are excluded by default on first
            // guess
            contains("money").and(excludes("sales").and(excludes("fares"))),
        );

    let (path, file) = tmp_file("attempts.txt")?;

    file.write_str("mon?ey\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path, "-n", "50"])
        .assert()
        .success()
        .stdout(
            // Words with repeated characters are allowed after first guess
            contains("teens")
                // Plural words are allowed after first guess
                .and(contains("beans"))
                // Option `-n 50` ensure we get 50 results back
                .and(line_count(eq(50))),
        );

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path, "--unique", "--singular"])
        .assert()
        .success()
        .stdout(
            contains("cabin")
                .and(
                    // Plural words are disallowed with explicit `--singular`
                    excludes("beans"),
                )
                .and(
                    // Repeated characters are disallowed with explicit `--unique`
                    excludes("teens"),
                ),
        );

    file.write_str("cabi^n?")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path, "--all"])
        .assert()
        .success()
        .stdout(
            // Option `--all` removes limit from returned results
            line_count(gt(10)).and(
                // Contains the solution
                contains("unzip"),
            ),
        );

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

fn line_count(p: impl Predicate<usize>) -> impl Predicate<str> {
    function(move |stdout: &str| p.eval(&stdout.lines().count()))
}
