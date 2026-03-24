use chrono::{DateTime, Datelike, Duration, Local, NaiveDate};

use crate::models::{activity_with_log::PrintableActivityWithLogs, log::PrintableLog};

pub fn convert_to_log_line(
    log: &PrintableLog,
    parent_activity: &PrintableActivityWithLogs,
) -> Vec<String> {
    let duration = log.duration_sec();

    vec![
        parent_activity.id.to_string(),
        parent_activity.name.clone(),
        parent_activity.description.clone().unwrap_or_default(),
        parent_activity.tags_str(),
        log.starts_at.format("%H:%M").to_string(),
        log.ends_at
            .map(|t| t.format("%H:%M").to_string())
            .unwrap_or("-".to_string()),
        format_duration(duration),
    ]
}

pub fn format_duration(seconds: i64) -> String {
    if seconds < 60 {
        return format!("{seconds}s");
    }

    let hours = seconds / 3600;
    let seconds = seconds - hours * 3600;

    let minutes = seconds / 60;
    let seconds = seconds - minutes * 60;

    let _seconds = seconds % 60;

    let mut parts = vec![];
    if hours > 0 {
        parts.push(format!("{hours}h"))
    }

    parts.push(format!("{minutes}m"));
    parts.join(" ")
}

pub fn is_today<Tz>(dt: DateTime<Tz>) -> bool
where
    Tz: chrono::TimeZone,
{
    let today = Local::now().date_naive();
    let other_dt = dt.with_timezone(&Local);
    other_dt.date_naive() == today
}

pub fn is_yesterday<Tz>(dt: DateTime<Tz>) -> bool
where
    Tz: chrono::TimeZone,
{
    let today = Local::now().date_naive();
    let yesterday = today - Duration::days(1);
    let other_dt = dt.with_timezone(&Local);
    other_dt.date_naive() == yesterday
}

pub fn is_this_week<Tz>(dt: DateTime<Tz>) -> bool
where
    Tz: chrono::TimeZone,
{
    let today = Local::now().date_naive();
    let this_week_iso = today.iso_week();

    let other_dt = dt.with_timezone(&Local);
    let other_week_iso = other_dt.date_naive().iso_week();

    this_week_iso.year() == other_week_iso.year() && this_week_iso.week() == other_week_iso.week()
}

pub fn is_last_week<Tz>(dt: DateTime<Tz>) -> bool
where
    Tz: chrono::TimeZone,
{
    let today = Local::now().date_naive();
    let other_dt = dt.with_timezone(&Local);

    let this_week_iso = today.iso_week();
    let this_week_year = this_week_iso.year();
    let this_week = this_week_iso.week();

    let other_week_iso = other_dt.date_naive().iso_week();
    let other_week_year = other_week_iso.year();
    let other_week = other_week_iso.week();

    // Previous ISO week, handling year boundary via 28 Dec trick
    if this_week > 1 {
        other_week_year == this_week_year && other_week == this_week - 1
    } else {
        let last_year = this_week_year - 1;
        let last_week_of_last_year = NaiveDate::from_ymd_opt(last_year, 12, 28)
            .unwrap()
            .iso_week()
            .week();
        other_week_year == last_year && other_week == last_week_of_last_year
    }
}

pub fn is_this_month<Tz>(dt: DateTime<Tz>) -> bool
where
    Tz: chrono::TimeZone,
{
    let today = Local::now().date_naive();
    let other_dt = dt.with_timezone(&Local);

    let this_year = today.year();
    let this_month = today.month();

    let other = other_dt.date_naive();
    other.year() == this_year && other.month() == this_month
}

pub fn is_last_month<Tz>(dt: DateTime<Tz>) -> bool
where
    Tz: chrono::TimeZone,
{
    let today = Local::now().date_naive();
    let other_dt = dt.with_timezone(&Local);

    let this_year = today.year();
    let this_month = today.month();

    let (last_month_year, last_month) = if this_month > 1 {
        (this_year, this_month - 1)
    } else {
        (this_year - 1, 12)
    };

    let other = other_dt.date_naive();
    other.year() == last_month_year && other.month() == last_month
}
