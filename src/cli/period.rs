use chrono::{Datelike, Local, Months, NaiveDate};
use log::debug;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::{fmt::Display, str::FromStr};

use crate::utils::date::DateTimeRenderMode;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
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

impl Display for PresetPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = (match self {
            PresetPeriod::Today => "today",
            PresetPeriod::Yesterday => "yesterday",
            PresetPeriod::ThisWeek => "this-week",
            PresetPeriod::LastWeek => "last-week",
            PresetPeriod::ThisMonth => "this-month",
            PresetPeriod::LastMonth => "last-month",
            PresetPeriod::AllTime => "all-time",
        })
        .to_string();
        write!(f, "{str}")
    }
}

impl PresetPeriod {
    pub fn display_pretty(&self) -> String {
        let now = Local::now();
        let last_month = now - Months::new(1);
        match self {
            PresetPeriod::Today => "Today".to_string(),
            PresetPeriod::Yesterday => "Yesterday".to_string(),
            PresetPeriod::ThisWeek => "This week".to_string(),
            PresetPeriod::LastWeek => "Last week".to_string(),
            PresetPeriod::ThisMonth => format!("{} {}", now.format("%B"), now.year()),
            PresetPeriod::LastMonth => format!("{} {}", last_month.format("%B"), last_month.year()),
            PresetPeriod::AllTime => "All time".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PeriodInput {
    Preset(PresetPeriod),
    Single(NaiveDate),
    Range { start: NaiveDate, end: NaiveDate },
}

impl Default for PeriodInput {
    fn default() -> Self {
        Self::Preset(PresetPeriod::AllTime)
    }
}

impl PeriodInput {
    const ERR_MSG: &'static str =
        "Provide either a range (YYYY-MM-DD..YYYY-MM-DD) or a single date (YYYY-MM-DD)";
}

impl FromStr for PeriodInput {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("period input from: {s}");
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
            "all-time" | "all" => return Ok(PeriodInput::Preset(PresetPeriod::AllTime)),
            _ => {}
        }
        // Match range
        if let Some((start, end)) = s.split_once("..") {
            let start = crate::utils::date::parse_date(start).map_err(|_| Self::ERR_MSG)?;
            let (end, inclusive) = match end.strip_prefix('=') {
                Some(substr) => (substr, true),
                None => (end, false),
            };
            let end = crate::utils::date::parse_date(end).map_err(|_| Self::ERR_MSG)?;
            if start > end {
                return Err("DateInput: start cannot be after end when using range".to_string());
            }
            return Ok(PeriodInput::Range { start, end });
        }
        // Single date
        let date = crate::utils::date::parse_date(&s).map_err(|_| Self::ERR_MSG)?;
        Ok(PeriodInput::Single(date))
    }
}

impl Serialize for PeriodInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PeriodInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        PeriodInput::from_str(&s).map_err(de::Error::custom)
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
            PeriodInput::Range { start, end } => {
                let start = DateTimeRenderMode::DateOnly.render_naive_date(start);
                let end = DateTimeRenderMode::DateOnly.render_naive_date(end);
                write!(f, "{start}..{end}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use toml;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Wrapper {
        val: PeriodInput,
    }
    #[test]
    fn periodinput_toml_serializes_and_deserializes_as_string_forms() {
        let cases = [
            ("today", PeriodInput::from_str("today").unwrap()),
            ("last-week", PeriodInput::from_str("last-week").unwrap()),
            ("2023-01-02", PeriodInput::from_str("2023-01-02").unwrap()),
            (
                "2023-01-02..2023-01-06",
                PeriodInput::from_str("2023-01-02..2023-01-06").unwrap(),
            ),
            (
                "2023-01-02..=2023-01-07",
                PeriodInput::from_str("2023-01-02..=2023-01-07").unwrap(),
            ),
        ];
        for (s, val) in cases {
            let wrap = Wrapper { val };
            let t = toml::to_string(&wrap).unwrap();
            assert_eq!(t.trim(), format!("val = \"{}\"", s));
            let round: Wrapper = toml::from_str(&format!("val = \"{}\"", s)).unwrap();
            assert_eq!(format!("{}", round.val), s);
        }
    }

    #[test]
    fn periodinput_toml_deserialize_invalid() {
        #[derive(Deserialize, Debug)]
        struct Wrapper {
            val: PeriodInput,
        }
        let err = toml::from_str::<Wrapper>("val = \"not-a-real-period\"");
        assert!(err.is_err());
    }
}
