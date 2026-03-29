use std::str::FromStr;

use boat_lib::repository::Id;
use chrono::NaiveDate;
use clap::ColorChoice;
use clap::Parser;
use clap::{ArgAction, Args, Subcommand, ValueEnum};

use crate::utils;

#[derive(Parser)]
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

#[derive(Subcommand)]
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

    /// Query boat objects
    #[command(alias = "q")]
    Query {
        #[command(subcommand)]
        command: QuerySubcommand,
    },

    // This is ONLY way I could find to use the 'h' short alias for help.
    #[command(alias = "h", hide = true)]
    HelpExtension,
    // Edit the raw content of activity files
    // #[command(alias = "e", alias = "ed")]
    // Edit(EditFilesArgs),

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
    #[arg(short = 'p', long = "period", value_name = "PERIOD", default_value_t = Period::ThisWeek, value_enum, conflicts_with = "date_range")]
    pub period: Period,

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

    /// Do not group activity logs by date
    #[arg(short = 'n', long = "no-grouping", conflicts_with = "show_summary")]
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

#[derive(ValueEnum, Clone, Debug)]
pub enum Period {
    #[value(name = "today", alias = "td", alias = "tod")]
    Today,
    #[value(name = "yesterday", alias = "yd", alias = "ytd")]
    Yesterday,
    #[value(name = "this-week", alias = "tw", alias = "twk", alias = "wk")]
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

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct PrintActivityArgs {
    /// Output in pretty format
    #[arg(short = 'p', long = "pretty")]
    pub use_pretty_format: bool,

    /// Output in JSON
    #[arg(short = 'j', long = "json")]
    pub use_json_format: bool,
}

impl Default for PrintActivityArgs {
    fn default() -> Self {
        Self {
            use_pretty_format: true,
            use_json_format: false,
        }
    }
}

#[derive(Args, Debug)]
pub struct SelectActivityArgs {
    /// ID of the activity
    pub activity_id: Id,

    /// Output in JSON
    #[arg(short = 'j', long = "json")]
    pub use_json_format: bool,
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
#[group(required = true, multiple = false)]
pub struct EditFilesArgs {
    #[arg(short = 'd', long = "definitions", alias = "def")]
    pub edit_definitions: bool,

    #[arg(short = 'l', long = "logs")]
    pub edit_logs: bool,
}
