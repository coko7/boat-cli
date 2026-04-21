use anyhow::Result;
use boat_lib::repository::activities_repository as activities;
use chrono::{Local, NaiveDate};
use log::info;
use rusqlite::Connection;
use std::collections::BTreeMap;

use crate::{
    cli::{self, PeriodInput},
    config::Configuration,
    models::{activity_log::PrintableActivityLog, boat_data::BoatData},
    utils::{self, date::DateTimeRenderMode},
};

pub fn list_activity_logs(
    config: &Configuration,
    conn: &Connection,
    args: &cli::FilterActivitiesArgs,
) -> Result<()> {
    info!("getting all activities");
    let period = args
        .period
        .or(config.commands.list.period)
        .or(config.period)
        .unwrap_or(PeriodInput::Preset(cli::PresetPeriod::AllTime));
    info!("using period: {period}");

    let db_acts: Vec<_> = activities::get_all(conn)?;
    let boat_data = BoatData::create_filtered_data(db_acts, period);

    info!("listing individual activity logs");
    let prt_logs = boat_data.get_printable_logs();

    // if args.no_grouping {
    //     info!("activity logs will not be grouped by date");
    //     return utils::common::list_printable_items(&prt_logs, args.use_json_format);
    // }

    let act_logs_by_date = group_by_date(&prt_logs);

    if args.use_json_format {
        let json = serde_json::to_string(&act_logs_by_date)?;
        println!("{json}");
        return Ok(());
    }

    if act_logs_by_date.is_empty() {
        println!("no available data");
        return Ok(());
    }

    info!("displaying activity logs grouped by date");
    for (date, act_logs) in act_logs_by_date.iter() {
        let dt = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
        let diff_msg = utils::common::get_date_info_msg(Local::now().date_naive(), dt);
        let ribbon = utils::display::format_ascii_ribbon(date, Some(&diff_msg));

        println!("{ribbon}");
        utils::common::list_printable_items(act_logs, false)?;
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
