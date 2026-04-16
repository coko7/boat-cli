use anyhow::{Result, bail};
use boat_lib::repository::activities_repository as activities;
use log::info;
use rusqlite::Connection;

use crate::{config::Configuration, utils};

pub fn cancel_current(config: &Configuration, conn: &mut Connection) -> Result<()> {
    match activities::get_current_ongoing(conn)? {
        Some(current) => {
            info!("cancelling current activity: {current:?}");
            activities::cancel_current(conn)?;
            println!("{}", utils::display::cancelled_activity_msg(&current));
        }
        None => {
            bail!(utils::display::no_current_act_msg())
        }
    }
    Ok(())
}
