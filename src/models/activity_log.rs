use boat_lib::models::activity::Activity as DatabaseActivity;
use serde::{Deserialize, Serialize};

use crate::{
    models::{RowPrintable, activity::PrintableActivity, log::PrintableLog},
    utils,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintableActivityLog {
    pub log: PrintableLog,
    pub activity: PrintableActivity,
}

impl PrintableActivityLog {
    pub fn from_activity(activity: &DatabaseActivity) -> Vec<Self> {
        activity
            .logs
            .iter()
            .map(|l| PrintableActivityLog {
                log: PrintableLog::from_log(l),
                activity: PrintableActivity::from_activity(activity),
            })
            .collect()
    }
}

impl RowPrintable for PrintableActivityLog {
    fn row_spec() -> String {
        "{:>}  {:<}  {:<}  {:<}  {:<}  {:<}  {:<}".to_string()
    }

    fn header_names() -> Vec<String> {
        [
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
        let duration = self.log.duration_sec();

        vec![
            self.activity.id.to_string(),
            self.activity.name.clone(),
            self.activity.description.clone().unwrap_or_default(),
            self.activity.tags_str(),
            self.log.starts_at.format("%H:%M").to_string(),
            self.log
                .ends_at
                .map(|t| t.format("%H:%M").to_string())
                .unwrap_or("-".to_string()),
            utils::date::pretty_format_duration(duration),
        ]
    }
}
