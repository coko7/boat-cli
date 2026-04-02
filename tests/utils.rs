use anyhow::Result;
use assert_cmd::Command;
use tempfile::TempDir;

pub fn cli_args_for_temp() -> Result<(TempDir, std::path::PathBuf)> {
    let tmp = TempDir::new()?;
    let db_path = tmp.path().join("boat.db");
    let config_path = tmp.path().join("boat_config.toml");

    std::fs::write(
        &config_path,
        format!("database_path = {:?}", db_path.display()),
    )?;
    Ok((tmp, config_path))
}

pub fn run_boat<I, S, P>(args: I, config_path: P) -> assert_cmd::assert::Assert
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
    P: AsRef<std::ffi::OsStr>,
{
    Command::cargo_bin("boat")
        .unwrap()
        .env("BOAT_CONFIG", config_path)
        .args(args)
        .assert()
}
