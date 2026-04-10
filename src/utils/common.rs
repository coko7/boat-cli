use anyhow::Result;
use boat_lib::models::log::Log as DatabaseLog;
use chrono::{Local, NaiveDate};
use log::debug;
use serde::Serialize;
use std::collections::HashSet;

use crate::{
    cli::{DateInput, Period},
    models::{RowPrintable, TablePrintable},
    utils,
};

pub fn list_printable_items<T: RowPrintable + Serialize>(
    items: &Vec<T>,
    show_as_json: bool,
) -> Result<()> {
    if show_as_json {
        let json = serde_json::to_string(&items)?;
        println!("{json}");
        return Ok(());
    }

    if items.is_empty() {
        println!("no available data");
        return Ok(());
    }

    let table = items.to_printable_table();
    println!("{table}");
    Ok(())
}

pub fn tags_str(tags: &HashSet<String>) -> String {
    let mut tags: Vec<_> = tags.iter().map(String::as_str).collect();
    tags.sort_unstable();
    tags.join(",")
}

pub fn matches_date_filter(
    log: &DatabaseLog,
    date_input_opt: Option<DateInput>,
    period_opt: Option<Period>,
) -> bool {
    if let Some(date_range) = date_input_opt {
        matches_date_range(log, &date_range)
    } else if let Some(period) = period_opt {
        matches_period(log, &period)
    } else {
        debug!("no period / date filter provided, retaining activity log");
        true
    }
}

pub fn matches_period(log: &DatabaseLog, period: &Period) -> bool {
    debug!("checking if {log:?} matches the given period: {period:?}");

    match period {
        Period::Today => utils::date::is_today(log.starts_at),
        Period::Yesterday => utils::date::is_yesterday(log.starts_at),
        Period::ThisWeek => utils::date::is_this_week(log.starts_at),
        Period::LastWeek => utils::date::is_last_week(log.starts_at),
        Period::ThisMonth => utils::date::is_this_month(log.starts_at),
        Period::LastMonth => utils::date::is_last_month(log.starts_at),
    }
}

pub fn matches_date_range(log: &DatabaseLog, date_range: &DateInput) -> bool {
    debug!("checking if {log:?} matches the given date_range: {date_range:?}");

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

pub fn get_date_info_msg(today: NaiveDate, compare_to: NaiveDate) -> String {
    let diff_days = (today - compare_to).num_days();
    if diff_days == 0 {
        return "Today".to_string();
    }

    if diff_days == 1 {
        return "Yesterday".to_string();
    }

    if diff_days <= 7 {
        return format!("{diff_days} days ago");
    }

    let diff_weeks = diff_days / 7;
    if diff_weeks <= 4 {
        return format!(
            "{diff_weeks} week{} ago",
            if diff_weeks == 1 { "" } else { "s" }
        );
    }

    let diff_months = diff_weeks / 4;
    if diff_months == 1 {
        return "Last month".to_string();
    }

    format!("{diff_months} months ago")
}
