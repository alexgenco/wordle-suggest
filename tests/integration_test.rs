use anyhow::Result;
use assert_cmd::Command;
use assert_fs::{fixture::FileWriteStr, NamedTempFile};
use predicates::{
    boolean::{NotPredicate, PredicateBooleanExt},
    function::function,
    ord::{eq, gt},
    prelude::predicate::str::contains,
    str::ContainsPredicate,
    Predicate,
};

#[test]
fn happy_path() -> Result<()> {
    Command::cargo_bin("wordle-suggest")?
        .assert()
        .success()
        .stdout(
            // Words with repeated characters and plurals are excluded by default on first hint
            contains("money").and(excludes("sales").and(excludes("fares"))),
        );

    let (path, file) = tmp_file("hints.txt")?;

    file.write_str("mon?ey\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path, "-n", "50"])
        .assert()
        .success()
        .stdout(
            // Words with repeated characters and plurals are allowed after first hint
            contains("signs")
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

#[test]
fn read_hints_from_stdin() -> Result<()> {
    Command::cargo_bin("wordle-suggest")?
        .args(["-f-", "-a"])
        .write_stdin("mon?ey\n")
        .assert()
        .success()
        .stdout(contains("signs").and(excludes("money")));

    Ok(())
}

#[test]
fn read_hints_from_opts() -> Result<()> {
    Command::cargo_bin("wordle-suggest")?
        .args(["-H", "mon?ey", "-H", "cabi^n?", "-a"])
        .assert()
        .success()
        .stdout(
            contains("unzip")
                .and(excludes("money"))
                .and(excludes("cabin")),
        );

    Ok(())
}

#[test]
fn randomize() -> Result<()> {
    Command::cargo_bin("wordle-suggest")?
        .args(["-r", "-n3"])
        .assert()
        .success()
        .stdout(line_count(eq(3)));

    Command::cargo_bin("wordle-suggest")?
        .args(["-r123", "-n2"])
        .assert()
        .success()
        .stdout(eq("wived\ngrebo\n"));

    Command::cargo_bin("wordle-suggest")?
        .args(["-r234", "-n2"])
        .assert()
        .success()
        .stdout(eq("money\nmolar\n"));

    Ok(())
}

#[test]
fn invalid_hint_syntax() -> Result<()> {
    let (path, file) = tmp_file("hints.txt")?;

    file.write_str("mon?ey\nmon!ey\n")?;

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path])
        .assert()
        .failure()
        .stderr(
            contains("Parse error")
                .and(contains("line 2"))
                .and(contains("\"mon!ey\"")),
        )
        .stdout(predicates::str::is_empty());

    Ok(())
}

#[test]
fn missing_hints_file() -> Result<()> {
    let (path, file) = tmp_file("hints.txt")?;

    assert!(!file.exists());

    Command::cargo_bin("wordle-suggest")?
        .args(["-f", &path])
        .assert()
        .failure()
        .stderr(contains("Error: No such file or directory"))
        .stdout(predicates::str::is_empty());

    Ok(())
}

#[test]
fn option_conflicts() -> Result<()> {
    Command::cargo_bin("wordle-suggest")?
        .args(["-a", "-n", "3"])
        .assert()
        .failure()
        .stderr(contains(
            "The argument '--all' cannot be used with '--limit",
        ))
        .stdout(predicates::str::is_empty());

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
