use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReactPackage {
    pub name: String,
    pub version: String,
    pub private: bool,
    pub dependencies: Value,
    pub scripts: ReactScripts,
    pub eslintConfig: Value,
    pub browserslist: Value,
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
    pub name: String,
    pub version: String,
    pub scripts: AngularScripts,
    pub private: bool,
    pub dependencies: HashMap<String, String>,
    pub devDependencies: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AngularScripts {
    pub ng: String,
    pub start: String,
    pub build: String,
    pub watch: String,
    pub test: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ProjectTypes {
    Angular,
    React,
}

impl ProjectTypes {
    pub fn all_variants() -> Vec<ProjectTypes> {
        vec![ProjectTypes::Angular, ProjectTypes::React]
    }
}
