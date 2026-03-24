use boat_lib::models::activity::Activity as DatabaseActivity;
use boat_lib::repository::Id;
use serde::{Deserialize, Serialize};

use crate::models::RowPrintable;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintableActivity {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub ongoing: bool,
}

impl PrintableActivity {
    pub fn from_activity(activity: &DatabaseActivity) -> Self {
        Self {
            id: activity.id,
            name: activity.name.clone(),
            description: activity.description.clone(),
            ongoing: activity.logs.iter().any(|l| l.ends_at.is_none()),
        }
    }
}

impl RowPrintable for PrintableActivity {
    fn row_spec() -> String {
        "{:>}  {:<}  {:<}  {:^}".to_string()
    }

    fn header_names() -> Vec<String> {
        vec!["ID", "Name", "Description", "Ongoing"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn row_values(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.description.clone().unwrap_or_default(),
            (if self.ongoing { "*" } else { "" }).to_string(),
        ]
    }
}
