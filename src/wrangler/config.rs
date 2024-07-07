use serde::{self, Deserialize, Serialize};
use toml;


#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    number: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    command: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WranglerConfig {
    author: Author,
    version: Version,
    build: Build,
}


pub fn read_toml_file() -> Option<WranglerConfig> {
    let data = r#"
        [author]
        name = "port"

        [version]
        number = "1.0"

        [build]
        command = "npm start"
    "#;

    match toml::from_str(data) {
        Ok(config) => Some(config),
        Err(e) => {
            eprintln!("Error parsing toml file: {}", e);
            None
        }
    }
}