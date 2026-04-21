use anyhow::{Result, bail};
use boat_lib::repository::activities_repository as activities;
use log::info;
use rusqlite::Connection;

use crate::{cli, config::Configuration, utils};

pub fn delete(
    config: &Configuration,
    conn: &mut Connection,
    args: &cli::SelectActivityArgs,
) -> Result<()> {
    let Ok(to_delete) = activities::get_by_id(conn, args.activity_id) else {
        info!("cannot delete because ID is invalid: {}", args.activity_id);
        bail!(utils::display::activity_id_does_not_exist(args.activity_id));
    };

    info!("deleting activity: {to_delete:?}");
    activities::delete(conn, args.activity_id)?;
    println!("{}", utils::display::deleted_activity_msg(&to_delete));

    Ok(())
}
