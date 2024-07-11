use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug)]
pub struct ReactPackage {
    name: String,
    version: String,
    private: bool,
    dependencies: Value,
    pub scripts: ReactScripts,
    eslintConfig: Value,
    browserslist: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReactScripts {
    pub start: String,
    pub build: String,
    pub test: String,
    pub eject: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AngularPackage {
    name: String,
    version: String,
    pub scripts: AngularScripts,
    private: bool,
    dependencies: HashMap<String, String>,
    devDependencies: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AngularScripts {
    ng: String,
    start: String,
    build: String,
    watch: String,
    test: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ProjectTypes {
    Angular,
    React,
}

impl ProjectTypes {
    pub fn all_variants() -> Vec<ProjectTypes> {
        vec![
            ProjectTypes::Angular,
            ProjectTypes::React,
        ]
    }
}