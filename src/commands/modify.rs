use anyhow::{Result, bail};
use boat_lib::repository::activities_repository as activities;
use log::info;
use rusqlite::Connection;

use crate::{cli, config::Configuration, utils};

pub fn modify(
    config: &Configuration,
    conn: &mut Connection,
    args: &cli::ModifyActivityArgs,
) -> Result<()> {
    let Ok(old) = activities::get_by_id(conn, args.id) else {
        info!("cannot modify because ID is invalid: {}", args.id);
        bail!(utils::display::invalid_activity_id(args.id));
    };

    info!("about to modify activity: {old:?}");
    activities::update(
        conn,
        args.id,
        args.update.name.as_deref(),
        args.update.description.as_deref(),
        args.update.tags.as_deref(),
    )?;

    let modified = activities::get_by_id(conn, args.id)?;
    info!("modified activity: {modified:?}");
    println!("{}", utils::display::modified_activity_msg(&modified));

    Ok(())
}
