use anyhow::{Context, Result};
use boat_lib::models::activity::NewActivity;
use boat_lib::repository::activities_repository as activities;
use clap::{CommandFactory, Parser};
use log::{LevelFilter, error, info};
use rusqlite::Connection;
use std::process::ExitCode;
use tabular::{Row, Table};

use crate::cli::ModifyActivityArgs;
use crate::{
    cli::{Cli, CreateActivityArgs, PrintActivityArgs, SelectActivityArgs},
    config::Configuration,
    models::{RowPrintable, activity_with_log::PrintableActivityWithLogs},
};

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
        // .filter_level(args.verbose.log_level_filter())
        .init();

    info!("process cli args");
    match process_args(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            error!("{}", e);
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
        cli::Commands::New(args) => new_activity(&mut conn, args),
        cli::Commands::Start(args) => start_activity(&mut conn, args),
        cli::Commands::Pause => pause_current(&mut conn),
        cli::Commands::Modify(args) => modify_activity(&mut conn, args),
        cli::Commands::Delete(args) => delete_activity(&mut conn, args),
        cli::Commands::Get(args) => get_current(&mut conn, args),
        // cli::Commands::Config {} => todo!(),
        cli::Commands::HelpExtension => print_help(),
        cli::Commands::List { command } => commands::list::list(&mut conn, command),
    }
}

fn print_help() -> Result<()> {
    Cli::command().print_help()?;
    Ok(())
}

fn new_activity(conn: &mut Connection, args: &CreateActivityArgs) -> Result<()> {
    let new_activity = NewActivity {
        name: args.name.clone(),
        description: args.description.clone(),
        tags: args.tags.clone(),
    };

    let created = activities::create(conn, new_activity)?;
    if args.auto_start {
        activities::start(conn, created.id)?;
    }

    println!("{}", created.id);
    Ok(())
}

fn start_activity(conn: &mut Connection, args: &SelectActivityArgs) -> Result<()> {
    activities::start(conn, args.activity_id)?;
    Ok(())
}

fn pause_current(conn: &mut Connection) -> Result<()> {
    if let Some(current) = activities::get_current_ongoing(conn)? {
        let current = PrintableActivityWithLogs::from_activity(&current);
        activities::stop_current(conn)?;
        println!("stopped activity: {current:?}");
    } else {
        println!("no current activity");
    }
    Ok(())
}

fn modify_activity(conn: &mut Connection, args: &ModifyActivityArgs) -> Result<()> {
    activities::update(
        conn,
        args.id,
        args.update.name.as_deref(),
        args.update.description.as_deref(),
        args.update.tags.as_deref(),
    )?;
    Ok(())
}

fn delete_activity(conn: &mut Connection, args: &SelectActivityArgs) -> Result<()> {
    activities::delete(conn, args.activity_id)?;
    Ok(())
}

fn get_current(conn: &mut Connection, args: &PrintActivityArgs) -> Result<()> {
    let act = activities::get_current_ongoing(conn)?;
    match act {
        Some(v) => {
            let act = PrintableActivityWithLogs::from_activity(&v);
            if args.use_json_format {
                let json = serde_json::to_string(&act)?;
                println!("{json}");
            } else {
                let mut table = Table::new(&PrintableActivityWithLogs::row_spec());
                table.add_row(
                    Row::new()
                        .with_cell("ID")
                        .with_cell("Name")
                        .with_cell("Description")
                        .with_cell("Tags")
                        .with_cell("Start")
                        .with_cell("End")
                        .with_cell("Duration"),
                );
                let log = act
                    .logs
                    .iter()
                    .find(|l| l.ends_at.is_none())
                    .context("there should be an active one")?;

                let log_line = utils::convert_to_log_line(log, &act);
                let mut row = Row::new();
                for val in log_line.iter() {
                    row = row.with_cell(val);
                }
                table.add_row(row);
                println!("{table}");
            }
        }
        None => println!("no current activity"),
    }
    Ok(())
}
