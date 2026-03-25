use anyhow::{Context, Result};
use boat_lib::{models::activity::NewActivity, repository::activities_repository as activities};
use rusqlite::Connection;

use crate::{
    cli,
    models::{TablePrintable, activity_log::PrintableActivityLog},
};

pub fn create(conn: &mut Connection, args: &cli::CreateActivityArgs) -> Result<()> {
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

pub fn start(conn: &mut Connection, args: &cli::SelectActivityArgs) -> Result<()> {
    if let Some(current) = activities::get_current_ongoing(conn)? {
        println!("stopping activity: {current:?}");
    }

    let act = activities::get_by_id(conn, args.activity_id)?;
    activities::start(conn, args.activity_id)?;
    println!("started activity: {act:?}");
    Ok(())
}

pub fn pause_current(conn: &mut Connection) -> Result<()> {
    if let Some(current) = activities::get_current_ongoing(conn)? {
        let current = PrintableActivityLog::from_activity(&current);
        activities::stop_current(conn)?;
        println!("stopped activity: {current:?}");
    } else {
        println!("no current activity");
    }
    Ok(())
}

pub fn modify(conn: &mut Connection, args: &cli::ModifyActivityArgs) -> Result<()> {
    activities::update(
        conn,
        args.id,
        args.update.name.as_deref(),
        args.update.description.as_deref(),
        args.update.tags.as_deref(),
    )?;
    let act = activities::get_by_id(conn, args.id)?;
    println!("modified activity: {act:?}");
    Ok(())
}

pub fn delete(conn: &mut Connection, args: &cli::SelectActivityArgs) -> Result<()> {
    let act = activities::get_by_id(conn, args.activity_id)?;
    activities::delete(conn, args.activity_id)?;
    println!("deleted activity: {act:?}");
    Ok(())
}

pub fn get_current(conn: &mut Connection, args: &cli::PrintActivityArgs) -> Result<()> {
    let act = activities::get_current_ongoing(conn)?;
    match act {
        Some(v) => {
            let activity_logs = PrintableActivityLog::from_activity(&v);
            let ongoing_log = activity_logs
                .iter()
                .find(|l| l.log.ends_at.is_none())
                .context("there should be an ongoing log")?
                .clone();

            if args.use_json_format {
                let json = serde_json::to_string(&ongoing_log)?;
                println!("{json}");
                return Ok(());
            }

            let items = vec![ongoing_log];
            let table = items.to_printable_table();
            println!("{table}");
        }
        None => println!("no current activity"),
    }
    Ok(())
}
