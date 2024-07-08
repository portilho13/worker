use std::path::Path;

use serde::{self, Deserialize, Serialize};
use toml;


#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub number: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    pub command: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WranglerConfig {
    pub author: Author,
    pub version: Version,
    pub build: Build,
}

#[derive(Debug)]
pub struct ProjectConfig {
    pub wrangler_config: WranglerConfig,
    pub path: String
}

impl ProjectConfig {
    pub fn new(config: WranglerConfig, path: String) -> Self {
        ProjectConfig {
            wrangler_config: config,
            path: path
        }
    }
}


pub fn read_toml_file(data: String) -> Option<WranglerConfig> {
    match toml::from_str(&data) {
        Ok(config) => Some(config),
        Err(e) => {
            eprintln!("Error parsing toml file: {}", e);
            None
        }
    }
}