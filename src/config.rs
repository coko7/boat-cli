use anyhow::{Context, Result, bail};
use directories::ProjectDirs;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

pub const APP_NAME: &str = "boat";
pub const CONFIG_VAR: &str = "BOAT_CONFIG";
pub const DEFAULT_CONFIG_PATH: &str = "config.toml";

pub const DEFAULT_ACT_DEFS_PATH: &str = "act_defs.csv";
pub const DEFAULT_ACT_LOGS_PATH: &str = "act_logs.csv";

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(rename = "activity_definitions_path")]
    pub activity_definitions_path: PathBuf,

    #[serde(rename = "activity_logs_path")]
    pub activity_logs_path: PathBuf,
}

impl Configuration {
    pub fn new() -> Result<Self> {
        let config_file = get_config_file()?;
        let config_dir = config_file.parent().context("wait")?;

        let activity_definitions_path = config_dir.join(DEFAULT_ACT_DEFS_PATH);
        let activity_logs_path = config_dir.join(DEFAULT_ACT_LOGS_PATH);

        Ok(Self {
            activity_definitions_path,
            activity_logs_path,
        })
    }

    pub fn to_toml_str(&self) -> Result<String> {
        let toml = toml::to_string(&self)?;
        debug!("config serialized to TOML: {}", toml);
        Ok(toml)
    }
}

pub fn get_config_file() -> Result<PathBuf> {
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
    let config_path = get_config_file()?;
    if let Some(parent) = config_path.parent() {
        info!("creating config dir at: {}", parent.display());
        fs::create_dir_all(parent)?;
    }

    let config = Configuration::new()?;
    let toml = config.to_toml_str()?;
    debug!("generating default config: {config:?}");

    debug!("writing config to: {}", config_path.display());
    fs::write(config_path, toml)?;
    Ok(())
}
