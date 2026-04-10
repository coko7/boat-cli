use anyhow::Result;
use boat_lib::repository::activities_repository as activities;
use chrono::{Local, NaiveDate};
use log::info;
use rusqlite::Connection;
use std::collections::BTreeMap;
use yansi::Paint;

use crate::{
    cli::{self},
    models::{activity_log::PrintableActivityLog, boat_data::BoatData},
    utils::{self, date::DateTimeRenderMode},
};

pub fn list_activities(conn: &mut Connection, args: &cli::ListActivityArgs) -> Result<()> {
    info!("getting all activities");
    let db_acts: Vec<_> = activities::get_all(conn)?;
    let boat_data = BoatData::create_filtered_data(db_acts, args.date_range, args.period);

    if args.show_summary {
        info!("showing summary");
        if !args.use_json_format {
            info!("using JSON format for summary");
            let date_msg = match args.date_range {
                Some(dt_range) => Some(dt_range.to_string()),
                None => args.period.map(|p| p.to_string()),
            };

            if let Some(date_msg) = date_msg {
                info!("using custom date msg for summary");
                println!("{} {}\n", "Summary:".underline(), date_msg.green());
            }
        }

        return list_activity_summaries(&boat_data, args.show_all, args.use_json_format);
    }

    list_activity_logs(&boat_data, args)
}

fn list_activity_summaries(boat_data: &BoatData, show_all: bool, use_json: bool) -> Result<()> {
    info!("listing activity summaries (show_all: {show_all})");
    let prt_acts = boat_data
        .get_printable_activities()
        .into_iter()
        .filter(|act| show_all || act.duration > 0)
        .collect();

    utils::common::list_printable_items(&prt_acts, use_json)?;

    if !use_json && !prt_acts.is_empty() {
        let total_sec: i64 = prt_acts.iter().map(|pa| pa.duration).sum();
        println!(
            "{} {}",
            "Total:".underline(),
            utils::date::pretty_format_duration(total_sec, false).green()
        );
    }

    Ok(())
}

fn list_activity_logs(boat_data: &BoatData, args: &cli::ListActivityArgs) -> Result<()> {
    info!("listing individual activity logs");
    let prt_logs = boat_data.get_printable_logs();

    if args.no_grouping {
        info!("activity logs will not be grouped by date");
        return utils::common::list_printable_items(&prt_logs, args.use_json_format);
    }

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
