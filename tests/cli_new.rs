//! Integration tests for the `new` CLI command.
use anyhow::Result;
use predicates::prelude::*;

use crate::utils::{cli_args_for_temp, run_boat};

mod utils;

#[test]
fn new_without_name_fails() -> Result<()> {
    let (_tmp, config_path) = cli_args_for_temp()?;

    run_boat(["new"], config_path).failure().stderr(
        predicates::str::contains("error").or(predicates::str::contains("required arguments")),
    );

    Ok(())
}
