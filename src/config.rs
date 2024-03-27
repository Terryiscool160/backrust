use serde::Deserialize;
use std::collections::HashMap;
use std::{fmt, fs, io};
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BackblazeConfig {
    pub application_id: String,
    pub application_key: String,
    pub bucket_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BackupConfig {
    pub db_name: String,
    pub db_host: String,
    pub bucket: String,
    pub remote_path: String,
    pub databases: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MiscConfig {
    pub backup_interval: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub config: MiscConfig,
    pub hosts: HashMap<String, DatabaseConfig>,
    pub databases: HashMap<String, BackupConfig>,
    pub buckets: HashMap<String, BackblazeConfig>,
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Parse(toml::de::Error),
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "Could not read config: {}", err),
            Error::Parse(err) => write!(f, "Failed to parse config: {}", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Error::Parse(error)
    }
}

pub fn read_config(path: String) -> Result<Config, Error> {
    let file_output = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&file_output)?;

    Ok(config)
}
