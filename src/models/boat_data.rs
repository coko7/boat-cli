use boat_lib::{
    models::{activity::Activity as DatabaseActivity, log::Log as DatabaseLog},
    repository::Id,
};
use std::{cmp::Reverse, collections::HashMap};

use crate::{
    cli,
    models::{
        activity::{PrintableActivity, SimpleActivity},
        activity_log::PrintableActivityLog,
    },
    utils,
};

pub struct BoatData {
    pub activities: HashMap<Id, SimpleActivity>,
    pub logs: HashMap<Id, Vec<DatabaseLog>>,
}

impl BoatData {
    pub fn create_filtered_data(
        db_activities: Vec<DatabaseActivity>,
        args: &cli::ListActivityArgs,
    ) -> Self {
        let activities = db_activities
            .iter()
            .map(|db_act| (db_act.id, SimpleActivity::from_db_activity(db_act)))
            .collect();

        let logs = db_activities
            .into_iter()
            .map(|db_act| {
                let filtered_logs = db_act
                    .logs
                    .into_iter()
                    .filter(|log| utils::common::matches_date_filter(log, args))
                    .collect();
                (db_act.id, filtered_logs)
            })
            .collect();

        Self { activities, logs }
    }

    pub fn get_printable_activities(&self) -> Vec<PrintableActivity> {
        let mut prt_acts: Vec<_> = self
            .activities
            .values()
            .map(|act| {
                let related_logs = self.logs.get(&act.id).map(Vec::as_slice).unwrap_or(&[]);
                PrintableActivity::from_activity_and_logs(act, related_logs)
            })
            .collect();

        prt_acts.sort_by_key(|act| Reverse(act.duration));
        prt_acts
    }

    pub fn get_printable_logs(&self) -> Vec<PrintableActivityLog> {
        let mut prt_logs: Vec<_> = self
            .logs
            .values()
            .flatten()
            .map(|log| {
                let act = self.activities.get(&log.activity_id).unwrap();
                PrintableActivityLog::from_activity_and_log(act, log)
            })
            .collect();

        prt_logs.sort_by_key(|al| al.log.starts_at);
        prt_logs
    }
}
