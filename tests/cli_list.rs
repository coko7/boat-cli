//! Integration tests for the `list` CLI command.
use anyhow::Result;

use crate::utils::{cli_args_for_temp, run_boat};

mod utils;

#[test]
fn list_with_invalid_date_input_fails() -> Result<()> {
    let (_tmp, config_path) = cli_args_for_temp()?;

    run_boat(["list", "--period", "not-a-date"], config_path)
        .failure()
        .stderr(predicates::str::contains("Provide either a range"));

    Ok(())
}
