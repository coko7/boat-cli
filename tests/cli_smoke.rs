//! Basic smoke tests: help, version, invalid command
use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;

#[test]
fn test_help_arg() {
    let mut cmd = Command::cargo_bin("boat").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Usage").or(predicates::str::contains("USAGE")));
}

#[test]
fn test_help_subcommand_short_alias() {
    let mut cmd = Command::cargo_bin("boat").unwrap();
    cmd.arg("h");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Usage").or(predicates::str::contains("USAGE")));
}

#[test]
fn test_version_arg() {
    let mut cmd = Command::cargo_bin("boat").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("boat"));
}

#[test]
fn test_unknown_subcommand_fails() {
    let mut cmd = Command::cargo_bin("boat").unwrap();
    cmd.arg("definitely-not-a-command");
    cmd.assert().failure().stderr(
        predicates::str::contains("error").or(predicates::str::contains("not a valid subcommand")),
    );
}
