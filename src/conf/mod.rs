
use std::fs;
use std::path::{Path};
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AppConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) authentication: AuthConfig,
    pub(crate) database: DatabaseConfig,
    pub(crate) cache: CacheConfig,
    pub(crate) log: Option<LogConfig>,
    pub(crate) kubernetes: KubeConfig,
    pub(crate) custom: Option<CustomConfig>,
}

impl AppConfig {
    pub fn new(config: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config= toml::from_str(config);
        match config {
            Ok(config) => Ok(config),
            Err(e) => {
                eprintln!("{:?}", e);
                Err(Box::new(e))
            },
        }
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let config = fs::read_to_string(path)?;
        Self::new(&config)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthConfig {
    admin_user: Option<String>,
    admin_password: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub(crate) driver: String,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) database: String,
    pub(crate) socket: Option<String>,
    pub(crate) pool_size: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CacheConfig {
    pub(crate) driver: String,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) cache_size: Option<u64>,
    pub(crate) namespace: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>
}


#[derive(Deserialize, Debug, Clone)]
pub struct LogConfig {
    pub(crate) access_log: Option<String>,
    pub(crate) error_log: Option<String>,
    pub(crate) level: Option<String>
}


#[derive(Deserialize, Debug, Clone)]
pub struct KubeConfig {

}

#[derive(Deserialize, Debug, Clone)]
pub struct CustomConfig {
    title: Option<String>,
    platform_name: String,
}