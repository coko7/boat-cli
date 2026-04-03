use anyhow::{Result, bail};
use boat_lib::repository::activities_repository as activities;
use chrono::Local;
use log::info;
use rusqlite::Connection;
use yansi::Paint;

use crate::{cli, commands::pause_current, utils};

pub fn start(conn: &mut Connection, args: &cli::SelectActivityArgs) -> Result<()> {
    let Ok(to_start) = activities::get_by_id(conn, args.activity_id) else {
        info!("cannot start because ID is invalid: {}", args.activity_id);
        bail!(utils::display::invaid_activity_id(args.activity_id));
    };

    if let Some(current) = activities::get_current_ongoing(conn)? {
        info!("ongoing activity: {current:?}");

        if current.id == args.activity_id {
            info!("not starting because same activity already ongoing");
            println!("{}", "activity already ongoing...".italic());
            return Ok(());
        }

        info!("pausing current...");
        pause_current(conn)?;
    }

    info!("about to start: {to_start:?}");
    activities::start(conn, args.activity_id)?;
    println!(
        "{}",
        utils::display::started_activity_msg(&to_start, Local::now())
    );

    Ok(())
}
