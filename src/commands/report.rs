use anyhow::Result;
use boat_lib::repository::activities_repository as activities;
use log::info;
use rusqlite::Connection;
use yansi::Paint;

use crate::{
    cli::{self, PeriodInput},
    config::Configuration,
    models::boat_data::BoatData,
    utils,
};

pub fn show_report(
    config: &Configuration,
    conn: &Connection,
    args: &cli::FilterActivitiesArgs,
) -> Result<()> {
    info!("getting all activities");
    let period = args.period.unwrap_or(
        config
            .period
            .unwrap_or(PeriodInput::Preset(cli::PresetPeriod::AllTime)),
    );
    info!("using period: {period}");

    let db_acts: Vec<_> = activities::get_all(conn)?;
    let boat_data = BoatData::create_filtered_data(db_acts, period);

    info!("listing individual activity logs");

    info!("showing summary");
    if !args.use_json_format {
        // let date_msg = match args.period {
        //     Some(dt_range) => Some(dt_range.to_string()),
        //     None => args.period.map(|p| p.to_string()),
        //     cli::PeriodInput::Preset(preset_period) => todo!(),
        //     cli::PeriodInput::Single(naive_date) => todo!(),
        //     cli::PeriodInput::Range(range) => range.to_string(),
        // };

        // if let Some(date_msg) = date_msg {
        //     info!("using custom date msg for summary");
        //     println!("{} {}\n", "Summary:".underline(), date_msg.green());
        // }
    }

    list_activity_summaries(&boat_data, args.show_all, args.use_json_format)
}

fn list_activity_summaries(boat_data: &BoatData, show_all: bool, use_json: bool) -> Result<()> {
    info!("listing activity summaries (show_all: {show_all})");
    let prt_acts = boat_data
        .get_printable_activities()
        .into_iter()
        .filter(|act| show_all || act.duration > 0)
        .collect();

    utils::common::list_printable_items(&prt_acts, use_json)?;

    if !use_json && !prt_acts.is_empty() {
        let total_sec: i64 = prt_acts.iter().map(|pa| pa.duration).sum();
        println!(
            "{} {}",
            "Total:".underline(),
            utils::date::pretty_format_duration(total_sec, false).green()
        );
    }

    Ok(())
}
