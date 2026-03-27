use chrono::{DateTime, Datelike, Duration, Local, NaiveDate};

pub fn pretty_format_duration(mut seconds: i64) -> String {
    let hours = seconds / 3600;
    seconds %= 3600;

    let minutes = seconds / 60;
    seconds %= 60;

    let mut parts = Vec::new();

    if hours > 0 {
        parts.push(format!("{hours}h"));
    }

    if minutes > 0 || hours > 0 {
        parts.push(format!("{minutes}m"));
    }

    if parts.len() < 2 && (seconds > 0 || parts.is_empty()) {
        parts.push(format!("{seconds}s"));
    }

    parts.join(" ")
}

pub enum DateTimeRenderMode {
    TimeOnly,
    DateOnly,
    DateAndTime,
}

impl DateTimeRenderMode {
    pub fn render_date_time<Tz>(&self, dt: chrono::DateTime<Tz>) -> String
    where
        Tz: chrono::TimeZone,
        Tz::Offset: std::fmt::Display,
    {
        let fmt = match self {
            DateTimeRenderMode::TimeOnly => "%H:%M",
            DateTimeRenderMode::DateOnly => "%Y-%m-%d",
            DateTimeRenderMode::DateAndTime => "%Y-%m-%d %H:%M",
        };

        dt.format(fmt).to_string()
    }
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

    if this_week > 1 {
        other_week_year == this_week_year && other_week == this_week - 1
    } else {
        let last_year = this_week_year - 1;

        // Find previous ISO week by handling year boundary using the December 28 trick:
        // - ISO weeks start with Monday and end on Sunday.
        // - Each week's year is the Gregorian year in which the Thursday falls.
        // - The first week of the year, hence, always contains January 4.
        // - This means 7 days before is guaranteed to be the previous year.
        // - 7 days before Jan 4 gives us Dec 28.
        // Learn more: https://en.wikipedia.org/wiki/ISO_week_date
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

pub fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| format!("invalid date '{s}', expected format YYYY-MM-DD"))
}
