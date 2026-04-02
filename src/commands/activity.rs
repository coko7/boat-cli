use anyhow::{Context, Result, bail};
use boat_lib::{models::activity::NewActivity, repository::activities_repository as activities};
use chrono::Local;
use rusqlite::Connection;
use yansi::Paint;

use crate::{
    cli,
    models::{
        activity::{PrintableActivity, SimpleActivity},
        activity_log::PrintableActivityLog,
    },
    utils,
};

pub fn create(conn: &mut Connection, args: &cli::CreateActivityArgs) -> Result<()> {
    let new_activity = NewActivity {
        name: args.name.clone(),
        description: args.description.clone(),
        tags: args.tags.clone(),
    };

    let created = activities::create(conn, new_activity)?;
    if !args.use_json_format {
        println!("{}", utils::display::created_activity_msg(&created)?);
    }

    if args.auto_start {
        activities::start(conn, created.id)?;
        if !args.use_json_format {
            println!(
                "{}",
                utils::display::started_activity_msg(&created, Local::now())
            );
        }
    }

    let simp_act = SimpleActivity::from_db_activity(&created);
    let act = PrintableActivity::from_activity_and_logs(&simp_act, &created.logs);
    if args.use_json_format {
        let json = serde_json::to_string(&act)?;
        println!("{json}");
    }

    Ok(())
}

pub fn start(conn: &mut Connection, args: &cli::SelectActivityArgs) -> Result<()> {
    if let Some(current) = activities::get_current_ongoing(conn)? {
        if current.id == args.activity_id {
            println!("{}", "activity already ongoing...".italic());
            return Ok(());
        }

        pause_current(conn)?;
    }

    let Ok(act) = activities::get_by_id(conn, args.activity_id) else {
        bail!(utils::display::invaid_activity_id(args.activity_id));
    };
    activities::start(conn, args.activity_id)?;

    println!(
        "{}",
        utils::display::started_activity_msg(&act, Local::now())
    );
    Ok(())
}

pub fn pause_current(conn: &mut Connection) -> Result<()> {
    if let Some(current) = activities::get_current_ongoing(conn)? {
        activities::stop_current(conn)?;
        println!(
            "{}",
            utils::display::paused_activity_msg(&current, Local::now())?
        );
    } else {
        println!("{}", utils::display::no_current_act_msg());
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

    let Ok(act) = activities::get_by_id(conn, args.id) else {
        bail!(utils::display::invaid_activity_id(args.id));
    };

    println!("{}", utils::display::modified_activity_msg(&act));
    Ok(())
}

pub fn delete(conn: &mut Connection, args: &cli::SelectActivityArgs) -> Result<()> {
    let Ok(act) = activities::get_by_id(conn, args.activity_id) else {
        bail!(utils::display::invaid_activity_id(args.activity_id));
    };

    activities::delete(conn, args.activity_id)?;
    println!("{}", utils::display::deleted_activity_msg(&act));
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

            println!("{}", utils::display::current_activity_msg(&current)?);
        }
        None => println!("{}", utils::display::no_current_act_msg()),
    }
    Ok(())
}

pub fn cancel_current(conn: &mut Connection) -> Result<()> {
    match activities::get_current_ongoing(conn)? {
        Some(act) => {
            activities::cancel_current(conn)?;
            println!("{}", utils::display::cancelled_activity_msg(&act));
        }
        None => {
            println!("{}", utils::display::no_current_act_msg())
        }
    }
    Ok(())
}
