use boat_lib::models::activity::Activity as DatabaseActivity;
use boat_lib::repository::Id;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::models::RowPrintable;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintableActivity {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub ongoing: bool,
    pub tags: HashSet<String>,
}

impl PrintableActivity {
    pub fn from_activity(activity: &DatabaseActivity) -> Self {
        Self {
            id: activity.id,
            name: activity.name.clone(),
            description: activity.description.clone(),
            ongoing: activity.logs.iter().any(|l| l.ends_at.is_none()),
            tags: activity.tags.clone(),
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

impl RowPrintable for PrintableActivity {
    fn row_spec() -> String {
        "{:>}  {:<}  {:<}  {:<}  {:^}".to_string()
    }

    fn header_names() -> Vec<String> {
        ["ID", "Name", "Description", "Tags", "Ongoing"]
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
            (if self.ongoing { "*" } else { "" }).to_string(),
        ]
    }
}
