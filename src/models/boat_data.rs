use boat_lib::{
    models::{activity::Activity as DatabaseActivity, log::Log as DatabaseLog},
    repository::Id,
};
use log::info;
use std::{cmp::Reverse, collections::HashMap};

use crate::{
    cli,
    models::{
        activity::{PrintableActivity, SimpleActivity},
        activity_log::PrintableActivityLog,
    },
    utils::{self, date::DateTimeRenderMode},
};

pub struct BoatData {
    pub activities: HashMap<Id, SimpleActivity>,
    pub logs: HashMap<Id, Vec<DatabaseLog>>,
}

impl BoatData {
    pub fn create_filtered_data(
        db_activities: Vec<DatabaseActivity>,
        period_input: cli::PeriodInput,
    ) -> Self {
        info!("creating filtered boat data");
        let activities = db_activities
            .iter()
            .map(|db_act| (db_act.id, SimpleActivity::from_db_activity(db_act)))
            .collect();

        info!("filtering activity logs based on date filters (period / dates)");
        let logs = db_activities
            .into_iter()
            .map(|db_act| {
                let filtered_logs = db_act
                    .logs
                    .into_iter()
                    .filter(|log| utils::common::matches_period_filter(log, &period_input))
                    .collect();
                (db_act.id, filtered_logs)
            })
            .collect();

        Self { activities, logs }
    }

    pub fn get_printable_activities(&self) -> Vec<PrintableActivity> {
        info!("retrieving printable activities");
        let mut prt_acts: Vec<_> = self
            .activities
            .values()
            .map(|act| {
                let related_logs = self.logs.get(&act.id).map(Vec::as_slice).unwrap_or(&[]);
                PrintableActivity::from_activity_and_logs(act, related_logs)
            })
            .collect();

        info!("sorting printable activities by duration DESC");
        prt_acts.sort_by_key(|act| Reverse(act.duration));
        prt_acts
    }

    pub fn get_printable_logs(&self) -> Vec<PrintableActivityLog> {
        info!("retrieving printable logs");
        let mut prt_logs: Vec<_> = self
            .logs
            .values()
            .flatten()
            .map(|log| {
                let act = self.activities.get(&log.activity_id).unwrap();
                PrintableActivityLog::from_activity_and_log(act, log)
            })
            .collect();

        info!("sorting printable logs by start dt ASC");
        prt_logs.sort_by_key(|al| al.log.starts_at);
        prt_logs
    }

    pub fn to_csv_str(&self, include_instructions: bool) -> String {
        let mut csv_data = String::new();

        if include_instructions {
            csv_data.push_str("# This is a CSV export of your activities and logs.\n");
            csv_data
                .push_str("# Lines starting with '#' are comments and should not be modified.\n");
            csv_data.push_str(
            "# Activity definitions are included for reference but are not meant to be edited.\n",
        );
            csv_data.push_str("# You may only edit activity logs here.\n#\n");
        }

        csv_data.push_str("# Activity definitions:\n");
        csv_data.push_str("# | ID | Name | Description | Tags |\n");
        csv_data.push_str("# | -- | ---- | ----------- | ---- |\n");

        let mut activities = self.activities.values().collect::<Vec<_>>();
        activities.sort_by_key(|act| act.id);

        let mut act_logs = vec![];
        for act in activities {
            let related_logs = self.logs.get(&act.id).map(Vec::as_slice).unwrap_or(&[]);
            if related_logs.is_empty() {
                continue;
            }

            let act_csv = format!(
                "# | {} | {} | {} | {} |\n",
                act.id,
                act.name,
                act.description.clone().unwrap_or("".to_string()),
                utils::common::tags_str(&act.tags)
            );
            csv_data.push_str(&act_csv);

            for log in related_logs {
                let local_starts_at = log.starts_at.with_timezone(&chrono::Local);
                let local_ends_at = log.ends_at.map(|dt| dt.with_timezone(&chrono::Local));
                act_logs.push((act.id, log.id, local_starts_at, local_ends_at));
            }
        }

        csv_data.push_str("\n\n");

        if include_instructions {
            csv_data.push_str(
                "# Below are your activity logs. You can edit the start and end times here.\n",
            );
            csv_data.push_str(
            "# If you want to mark the latest activity as ongoing, simply remove the end time (leave it blank) for that log.\n",
        );
            csv_data.push_str(
                "# Please keep the activity_id and log_id unchanged to avoid breaking the data.\n#\n",
            );
        }

        csv_data.push_str("# Logs (activity_id,log_id,starts_at,ends_at):\n");
        csv_data.push_str("# ===== EDIT DATA BELOW =====\n");

        act_logs.sort_by_key(|(_, _, starts_at, _)| *starts_at);
        for (act_id, log_id, starts_at, ends_at) in act_logs {
            let ends_at = match ends_at {
                Some(dt) => DateTimeRenderMode::DateAndTime.render_date_time(dt),
                None => "".to_string(),
            };

            let log_csv = format!(
                "{},{},{},{}\n",
                act_id,
                log_id,
                DateTimeRenderMode::DateAndTime.render_date_time(starts_at),
                ends_at
            );
            csv_data.push_str(&log_csv);
        }

        csv_data
    }
}
