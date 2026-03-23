use boat_lib::{
    models::{activity::Activity, log::Log},
    repository::Id,
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{self, Display},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleActivity {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub tags: HashSet<String>,
    pub logs: Vec<SimpleLog>,
    pub ongoing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleLog {
    pub starts_at: DateTime<Local>,
    pub ends_at: Option<DateTime<Local>>,
}

impl SimpleLog {
    pub fn from_log(log: &Log) -> Self {
        Self {
            starts_at: log.starts_at.with_timezone(&Local),
            ends_at: log.ends_at.map(|t| t.with_timezone(&Local)),
        }
    }
}

impl SimpleActivity {
    pub fn from_activity(activity: &Activity) -> Self {
        Self {
            id: activity.id,
            name: activity.name.clone(),
            description: activity.description.clone(),
            tags: activity.tags.clone(),
            logs: activity.logs.iter().map(SimpleLog::from_log).collect(),
            ongoing: activity.logs.iter().any(|l| l.ends_at.is_none()),
        }
    }
}

impl Display for SimpleActivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = self.description.as_deref().unwrap_or_default();
        let tags = self
            .tags
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(",");
        writeln!(f, "{}\t{}\t{desc}\t{tags}", self.id, self.name)?;

        for log in self.logs.iter() {
            let starts_at = log.starts_at.format("%H:%M:%S");
            let ends_at = log
                .ends_at
                .as_ref()
                .map(|t| t.format("%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "ongoing".to_string());
            writeln!(f, "\t - {} ---> {}", starts_at, ends_at)?;
        }
        Ok(())
    }
}
