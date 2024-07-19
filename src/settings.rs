use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    pub ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSettings {
    pub author: Author,
    pub version: Version,
    pub conn: Settings,
    pub api: Api
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
