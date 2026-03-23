use anyhow::Result;
use boat_lib::models::activity::NewActivity;
use clap::Parser;
use log::{LevelFilter, error, info};
use rusqlite::Connection;
use std::process::ExitCode;

use boat_lib::repository::activities_repository as activities;

use crate::{
    cli::{
        Cli, CreateActivityArgs, ListActivityArgs, ModifyActivityArgs, PrintActivityArgs,
        SelectActivityArgs,
    },
    models::SimpleActivity,
};

mod cli;
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
            error!("{}", e);
            ExitCode::FAILURE
        }
    }
}

fn process_args(args: Cli) -> Result<()> {
    let mut conn = boat_lib::utils::init_database("boat.db")?;

    match &args.command {
        cli::Commands::New(args) => new_activity(&mut conn, args),
        cli::Commands::Start(args) => start_activity(&mut conn, args),
        cli::Commands::Pause => pause_current(&mut conn),
        cli::Commands::Modify(args) => modify_activity(&mut conn, args),
        cli::Commands::Delete(args) => delete_activity(&mut conn, args),
        cli::Commands::Get(args) => get_current(&mut conn, args),
        cli::Commands::List(args) => list_activities(&mut conn, args),
    }
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

fn pause_current(conn: &mut Connection) -> std::result::Result<(), anyhow::Error> {
    activities::stop_current(conn)?;
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
            let act = SimpleActivity::from_activity(&v);
            if args.use_json_format {
                let json = serde_json::to_string(&act)?;
                println!("{json}");
            } else {
                println!("{act}");
            }
        }
        None => println!("no current activity"),
    }
    Ok(())
}

fn list_activities(conn: &mut Connection, args: &ListActivityArgs) -> Result<()> {
    let all: Vec<_> = activities::get_all(conn)?
        .iter()
        .map(SimpleActivity::from_activity)
        .collect();

    if args.use_json_format {
        let json = serde_json::to_string(&all)?;
        println!("{json}");
    } else {
        for act in all.iter() {
            println!("{act}");
        }
    }

    Ok(())
}
