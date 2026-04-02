use anyhow::Result;
use clap::{CommandFactory, Parser};
use log::{LevelFilter, error, info};
use std::process::ExitCode;
use yansi::Paint;

use crate::{cli::Cli, config::Configuration};

mod cli;
mod commands;
mod config;
mod models;
mod utils;

fn main() -> ExitCode {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_module("boat-cli", LevelFilter::Warn)
        .filter_module("boat-lib", LevelFilter::Debug)
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("process cli args");
    match process_args(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e.red());
            ExitCode::FAILURE
        }
    }
}

fn process_args(args: Cli) -> Result<()> {
    info!("getting config file");
    let config_file = config::get_config_file_path()?;
    if !config_file.exists() {
        config::initialize_config()?;
        info!("config file created");
    }

    info!("loading config");
    let config = Configuration::load_from_fs()?;
    let mut conn = boat_lib::utils::init_database(config.database_path)?;

    match &args.command {
        cli::Commands::New(args) => commands::activity::create(&mut conn, args),
        cli::Commands::Start(args) => commands::activity::start(&mut conn, args),
        cli::Commands::Pause => commands::activity::pause_current(&mut conn),
        cli::Commands::Modify(args) => commands::activity::modify(&mut conn, args),
        cli::Commands::Delete(args) => commands::activity::delete(&mut conn, args),
        cli::Commands::Get(args) => commands::activity::get_current(&mut conn, args),
        cli::Commands::HelpExtension => print_help(),
        // cli::Commands::Query { command } => commands::query::query_subcommand(&mut conn, command),
        cli::Commands::Cancel => commands::activity::cancel_current(&mut conn),
        cli::Commands::List(args) => commands::list::list_activities(&mut conn, args),
    }
}

fn print_help() -> Result<()> {
    Cli::command().print_help()?;
    Ok(())
}
