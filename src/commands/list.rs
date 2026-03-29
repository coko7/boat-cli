use anyhow::Result;
use boat_lib::{
    models::activity::Activity as DatabaseActivity,
    models::log::Log as DatabaseLog,
    repository::{Id, activities_repository as activities},
};
use chrono::Local;
use rusqlite::Connection;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap},
};

use crate::{
    cli::{self, DateInput, Period},
    models::{
        activity::{PrintableActivity, SimpleActivity},
        activity_log::PrintableActivityLog,
    },
    utils::{self, date::DateTimeRenderMode},
};

struct BoatData {
    activities: HashMap<Id, SimpleActivity>,
    logs: HashMap<Id, Vec<DatabaseLog>>,
}

impl BoatData {
    fn create_filtered_data(
        db_activities: Vec<DatabaseActivity>,
        args: &cli::ListActivityArgs,
    ) -> Self {
        let activities = db_activities
            .iter()
            .map(|db_act| (db_act.id, SimpleActivity::from_db_activity(db_act)))
            .collect();

        let logs = db_activities
            .into_iter()
            .map(|db_act| {
                let filtered_logs = db_act
                    .logs
                    .into_iter()
                    .filter(|log| matches_period(log, &args.period))
                    .filter(|log| try_match_date_range_or_ignore(log, &args.date_range))
                    .collect();
                (db_act.id, filtered_logs)
            })
            .collect();

        Self { activities, logs }
    }

    fn get_printable_activities(&self) -> Vec<PrintableActivity> {
        let mut prt_acts: Vec<_> = self
            .activities
            .values()
            .map(|act| {
                let related_logs = self.logs.get(&act.id).map(Vec::as_slice).unwrap_or(&[]);
                PrintableActivity::from_activity_and_logs(act, related_logs)
            })
            .collect();

        prt_acts.sort_by_key(|a| Reverse(a.id));
        prt_acts
    }

    fn get_printable_logs(&self) -> Vec<PrintableActivityLog> {
        let mut prt_logs: Vec<_> = self
            .logs
            .values()
            .flatten()
            .map(|log| {
                let act = self.activities.get(&log.activity_id).unwrap();
                PrintableActivityLog::from_activity_and_log(act, log)
            })
            .collect();

        prt_logs.sort_by_key(|al| al.log.starts_at);
        prt_logs
    }
}

pub fn list_activities(conn: &mut Connection, args: &cli::ListActivityArgs) -> Result<()> {
    let db_acts: Vec<_> = activities::get_all(conn)?;
    let boat_data = BoatData::create_filtered_data(db_acts, args);

    if args.show_summary {
        return list_activity_summaries(&boat_data, args.use_json_format);
    }

    list_activity_logs(&boat_data, args)
}

fn matches_period(log: &DatabaseLog, period: &Period) -> bool {
    match period {
        cli::Period::Today => utils::date::is_today(log.starts_at),
        cli::Period::Yesterday => utils::date::is_yesterday(log.starts_at),
        cli::Period::ThisWeek => utils::date::is_this_week(log.starts_at),
        cli::Period::LastWeek => utils::date::is_last_week(log.starts_at),
        cli::Period::ThisMonth => utils::date::is_this_month(log.starts_at),
        cli::Period::LastMonth => utils::date::is_last_month(log.starts_at),
    }
}

fn try_match_date_range_or_ignore(log: &DatabaseLog, date_range_opt: &Option<DateInput>) -> bool {
    let Some(date_range) = date_range_opt else {
        return true;
    };

    matches_date_range(log, date_range)
}

fn matches_date_range(log: &DatabaseLog, date_range: &DateInput) -> bool {
    let log_start = log.starts_at.date_naive();
    let log_end = log.ends_at.unwrap_or(Local::now().into()).date_naive();

    match date_range {
        DateInput::Single(naive_date) => log_start == *naive_date && log_end == *naive_date,
        DateInput::Range {
            start,
            end,
            inclusive,
        } => {
            let log_ends_before_range_end = if *inclusive {
                log_end <= *end
            } else {
                log_end < *end
            };

            log_start >= *start && log_ends_before_range_end
        }
    }
}

fn list_activity_summaries(boat_data: &BoatData, use_json: bool) -> Result<()> {
    let prt_acts = boat_data
        .get_printable_activities()
        .into_iter()
        .filter(|pa| pa.duration > 0)
        .collect();
    utils::common::list_printable_items(prt_acts, use_json)
}

fn list_activity_logs(boat_data: &BoatData, args: &cli::ListActivityArgs) -> Result<()> {
    let prt_logs = boat_data.get_printable_logs();

    if args.no_grouping {
        return utils::common::list_printable_items(prt_logs.to_vec(), args.use_json_format);
    }

    let act_logs_by_date = group_by_date(&prt_logs);

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
