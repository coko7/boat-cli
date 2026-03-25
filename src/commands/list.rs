use anyhow::Result;
use boat_lib::repository::{activities_repository as activities, tags_repository as tags};
use rusqlite::Connection;
use serde::Serialize;
use std::cmp::Reverse;

use crate::{
    cli,
    models::{
        RowPrintable, TablePrintable, activity::PrintableActivity,
        activity_log::PrintableActivityLog, tag::PrintableTag,
    },
    utils,
};

pub fn list(conn: &mut Connection, command: &cli::ListSubcommand) -> Result<()> {
    match command {
        cli::ListSubcommand::Logs(args) => list_activity_logs(conn, args),
        cli::ListSubcommand::Activities(args) => list_activities(conn, args),
        cli::ListSubcommand::Tags(args) => list_tags(conn, args),
    }
}

pub fn list_printable_items<T: RowPrintable + Serialize>(
    items: Vec<T>,
    show_as_json: bool,
) -> Result<()> {
    if show_as_json {
        let json = serde_json::to_string(&items)?;
        println!("{json}");
        return Ok(());
    }

    let table = items.to_printable_table();
    println!("{table}");
    Ok(())
}

fn list_tags(conn: &mut Connection, args: &cli::ListArgs) -> Result<()> {
    let mut all_tags: Vec<_> = tags::get_all(conn)?
        .iter()
        .map(PrintableTag::from_tag)
        .collect();
    all_tags.sort_by_key(|t| Reverse(t.id));

    list_printable_items(all_tags, args.use_json_format)
}

fn list_activities(conn: &mut Connection, args: &cli::ListArgs) -> Result<()> {
    let mut all_acts: Vec<_> = activities::get_all(conn)?
        .iter()
        .map(PrintableActivity::from_activity)
        .collect();
    all_acts.sort_by_key(|a| Reverse(a.id));

    list_printable_items(all_acts, args.use_json_format)
}

fn list_activity_logs(conn: &mut Connection, args: &cli::ListActivityArgs) -> Result<()> {
    let all: Vec<_> = activities::get_all(conn)?
        .iter()
        .flat_map(PrintableActivityLog::from_activity)
        .filter(|al| match args.period {
            cli::Period::Today => utils::date::is_today(al.log.starts_at),
            cli::Period::Yesterday => utils::date::is_yesterday(al.log.starts_at),
            cli::Period::ThisWeek => utils::date::is_this_week(al.log.starts_at),
            cli::Period::LastWeek => utils::date::is_last_week(al.log.starts_at),
            cli::Period::ThisMonth => utils::date::is_this_month(al.log.starts_at),
            cli::Period::LastMonth => utils::date::is_last_month(al.log.starts_at),
        })
        .collect();

    list_printable_items(all, args.use_json_format)
}
