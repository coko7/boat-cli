use boat_lib::repository::Id;
use chrono::Datelike;
use chrono::Local;
use chrono::Months;
use chrono::NaiveDate;
use clap::ColorChoice;
use clap::Parser;
use clap::{ArgAction, Args, Subcommand, ValueEnum};
use std::str::FromStr;

use crate::utils;
use crate::utils::date::DateTimeRenderMode;

#[derive(Parser, Debug)]
#[command(
    name = "boat",
    version,
    author = "Made by @coko7 <contact@coko7.fr>",
    color = ColorChoice::Auto,
    about = "Basic Opinionated Activity Tracker",
    help_template = "{name} {version}\n\n{about}\n\n{usage-heading}\n{usage}\n\n{all-args}\n\n{author}"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "kebab-case")]
pub enum Commands {
    /// Create a new activity
    #[command(alias = "n", alias = "create")]
    New(CreateActivityArgs),

    /// Start/resume an activity
    #[command(
        alias = "s",
        alias = "st",
        alias = "sail",
        alias = "continue",
        alias = "resume"
    )]
    Start(SelectActivityArgs),

    /// Cancel the current activity
    #[command(alias = "c", alias = "can")]
    Cancel,

    /// Pause/stop the current activity
    #[command(alias = "p", alias = "stop")]
    Pause,

    /// Modify an activity
    #[command(alias = "m", alias = "mod")]
    Modify(ModifyActivityArgs),

    /// Edit activity logs as text in an external editor
    #[command(alias = "e", alias = "ed")]
    Edit(EditLogsArgs),

    /// Delete an activity
    #[command(
        alias = "d",
        alias = "del",
        alias = "rm",
        alias = "rem",
        alias = "remove"
    )]
    Delete(SelectActivityArgs),

    /// Get the current activity
    #[command(alias = "g")]
    Get(PrintActivityArgs),

    /// List activities
    #[command(alias = "l", alias = "ls")]
    List(ListActivityArgs),

    // /// Query boat objects
    // #[command(alias = "q")]
    // Query {
    //     #[command(subcommand)]
    //     command: QuerySubcommand,
    // },

    // This is ONLY way I could find to use the 'h' short alias for help.
    #[command(alias = "h", hide = true)]
    HelpExtension,
    // Verify the activity data and report eventual issues
    // #[command(alias = "v", alias = "verif")]
    // Verify {},

    // Query the different objects: activities, logs, tags
    // #[command(alias = "q")]
    // Query {},

    // Display a report with statistics about activities
    // #[command(alias = "r", alias = "rep")]
    // Report {},
    // ^^^ or maybe export 'x' ???
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum QuerySubcommand {
    /// Manage logs
    #[command(name = "logs", alias = "l", alias = "log")]
    Logs(ListActivityArgs),

    /// Manage activities
    #[command(
        name = "acts",
        alias = "act",
        alias = "a",
        alias = "activity",
        alias = "activities"
    )]
    Activities(ListArgs),

    /// Manage tags
    #[command(name = "tags", alias = "t", alias = "tag")]
    Tags(ListArgs),
}

#[derive(Debug, Clone, Copy)]
pub enum DateInput {
    Single(NaiveDate),
    Range {
        start: NaiveDate,
        end: NaiveDate,
        inclusive: bool,
    },
}

impl DateInput {
    const ERR_MSG: &'static str =
        "Provide either a range (YYYY-MM-DD..YYYY-MM-DD) or a single date (YYYY-MM-DD)";
}

impl std::fmt::Display for DateInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateInput::Single(naive_date) => {
                let dt = DateTimeRenderMode::DateOnly.render_naive_date(naive_date);
                write!(f, "{dt}")
            }
            DateInput::Range {
                start,
                end,
                inclusive,
            } => {
                let start = DateTimeRenderMode::DateOnly.render_naive_date(start);
                let end = DateTimeRenderMode::DateOnly.render_naive_date(end);
                let inclusion_msg = (if *inclusive { "included" } else { "excluded" }).to_string();
                write!(f, "{start} to {end} ({inclusion_msg})")
            }
        }
    }
}

impl FromStr for DateInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Match range
        if let Some((start, end)) = s.split_once("..") {
            let start = utils::date::parse_date(start).map_err(|_| Self::ERR_MSG)?;
            let (end, inclusive) = match end.strip_prefix('=') {
                Some(substr) => (substr, true),
                None => (end, false),
            };
            let end = utils::date::parse_date(end).map_err(|_| Self::ERR_MSG)?;

            if start > end {
                return Err("DateInput: start cannot be after end when using range".to_string());
            }

            return Ok(DateInput::Range {
                start,
                end,
                inclusive,
            });
        }

        // Single date
        let date = utils::date::parse_date(s).map_err(|_| Self::ERR_MSG)?;
        Ok(DateInput::Single(date))
    }
}

#[derive(Args, Debug)]
pub struct ListActivityArgs {
    /// Restrict to entries starting in the given <PERIOD>
    #[arg(
        short = 'p',
        long = "period",
        value_name = "PERIOD",
        value_enum,
        conflicts_with = "date_range"
    )]
    pub period: Option<Period>,

    /// Restrict to entries matching <DATE_RANGE> (YYYY-MM-DD format)
    #[arg(
        short = 'd',
        long = "date",
        value_name = "DATE_RANGE",
        conflicts_with = "period"
    )]
    pub date_range: Option<DateInput>,

    /// Show a per-activity summary instead of listing all logs
    #[arg(short = 's', long = "summary", conflicts_with = "no_grouping")]
    pub show_summary: bool,

    /// Show all activities, even the ones with no log
    #[arg(short = 'a', long = "all", conflicts_with = "no_grouping")]
    pub show_all: bool,

    /// Do not group activity logs by date
    #[arg(short = 'n', long = "no-grouping", conflicts_with_all = ["show_summary", "show_all"])]
    pub no_grouping: bool,

    /// Output in JSON
    #[arg(short = 'j', long = "json")]
    pub use_json_format: bool,
    // /// Only show tags
    // #[arg(short = 't', long = "tags-only", conflicts_with = "no_grouping")]
    // pub tags_only: bool,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Output in JSON
    #[arg(short = 'j', long = "json")]
    pub use_json_format: bool,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default)]
pub enum Period {
    #[value(name = "today", alias = "td", alias = "tod")]
    Today,

    #[value(name = "yesterday", alias = "yd", alias = "ytd")]
    Yesterday,

    #[value(name = "this-week", alias = "tw", alias = "twk", alias = "wk")]
    #[default]
    ThisWeek,

    #[value(
        name = "last-week",
        alias = "lw",
        alias = "lwk",
        alias = "yesterweek",
        alias = "yw",
        alias = "ywk"
    )]
    LastWeek,

    #[value(name = "this-month", alias = "tm", alias = "tmo", alias = "mo")]
    ThisMonth,

    #[value(
        name = "last-month",
        alias = "lm",
        alias = "lmo",
        alias = "yestermonth",
        alias = "ym",
        alias = "ymo"
    )]
    LastMonth,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let now = Local::now();
        let last_month = now - Months::new(1);

        let period = match self {
            Period::Today => "Today".to_string(),
            Period::Yesterday => "Yesterday".to_string(),
            Period::ThisWeek => "This week".to_string(),
            Period::LastWeek => "Last week".to_string(),
            Period::ThisMonth => format!("{} {}", now.format("%B"), now.year()),
            Period::LastMonth => format!("{} {}", last_month.format("%B"), last_month.year()),
        };
        write!(f, "{period}")
    }
}

#[derive(Args, Debug, Default)]
#[group(multiple = false)]
pub struct PrintActivityArgs {
    /// Output in JSON
    #[arg(short = 'j', long = "json")]
    pub use_json_format: bool,
}

#[derive(Args, Debug)]
pub struct SelectActivityArgs {
    /// ID of the activity
    pub activity_id: Id,
}

#[derive(Args, Debug)]
pub struct CreateActivityArgs {
    /// Name of the activity
    pub name: String,

    /// ID of the parent activity
    #[arg(short, long)]
    pub description: Option<String>,

    /// List of tags to apply to the activity
    #[arg(short, long, value_delimiter = ',', action = ArgAction::Append)]
    pub tags: Vec<String>,

    /// Start the new activity automatically
    #[arg(short = 's', long = "start")]
    pub auto_start: bool,

    /// Output in JSON
    #[arg(short = 'j', long = "json")]
    pub use_json_format: bool,
}

#[derive(Args, Debug)]
pub struct ModifyActivityArgs {
    /// ID of the activity to edit
    pub id: Id,

    #[clap(flatten)]
    pub update: UpdateGroup,
}

#[derive(Args, Debug)]
#[group(required = true)]
pub struct UpdateGroup {
    /// New name for the activity
    #[arg(short = 'n', long = "name")]
    pub name: Option<String>,

    /// New description for the activity
    #[arg(short, long)]
    pub description: Option<String>,

    /// New list of tags to use for the activity
    #[arg(short, long, value_delimiter = ',', action = ArgAction::Append, num_args(0..))]
    pub tags: Option<Vec<String>>,
}

#[derive(Args, Debug)]
pub struct EditLogsArgs {
    /// Restrict to entries starting in the given <PERIOD>
    #[arg(
        short = 'p',
        long = "period",
        value_name = "PERIOD",
        value_enum,
        conflicts_with = "date_range"
    )]
    pub period: Option<Period>,

    /// Restrict to entries matching <DATE_RANGE> (YYYY-MM-DD format)
    #[arg(
        short = 'd',
        long = "date",
        value_name = "DATE_RANGE",
        conflicts_with = "period"
    )]
    pub date_range: Option<DateInput>,

    /// Do not include instruction comments in the editable file
    #[arg(short = 'n', long = "no-instructions")]
    pub hide_instructions: bool,
}
