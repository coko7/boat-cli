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
    //
    // #[command(flatten)]
    // pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum Commands {
    /// Create a new activity
    #[command(alias = "n")]
    New(CreateActivityArgs),

    // create a backup command
    /// Start/resume an activity
    #[command(alias = "s", alias = "st", alias = "sail")]
    Start(SelectActivityArgs),

    // /// Manage configuration
    // #[command(alias = "c", alias = "cfg", alias = "conf")]
    // Config {},
    /// Pause/stop the current activity
    #[command(alias = "p", alias = "stop")]
    Pause,

    /// Modify an activity
    #[command(alias = "m", alias = "mod")]
    Modify(ModifyActivityArgs),

    /// Delete an activity
    #[command(alias = "d", alias = "del")]
    Delete(SelectActivityArgs),

    /// Get the current activity
    #[command(alias = "g")]
    Get(PrintActivityArgs),

    /// List boat objects
    #[command(alias = "l", alias = "ls")]
    List {
        #[command(subcommand)]
        command: ListSubcommand,
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
pub enum ListSubcommand {
    /// List activity logs
    #[command(name = "logs", alias = "l", alias = "log")]
    Logs(ListActivityArgs),

    /// List activities
    #[command(
        name = "acts",
        alias = "act",
        alias = "a",
        alias = "activity",
        alias = "activities"
    )]
    Activities(ListArgs),

    /// List tags
    #[command(name = "tags", alias = "t", alias = "tag")]
    Tags(ListArgs),
}

#[derive(Args, Debug)]
pub struct ListActivityArgs {
    /// Restrict to entries starting in the given <PERIOD>
    #[arg(short = 'p', long = "period", value_name = "PERIOD", default_value_t = Period::Today, value_enum, conflicts_with_all = ["from", "to", "date"])]
    pub period: Period,

    /// Restrict to entries starting after <DATE> (YYYY-MM-DD format)
    #[arg(short = 'f', long = "from", value_name = "DATE", value_parser = utils::date::parse_date, conflicts_with = "date")]
    pub from: Option<NaiveDate>,

    /// Restrict to entries starting before <DATE> (YYYY-MM-DD format)
    #[arg(short = 't', long = "to", value_name = "DATE", value_parser = utils::date::parse_date, conflicts_with = "date")]
    pub to: Option<NaiveDate>,

    /// Restrict to entries starting and ending on <DATE> (YYYY-MM-DD format)
    #[arg(short = 'd', long = "date", value_name = "DATE", value_parser = utils::date::parse_date, conflicts_with_all = ["period", "from", "to"])]
    pub date: Option<NaiveDate>,

    /// Output in JSON
    #[arg(short = 'j', long = "json")]
    pub use_json_format: bool,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Output in JSON
    #[arg(short = 'j', long = "json")]
    pub use_json_format: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Period {
    #[value(name = "today", alias = "td")]
    Today,
    #[value(name = "yesterday", alias = "yd", alias = "ytd")]
    Yesterday,
    #[value(name = "this-week", alias = "tw", alias = "twk")]
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
    #[value(name = "this-month", alias = "tm", alias = "tmo")]
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
