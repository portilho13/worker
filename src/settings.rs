use std::path::Path;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Author {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Version {
    version: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    ip: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSettings {
    author: Author,
    version: Version,
    settings: Settings
}

pub fn read_toml_file(content: String) -> Option<ServerSettings> {
    match toml::from_str(&content) {
        Ok(content) => Some(content),
        Err(e) => {
            eprintln!("Error parsing toml: {}", e);
            None
        }
    }
}