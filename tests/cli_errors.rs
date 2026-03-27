//! Tests for error/failure scenarios in the CLI
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn cli_args_for_temp() -> (TempDir, String) {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("boat.db");
    let config_path = tmp.path().join("boat_config.toml");
    fs::write(
        &config_path,
        format!("database_path = {:?}", db_path.display()),
    )
    .unwrap();
    (tmp, config_path.display().to_string())
}

#[test]
fn new_without_name_should_fail() {
    let (_tmp, config_path) = cli_args_for_temp();
    let mut cmd = Command::cargo_bin("boat").unwrap();
    cmd.env("BOAT_CONFIG", &config_path).arg("new");

    // purposely omit activity name
    cmd.assert().failure().stderr(
        predicates::str::contains("error").or(predicates::str::contains("required arguments")),
    );
}

#[test]
fn list_mutually_exclusive_args_should_fail() {
    let (_tmp, config_path) = cli_args_for_temp();
    let mut cmd = Command::cargo_bin("boat").unwrap();

    cmd.env("BOAT_CONFIG", &config_path)
        .arg("list")
        .arg("--period")
        .arg("today")
        .arg("--date")
        .arg("2024-05-01");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("cannot be used with"));
}

#[test]
fn list_with_invalid_date_input_should_fail() {
    let (_tmp, config_path) = cli_args_for_temp();
    let mut cmd = Command::cargo_bin("boat").unwrap();

    cmd.env("BOAT_CONFIG", &config_path)
        .arg("list")
        .arg("--date")
        .arg("not-a-date");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("invalid date"));
}
