use anyhow::Result;
use boat_lib::repository::{activities_repository as activities, tags_repository as tags};
use rusqlite::Connection;
use serde::Serialize;
use std::cmp::Reverse;
use tabular::{Row, Table};
use yansi::Paint;

use crate::{
    cli,
    models::{
        RowPrintable, TablePrintable, activity::PrintableActivity,
        activity_with_log::PrintableActivityWithLogs, tag::PrintableTag,
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

fn list_printable_items<T: RowPrintable + Serialize>(
    items: Vec<T>,
    args: &cli::ListArgs,
) -> Result<()> {
    if args.use_json_format {
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

    list_printable_items(all_tags, args)
}

fn list_activities(conn: &mut Connection, args: &cli::ListArgs) -> Result<()> {
    let mut all_acts: Vec<_> = activities::get_all(conn)?
        .iter()
        .map(PrintableActivity::from_activity)
        .collect();
    all_acts.sort_by_key(|a| Reverse(a.id));

    list_printable_items(all_acts, args)
}

fn list_activity_logs(conn: &mut Connection, args: &cli::ListActivityArgs) -> Result<()> {
    let all: Vec<_> = activities::get_all(conn)?
        .iter()
        .map(PrintableActivityWithLogs::from_activity)
        .collect();

    if args.use_json_format {
        let json = serde_json::to_string(&all)?;
        println!("{json}");
    } else {
        let mut table = Table::new("{:>}  {:<}  {:<}  {:<}  {:<}  {:<}  {:<}");
        table.add_row(
            Row::new()
                .with_ansi_cell("ID".underline())
                .with_ansi_cell("Name".underline())
                .with_ansi_cell("Description".underline())
                .with_ansi_cell("Tags".underline())
                .with_ansi_cell("Start".underline())
                .with_ansi_cell("End".underline())
                .with_ansi_cell("Duration".underline()),
        );

        let mut log_lines: Vec<_> = all
            .iter()
            .flat_map(|act| act.logs.iter().zip(std::iter::repeat(act)))
            .filter(|(log, _act)| match args.period {
                cli::Period::Today => utils::is_today(log.starts_at),
                cli::Period::Yesterday => utils::is_yesterday(log.starts_at),
                cli::Period::ThisWeek => utils::is_this_week(log.starts_at),
                cli::Period::LastWeek => utils::is_last_week(log.starts_at),
                cli::Period::ThisMonth => utils::is_this_month(log.starts_at),
                cli::Period::LastMonth => utils::is_last_month(log.starts_at),
            })
            // .filter(|(log, _act)| utils::is_today(log.starts_at))
            .collect();

        log_lines.sort_by_key(|(log, _)| log.starts_at);

        for &(log, act) in log_lines.iter() {
            let mut row = Row::new();
            let values = utils::convert_to_log_line(log, act);
            for val in values.iter() {
                if log.ends_at.is_none() {
                    row = row.with_ansi_cell(val.green());
                } else {
                    row = row.with_cell(val);
                }
            }
            table.add_row(row);
        }
        println!("{table}");
    }

    Ok(())
}
