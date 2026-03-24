use crate::models::{SimpleActivity, SimpleLog};

pub fn convert_to_log_line(log: &SimpleLog, parent_activity: &SimpleActivity) -> Vec<String> {
    let duration = log.duration_sec();

    vec![
        parent_activity.id.to_string(),
        parent_activity.name.clone(),
        parent_activity.description.clone().unwrap_or_default(),
        parent_activity.tags_str(),
        log.starts_at.format("%H:%M").to_string(),
        log.ends_at
            .map(|t| t.format("%H:%M").to_string())
            .unwrap_or("-".to_string()),
        format_duration(duration),
    ]
}

pub fn format_duration(seconds: i64) -> String {
    if seconds < 60 {
        return format!("{seconds}s");
    }

    let hours = seconds / 3600;
    let seconds = seconds - hours * 3600;

    let minutes = seconds / 60;
    let seconds = seconds - minutes * 60;

    let _seconds = seconds % 60;

    let mut parts = vec![];
    if hours > 0 {
        parts.push(format!("{hours}h"))
    }

    parts.push(format!("{minutes}m"));
    parts.join(" ")
}
