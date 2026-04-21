use anyhow::{Context, Result, bail};
use directories::ProjectDirs;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

use crate::cli::PeriodInput;

pub const APP_NAME: &str = "boat";
pub const CONFIG_VAR: &str = "BOAT_CONFIG";
pub const DEFAULT_CONFIG_PATH: &str = "config.toml";
pub const DEFAULT_DB_FILE: &str = "boat.db";

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum OutputFormat {
    #[serde(rename = "plain")]
    #[default]
    Plain,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "csv")]
    Csv,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// Path to the SQLite database file
    #[serde(rename = "database_path")]
    pub database_path: PathBuf,

    /// Default period to use for activities
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PeriodInput>,

    /// Default output format to use ('plain', 'json', 'csv')
    #[serde(rename = "format")]
    pub format: OutputFormat,

    /// Configuration values for the various commands
    #[serde(rename = "commands")]
    pub commands: CommandsConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CommandsConfig {
    pub new: NewCommandConfig,
    pub start: StartCommandConfig,
    pub cancel: CancelCommandConfig,
    // pub pause: PauseCommandConfig,
    // pub modify: ModifyCommandConfig,
    pub edit: EditCommandConfig,
    pub delete: DeleteCommandConfig,
    // pub get: GetCommandConfig,
    pub list: ListCommandConfig,
    pub report: ReportCommandConfig,
}

/// Configuration values for the new command
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NewCommandConfig {
    /// Start the new activity automatically
    #[serde(rename = "auto_start")]
    pub auto_start: bool,
}

/// Configuration values for the start command
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StartCommandConfig {
    /// Allow to create and start a new activity by specifying its name instead of an activity number
    #[serde(rename = "quick_start")]
    pub quick_start: bool,
}

/// Configuration values for the cancel command
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CancelCommandConfig {
    /// Prompts for confirmation before cancelling activity
    pub confirm: bool,
}

/// Configuration values for the pause command
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PauseCommandConfig;

/// Configuration values for the modify command
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ModifyCommandConfig;

/// Configuration values for the edit command
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EditCommandConfig {
    /// Restrict to entries matching the given <PERIOD>
    pub period: Option<PeriodInput>,

    /// Do not include instruction comments in the editable file
    #[serde(rename = "show_instructions")]
    pub show_instructions: bool,

    /// Do not include activity definitions in the editable file
    #[serde(rename = "show_activity_definitions")]
    pub show_activity_definitions: bool,

    /// Prompts for confirmation before applying changes
    pub confirm: bool,
}

/// Configuration values for the delete command
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteCommandConfig {
    /// Prompts for confirmation before deleting an activity and all its logs
    pub confirm: bool,
}

/// Configuration values for the get command
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetCommandConfig;

/// Configuration values for the list command
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListCommandConfig {
    /// Restrict to entries matching the given <PERIOD>
    pub period: Option<PeriodInput>,
    // /// Specify how entries should be grouped
    // #[serde(rename = "group_by")]
    // pub group_by: Option<String>,
    // /// Specify how entries should be sorted
    // #[serde(rename = "sort_by")]
    // pub sort_by: Option<String>,
    // /// Format to use for data ('plain', 'json', 'csv')
    // pub format: Option<OutputFormat>,
}

/// Configuration values for the report command
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReportCommandConfig {
    /// Restrict to entries matching the given <PERIOD>
    pub period: Option<PeriodInput>,
    // /// Specify how entries should be grouped
    // #[serde(rename = "group_by")]
    // pub group_by: Option<String>,
    // /// Specify how entries should be sorted
    // #[serde(rename = "sort_by")]
    // pub sort_by: Option<String>,
    // /// Format to use for data ('plain', 'json', 'csv')
    // pub format: Option<OutputFormat>,
}

impl Configuration {
    pub fn create_default() -> Result<Self> {
        let config_file = get_config_file_path()?;
        let config_dir = config_file
            .parent()
            .context("config file should have a parent directory")?;
        let database_path = config_dir.join(DEFAULT_DB_FILE);

        Ok(Self {
            database_path,
            period: None,
            format: OutputFormat::Plain,
            commands: CommandsConfig {
                new: NewCommandConfig { auto_start: false },
                start: StartCommandConfig { quick_start: false },
                cancel: CancelCommandConfig { confirm: true },
                // pause: PauseCommandConfig,
                // modify: ModifyCommandConfig,
                edit: EditCommandConfig {
                    period: None,
                    show_instructions: true,
                    show_activity_definitions: true,
                    confirm: true,
                },
                delete: DeleteCommandConfig { confirm: true },
                // get: GetCommandConfig,
                list: ListCommandConfig {
                    period: None,
                    // group_by: None,
                    // sort_by: None,
                    // format: None,
                },
                report: ReportCommandConfig {
                    period: None,
                    // group_by: None,
                    // sort_by: None,
                    // format: None,
                },
            },
        })
    }

    pub fn load_from_fs() -> Result<Configuration> {
        let config_file_path = get_config_file_path()?;
        let content = fs::read_to_string(config_file_path)?;

        info!("parsing config toml");
        let config: Configuration = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn to_toml_str(&self) -> Result<String> {
        let toml = toml::to_string(&self)?;
        debug!("config serialized to TOML: {}", toml);
        Ok(toml)
    }
}

pub fn get_config_file_path() -> Result<PathBuf> {
    if let Ok(config_var) = env::var(CONFIG_VAR) {
        let val = PathBuf::from(config_var);
        debug!(
            "get config from env: {} = {}",
            CONFIG_VAR,
            val.to_string_lossy()
        );

        return Ok(val);
    }

    if let Some(proj_dirs) = ProjectDirs::from("", "", APP_NAME) {
        let config_dir = proj_dirs.config_dir();
        let config_path = config_dir.join(DEFAULT_CONFIG_PATH);
        debug!(
            "get default config path from proj dirs: {}",
            config_path.display()
        );
        return Ok(config_path);
    }

    bail!("could not get config directory")
}

pub fn initialize_config() -> Result<()> {
    let config_path = get_config_file_path()?;
    if let Some(parent) = config_path.parent() {
        info!("creating config dir at: {}", parent.display());
        fs::create_dir_all(parent)?;
    }

    let config = Configuration::create_default()?;
    let toml = config.to_toml_str()?;
    debug!("generating default config: {config:?}");

    debug!("writing config to: {}", config_path.display());
    fs::write(config_path, toml)?;
    Ok(())
}
