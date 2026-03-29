use anyhow::{Context, Result};
use boat_lib::{models::activity::NewActivity, repository::activities_repository as activities};
use rusqlite::Connection;

use crate::{
    cli,
    models::{
        TablePrintable,
        activity::{PrintableActivity, SimpleActivity},
        activity_log::PrintableActivityLog,
    },
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

    let simp_act = SimpleActivity::from_db_activity(&created);
    let act = PrintableActivity::from_activity_and_logs(&simp_act, &created.logs);
    if args.use_json_format {
        let json = serde_json::to_string(&act)?;
        println!("{json}");
        return Ok(());
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
        let simp_act = SimpleActivity::from_db_activity(&current);
        let current = PrintableActivity::from_activity_and_logs(&simp_act, &current.logs);
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
        Some(current) => {
            let ongoing_log = current
                .logs
                .iter()
                .find(|l| l.ends_at.is_none())
                .context("there should be an ongoing log")?
                .clone();

            let simp_act = SimpleActivity::from_db_activity(&current);
            let activity_log = PrintableActivityLog::from_activity_and_log(&simp_act, &ongoing_log);

            if args.use_json_format {
                let json = serde_json::to_string(&activity_log)?;
                println!("{json}");
                return Ok(());
            }

            let items = vec![activity_log];
            let table = items.to_printable_table();
            println!("{table}");
        }
        None => println!("no current activity"),
    }
    Ok(())
}

pub fn cancel_current(conn: &mut Connection) -> Result<()> {
    match activities::get_current_ongoing(conn)? {
        Some(act) => {
            activities::cancel_current(conn)?;
            println!("cancelled activity: {act:?}");
        }
        None => {
            println!("no current activity");
        }
    }
    Ok(())
}
