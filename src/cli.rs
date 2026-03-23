use boat_lib::repository::Id;
use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "boat",
    version,
    about = "Basic Opinionated Activity Tracker",
    author = "coko7",
    long_about = None
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
    #[command(alias = "n")]
    New(CreateActivityArgs),

    // create a backup command
    /// Start/resume an activity
    #[command(alias = "s", alias = "st", alias = "sail")]
    Start(SelectActivityArgs),

    /// Manage configuration
    // #[command(alias = "c", alias = "cfg", alias = "conf")]
    // Config {},

    /// Pause/stop the current activity
    #[command(alias = "p")]
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

    /// List activities and tags
    #[command(alias = "l", alias = "ls")]
    List(ListActivityArgs),
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

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct ListActivityArgs {
    #[arg(short = 'a', long = "all")]
    pub show_all: bool,

    #[arg(short = 'r', long = "recent")]
    pub show_recent: bool,

    #[arg(short = 'c', long = "current")]
    pub show_current: bool,

    #[arg(short = 't', long = "tags")]
    pub show_tags: bool,

    /// Output in pretty format
    #[arg(short = 'p', long = "pretty")]
    pub use_pretty_format: bool,

    /// Output in JSON
    #[arg(short = 'j', long = "json")]
    pub use_json_format: bool,
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
