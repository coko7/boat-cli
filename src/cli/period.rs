use chrono::{Datelike, Local, Months, NaiveDate};
use std::str::FromStr;

use crate::utils::{self, date::DateTimeRenderMode};

#[derive(Debug, Clone, Copy)]
pub enum PeriodInput {
    Preset(PresetPeriod),
    Single(NaiveDate),
    Range {
        start: NaiveDate,
        end: NaiveDate,
        inclusive: bool,
    },
}

impl PeriodInput {
    const ERR_MSG: &'static str =
        "Provide either a range (YYYY-MM-DD..YYYY-MM-DD) or a single date (YYYY-MM-DD)";
}

impl FromStr for PeriodInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        // Handle presets
        match s.as_str() {
            "today" | "td" | "tod" => return Ok(PeriodInput::Preset(PresetPeriod::Today)),
            "yesterday" | "yd" | "ytd" => return Ok(PeriodInput::Preset(PresetPeriod::Yesterday)),
            "this-week" | "tw" | "twk" | "wk" => {
                return Ok(PeriodInput::Preset(PresetPeriod::ThisWeek));
            }
            "last-week" | "lw" | "lwk" | "yesterweek" | "yw" | "ywk" => {
                return Ok(PeriodInput::Preset(PresetPeriod::LastWeek));
            }
            "this-month" | "tm" | "tmo" | "mo" => {
                return Ok(PeriodInput::Preset(PresetPeriod::ThisMonth));
            }
            "last-month" | "lm" | "lmo" | "yestermonth" | "ym" | "ymo" => {
                return Ok(PeriodInput::Preset(PresetPeriod::LastMonth));
            }
            _ => {}
        }

        // Match range
        if let Some((start, end)) = s.split_once("..") {
            let start = utils::date::parse_date(start).map_err(|_| Self::ERR_MSG)?;
            let (end, inclusive) = match end.strip_prefix('=') {
                Some(substr) => (substr, true),
                None => (end, false),
            };
            let end = utils::date::parse_date(end).map_err(|_| Self::ERR_MSG)?;

            if start > end {
                return Err("DateInput: start cannot be after end when using range".to_string());
            }

            return Ok(PeriodInput::Range {
                start,
                end,
                inclusive,
            });
        }

        // Single date
        let date = utils::date::parse_date(&s).map_err(|_| Self::ERR_MSG)?;
        Ok(PeriodInput::Single(date))
    }
}

impl std::fmt::Display for PeriodInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeriodInput::Preset(preset_period) => write!(f, "{}", preset_period),
            PeriodInput::Single(naive_date) => {
                let dt = DateTimeRenderMode::DateOnly.render_naive_date(naive_date);
                write!(f, "{dt}")
            }
            PeriodInput::Range {
                start,
                end,
                inclusive,
            } => {
                let start = DateTimeRenderMode::DateOnly.render_naive_date(start);
                let end = DateTimeRenderMode::DateOnly.render_naive_date(end);
                let inclusion_msg = (if *inclusive { "included" } else { "excluded" }).to_string();
                write!(f, "{start} to {end} ({inclusion_msg})")
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum PresetPeriod {
    Today,
    Yesterday,
    ThisWeek,
    LastWeek,
    ThisMonth,
    LastMonth,
    #[default]
    AllTime,
}

impl std::fmt::Display for PresetPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let now = Local::now();
        let last_month = now - Months::new(1);

        let period = match self {
            PresetPeriod::Today => "Today".to_string(),
            PresetPeriod::Yesterday => "Yesterday".to_string(),
            PresetPeriod::ThisWeek => "This week".to_string(),
            PresetPeriod::LastWeek => "Last week".to_string(),
            PresetPeriod::ThisMonth => format!("{} {}", now.format("%B"), now.year()),
            PresetPeriod::LastMonth => format!("{} {}", last_month.format("%B"), last_month.year()),
            PresetPeriod::AllTime => "All time".to_string(),
        };
        write!(f, "{period}")
    }
}
