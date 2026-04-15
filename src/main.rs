use anyhow::Result;
use clap::{CommandFactory, Parser};
use log::{LevelFilter, info};
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

    info!("process cli args: {args:?}");
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
    info!("init db connection");
    let mut conn = boat_lib::utils::init_database(config.database_path)?;

    match &args.command {
        cli::Commands::New(args) => commands::create(&mut conn, args),
        cli::Commands::Start(args) => commands::start(&mut conn, args),
        cli::Commands::Cancel => commands::cancel_current(&mut conn),
        cli::Commands::Pause => commands::pause_current(&mut conn),
        cli::Commands::Modify(args) => commands::modify(&mut conn, args),
        cli::Commands::Edit(args) => commands::edit(&mut conn, args),
        cli::Commands::Delete(args) => commands::delete(&mut conn, args),
        cli::Commands::Get(args) => commands::get_current(&conn, args),
        cli::Commands::List(args) => commands::list_activity_logs(&conn, args),
        cli::Commands::Report(list_activity_args) => todo!(),
        cli::Commands::HelpExtension => print_help(),
    }
}

fn print_help() -> Result<()> {
    Cli::command().print_help()?;
    Ok(())
}
