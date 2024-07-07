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
struct ServerSettings {
    author: Author,
    version: Version,
    settings: Settings
}