use anyhow::{Context, Result};
use boat_lib::{models::activity::Activity as DatabaseActivity, repository::Id};
use chrono::{DateTime, Local};
use yansi::Paint;

use crate::utils::{self, date::DateTimeRenderMode};

pub fn format_ascii_ribbon(text: &str, tooltip_text: Option<&str>) -> String {
    let top_bot = format!("    *{}*\n", "-".repeat(text.len() + 2));
    format!(
        "{}{} {} {} {} {}\n{}",
        top_bot.blue(),
        "====|".blue(),
        text.cyan().bold(),
        "|=========(".blue(),
        tooltip_text.unwrap_or_default().italic(),
        ")====================>".blue(),
        top_bot.blue(),
    )
}

pub fn created_activity_msg(activity: &DatabaseActivity) -> String {
    format!(
        "{} new #{} \"{}\"",
        "created".cyan(),
        activity.id,
        activity.name
    )
}

pub fn paused_activity_msg(
    activity: &DatabaseActivity,
    stop_dt: DateTime<Local>,
) -> Result<String> {
    let stop_dt = DateTimeRenderMode::TimeOnly.render_date_time(stop_dt);
    Ok(format!(
        "{} #{} \"{}\" at {stop_dt}",
        "paused".bright_blue(),
        activity.id,
        activity.name
    ))
}

pub fn started_activity_msg(activity: &DatabaseActivity, start_dt: DateTime<Local>) -> String {
    let start_dt = DateTimeRenderMode::TimeOnly.render_date_time(start_dt);
    format!(
        "{} #{} \"{}\" at {start_dt}",
        "started".green(),
        activity.id,
        activity.name
    )
}

pub fn invalid_activity_name(activity_name: &str) -> String {
    format!(
        "the activity name cannot only contain digits: \"{}\"",
        activity_name
    )
    .red()
    .to_string()
}

pub fn activity_id_does_not_exist(id: Id) -> String {
    format!("#{id} does not exist").red().to_string()
}

pub fn invalid_activity_id_format(id_str: &str) -> String {
    format!("invalid activity ID format: \"{}\"", id_str)
        .red()
        .to_string()
}

pub fn deleted_activity_msg(activity: &DatabaseActivity) -> String {
    format!("{} #{} \"{}\"", "deleted".red(), activity.id, activity.name)
}

pub fn modified_activity_msg(activity: &DatabaseActivity) -> String {
    format!(
        "{} #{} \"{}\"",
        "modified".yellow(),
        activity.id,
        activity.name
    )
}

pub fn cancelled_activity_msg(activity: &DatabaseActivity) -> String {
    format!(
        "{} #{} \"{}\"",
        "cancelled".bright_white(),
        activity.id,
        activity.name
    )
}

pub fn current_activity_msg(activity: &DatabaseActivity) -> Result<String> {
    let desc = if let Some(desc) = &activity.description {
        format!(" ({desc})").to_string()
    } else {
        "".to_string()
    };

    let started_at = activity
        .logs
        .iter()
        .find(|l| l.ends_at.is_none())
        .context("there should be an ongoing log")?
        .starts_at
        .with_timezone(&Local);

    let duration_secs = (Local::now() - started_at).num_seconds();

    let tags_str = if !activity.tags.is_empty() {
        let tags_str = utils::common::tags_str(&activity.tags);
        format!(" [{tags_str}]")
    } else {
        "".to_string()
    };

    Ok(format!(
        "{} #{} \"{}\"{}{}: {} -> {} ({})",
        "current:".blue(),
        activity.id,
        activity.name,
        desc,
        tags_str,
        DateTimeRenderMode::TimeOnly.render_date_time(started_at),
        "Now",
        utils::date::pretty_format_duration(duration_secs, true),
    ))
}

pub fn no_current_act_msg() -> String {
    "no current activity".to_string()
}
