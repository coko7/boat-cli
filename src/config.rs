use anyhow::{Context, Result, bail};
use directories::ProjectDirs;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

pub const APP_NAME: &str = "boat";
pub const CONFIG_VAR: &str = "BOAT_CONFIG";
pub const DEFAULT_CONFIG_PATH: &str = "config.toml";
pub const DEFAULT_DB_FILE: &str = "boat.db";

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(rename = "database_path")]
    pub database_path: PathBuf,
}

impl Configuration {
    pub fn create_default() -> Result<Self> {
        let config_file = get_config_file_path()?;
        let config_dir = config_file
            .parent()
            .context("config file should have a parent directory")?;
        let database_path = config_dir.join(DEFAULT_DB_FILE);

        Ok(Self { database_path })
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
