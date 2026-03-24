use boat_lib::{models::activity::Activity as DatabaseActivity, repository::Id};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::models::{RowPrintable, log::PrintableLog};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintableActivityWithLogs {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub ongoing: bool,
    pub tags: HashSet<String>,
    pub logs: Vec<PrintableLog>,
}

impl PrintableActivityWithLogs {
    pub fn from_activity(activity: &DatabaseActivity) -> Self {
        Self {
            id: activity.id,
            name: activity.name.clone(),
            description: activity.description.clone(),
            tags: activity.tags.clone(),
            logs: activity.logs.iter().map(PrintableLog::from_log).collect(),
            ongoing: activity.logs.iter().any(|l| l.ends_at.is_none()),
        }
    }

    pub fn tags_str(&self) -> String {
        self.tags
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl RowPrintable for PrintableActivityWithLogs {
    fn row_spec() -> String {
        "{:>}  {:<}  {:<}  {:<}  {:<}  {:<}  {:<}".to_string()
    }

    fn header_names() -> Vec<String> {
        vec![
            "ID",
            "Name",
            "Description",
            "Tags",
            "Start",
            "End",
            "Duration",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn row_values(&self) -> Vec<String> {
        todo!()
    }
}
