use anyhow::Result;
use boat_lib::repository::activities_repository as activities;
use chrono::Local;
use rusqlite::Connection;
use std::{cmp::Reverse, collections::BTreeMap};

use crate::{
    cli::{self, Period},
    models::{activity::PrintableActivity, activity_log::PrintableActivityLog},
    utils::{self, date::DateTimeRenderMode},
};

pub fn list_activities(conn: &mut Connection, args: &cli::ListActivityArgs) -> Result<()> {
    if args.activities_only {
        return list_activities_only(conn, args.use_json_format);
    }

    list_activity_logs(conn, args)
}

fn list_activities_only(conn: &mut Connection, use_json: bool) -> Result<()> {
    let mut all_acts: Vec<_> = activities::get_all(conn)?
        .iter()
        .map(PrintableActivity::from_activity)
        .collect();
    all_acts.sort_by_key(|a| Reverse(a.id));

    utils::common::list_printable_items(all_acts, use_json)
}

fn matches_period(al: &PrintableActivityLog, period: &Period) -> bool {
    match period {
        cli::Period::Today => utils::date::is_today(al.log.starts_at),
        cli::Period::Yesterday => utils::date::is_yesterday(al.log.starts_at),
        cli::Period::ThisWeek => utils::date::is_this_week(al.log.starts_at),
        cli::Period::LastWeek => utils::date::is_last_week(al.log.starts_at),
        cli::Period::ThisMonth => utils::date::is_this_month(al.log.starts_at),
        cli::Period::LastMonth => utils::date::is_last_month(al.log.starts_at),
    }
}

fn list_activity_logs(conn: &mut Connection, args: &cli::ListActivityArgs) -> Result<()> {
    let mut act_logs: Vec<_> = activities::get_all(conn)?
        .iter()
        .flat_map(PrintableActivityLog::from_activity)
        .filter(|al| matches_period(al, &args.period))
        .collect();
    act_logs.sort_by_key(|al| al.log.starts_at);

    if args.no_grouping {
        return utils::common::list_printable_items(act_logs.to_vec(), args.use_json_format);
    }

    let act_logs_by_date = group_by_date(&act_logs);

    if args.use_json_format {
        let json = serde_json::to_string(&act_logs_by_date)?;
        println!("{json}");
        return Ok(());
    }

    for (date, act_logs) in act_logs_by_date.iter() {
        println!("{date}");
        utils::common::list_printable_items(act_logs.to_vec(), false)?;
    }
    Ok(())
}

fn group_by_date(
    activity_logs: &[PrintableActivityLog],
) -> BTreeMap<String, Vec<PrintableActivityLog>> {
    let mut groups: BTreeMap<_, Vec<_>> = BTreeMap::new();

    for act_log in activity_logs {
        let latest_dt = act_log.log.ends_at.unwrap_or(Local::now());
        let key = DateTimeRenderMode::DateOnly.render_date_time(latest_dt);
        groups.entry(key).or_default().push(act_log.clone());
    }

    groups
}
