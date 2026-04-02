use boat_lib::models::{activity::Activity as DatabaseActivity, log::Log as DatabaseLog};
use boat_lib::repository::Id;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::models::RowPrintable;
use crate::utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleActivity {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub tags: HashSet<String>,
}

impl SimpleActivity {
    pub fn tags_str(&self) -> String {
        self.tags
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl SimpleActivity {
    pub fn from_db_activity(activity: &DatabaseActivity) -> Self {
        Self {
            id: activity.id,
            name: activity.name.clone(),
            description: activity.description.clone(),
            tags: activity.tags.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintableActivity {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub duration: i64,
    pub ongoing: bool,
    pub tags: HashSet<String>,
}

impl PrintableActivity {
    pub fn from_activity_and_logs(activity: &SimpleActivity, logs: &[DatabaseLog]) -> Self {
        let total_duration_sec: i64 = logs.iter().map(Self::count_log_duration_sec).sum();

        Self {
            id: activity.id,
            name: activity.name.clone(),
            description: activity.description.clone(),
            ongoing: logs.iter().any(|l| l.ends_at.is_none()),
            duration: total_duration_sec,
            tags: activity.tags.clone(),
        }
    }

    fn count_log_duration_sec(log: &DatabaseLog) -> i64 {
        let start = log.starts_at;
        let end = log.ends_at.unwrap_or(Utc::now());
        (end - start).num_seconds()
    }

    pub fn tags_str(&self) -> String {
        self.tags
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_str_renders_comma_separated() {
        let mut tags = HashSet::new();
        tags.insert("foo".to_owned());
        tags.insert("bar".to_owned());

        let act = PrintableActivity {
            id: 42,
            name: "n".to_owned(),
            description: None,
            duration: 0,
            ongoing: false,
            tags,
        };
        let tags_str = act.tags_str();

        assert!(tags_str.contains("foo"));
        assert!(tags_str.contains("bar"));
        assert!(tags_str.find(',').is_some());
    }
}

impl RowPrintable for PrintableActivity {
    fn row_spec() -> String {
        "{:>}  {:<}  {:<}  {:<}  {:<}".to_string()
    }

    fn header_names() -> Vec<String> {
        ["ID", "Name", "Description", "Tags", "Duration"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn row_values(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.description.clone().unwrap_or_default(),
            self.tags_str(),
            utils::date::pretty_format_duration(self.duration, false),
        ]
    }
}
