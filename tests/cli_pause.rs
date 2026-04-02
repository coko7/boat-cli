//! Integration tests for the `pause` CLI command.
use anyhow::Result;

use crate::utils::{cli_args_for_temp, run_boat};

mod utils;

#[test]
fn pause_when_nothing_running_fails() -> Result<()> {
    let (_tmp, config_path) = cli_args_for_temp()?;

    run_boat(["pause"], config_path)
        .success()
        .stdout(predicates::str::contains("no current activity"));

    Ok(())
}
