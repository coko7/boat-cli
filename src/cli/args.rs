use boat_lib::repository::Id;
use clap::ColorChoice;
use clap::Parser;
use clap::{ArgAction, Args, Subcommand, ValueEnum};

use crate::cli::PeriodInput;

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

    /// List activity logs
    #[command(alias = "l", alias = "ls")]
    List(FilterActivitiesArgs),

    /// Show activity summaries
    #[command(alias = "r", alias = "report")]
    Report(FilterActivitiesArgs),

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
    Logs(FilterActivitiesArgs),

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

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum GroupBy {
    #[value(name = "none", alias = "no")]
    #[default]
    None,
    #[value(name = "day", alias = "d")]
    Day,
    #[value(name = "week", alias = "wk", alias = "w")]
    Week,
    #[value(name = "month", alias = "mo", alias = "m")]
    Month,
    #[value(name = "year", alias = "y")]
    Year,
}

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum SortBy {
    #[value(name = "none", alias = "no")]
    #[default]
    None,
}

#[derive(Args, Debug)]
pub struct FilterActivitiesArgs {
    /// Restrict to entries matching a given time period
    #[arg(
        short = 'p',
        long = "period",
        help = "Period: day|d, week|w, month|m, year|y, <date>, or <start>..<end>"
    )]
    pub period: PeriodInput,

    /// Specify how entries should be grouped
    #[arg(short = 'g', long = "group-by")]
    pub group_by: bool,

    // /// Specify how entries should be sorted
    // #[arg(short = 's', long = "sort-by")]
    // pub sort_by: SortInput,
    /// Show all activities, even the ones with no log
    #[arg(short = 'a', long = "all", conflicts_with = "no_grouping")]
    pub show_all: bool,

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
    pub period: PeriodInput,

    /// Do not include instruction comments in the editable file
    #[arg(short = 'n', long = "no-instructions")]
    pub hide_instructions: bool,
}
