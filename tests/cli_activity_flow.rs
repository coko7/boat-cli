//! Test basic activity CRUD and flow in the CLI
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to spin up a temp config + db directory and return required CLI args
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
fn can_create_start_pause_list_activity() {
    let (_tmp, config_path) = cli_args_for_temp();

    // boat new
    let mut cmd = Command::cargo_bin("boat").unwrap();
    cmd.env("BOAT_CONFIG", &config_path)
        .arg("new")
        .arg("TestTask");
    cmd.assert().success();

    // boat start <ID: always 1 for first activity>
    let mut cmd = Command::cargo_bin("boat").unwrap();
    cmd.env("BOAT_CONFIG", &config_path).arg("start").arg("1");
    cmd.assert().success();

    // boat pause
    let mut cmd = Command::cargo_bin("boat").unwrap();
    cmd.env("BOAT_CONFIG", &config_path).arg("pause");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("paused"));

    // boat list --json, just check output contains the activity name 'TestTask'
    let mut cmd = Command::cargo_bin("boat").unwrap();
    cmd.env("BOAT_CONFIG", &config_path)
        .arg("list")
        .arg("--json");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("TestTask"));
}
