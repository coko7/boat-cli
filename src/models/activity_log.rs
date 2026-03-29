use boat_lib::models::log::Log as DatabaseLog;
use serde::{Deserialize, Serialize};
use yansi::Paint;

use crate::{
    models::{RowPrintable, activity::SimpleActivity, log::PrintableLog},
    utils::{self, date::DateTimeRenderMode},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintableActivityLog {
    pub log: PrintableLog,
    pub activity: SimpleActivity,
}

impl PrintableActivityLog {
    pub fn from_activity_and_log(activity: &SimpleActivity, log: &DatabaseLog) -> Self {
        Self {
            log: PrintableLog::from_log(log),
            activity: activity.clone(),
        }
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
        let dt_render = DateTimeRenderMode::TimeOnly;
        let duration = self.log.duration_sec();

        vec![
            self.activity.id.to_string(),
            self.activity.name.clone(),
            self.activity.description.clone().unwrap_or_default(),
            self.activity.tags_str(),
            dt_render.render_date_time(self.log.starts_at),
            self.log
                .ends_at
                .map(|t| dt_render.render_date_time(t))
                .unwrap_or("-".to_string()),
            utils::date::pretty_format_duration(duration),
        ]
    }

    fn style_cell(&self, value: String) -> String {
        if self.log.ends_at.is_none() {
            Paint::green(&value).to_string()
        } else {
            value
        }
    }
}
