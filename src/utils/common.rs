use anyhow::Result;
use boat_lib::models::log::Log as DatabaseLog;
use chrono::{Local, NaiveDate};
use dialoguer::Confirm;
use log::debug;
use serde::Serialize;
use std::collections::HashSet;
use yansi::Paint;

use crate::{
    cli::{PeriodInput, PresetPeriod},
    models::{RowPrintable, TablePrintable},
    utils,
};

pub fn resolve_tri_state(a: bool, b: bool, c: bool) -> bool {
    match (a, b) {
        (true, false) => true,
        (false, true) => false,
        _ => c, // neither specified → fallback
    }
}

pub fn prompt_for_confirmation(msg: &str, default_value: bool) -> Result<bool> {
    let proceed = Confirm::new()
        .with_prompt(Paint::yellow(&msg).to_string())
        .default(default_value)
        .interact()?;

    Ok(proceed)
}

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

pub fn matches_period_filter(log: &DatabaseLog, period_input: &PeriodInput) -> bool {
    match period_input {
        PeriodInput::Preset(preset_period) => matches_period(log, preset_period),
        PeriodInput::Single(date) => matches_date(log, date),
        PeriodInput::Range { start, end } => matches_date_range(log, start, end),
    }
}

pub fn matches_period(log: &DatabaseLog, period: &PresetPeriod) -> bool {
    debug!("checking if {log:?} matches the given period: {period:?}");

    match period {
        PresetPeriod::Today => utils::date::is_today(log.starts_at),
        PresetPeriod::Yesterday => utils::date::is_yesterday(log.starts_at),
        PresetPeriod::ThisWeek => utils::date::is_this_week(log.starts_at),
        PresetPeriod::LastWeek => utils::date::is_last_week(log.starts_at),
        PresetPeriod::ThisMonth => utils::date::is_this_month(log.starts_at),
        PresetPeriod::LastMonth => utils::date::is_last_month(log.starts_at),
        PresetPeriod::AllTime => true,
    }
}

pub fn matches_date(log: &DatabaseLog, date: &NaiveDate) -> bool {
    log.starts_at.date_naive() == *date
        && log.ends_at.unwrap_or(Local::now().into()).date_naive() == *date
}

pub fn matches_date_range(
    log: &DatabaseLog,
    range_start: &NaiveDate,
    range_end: &NaiveDate,
) -> bool {
    debug!("checking if {log:?} matches the given date_range: {range_start:?}, {range_end:?}");

    let log_start = log.starts_at.date_naive();
    let log_end = log.ends_at.unwrap_or(Local::now().into()).date_naive();
    log_start >= *range_start && log_end <= *range_end
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
