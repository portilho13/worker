use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::helper::{read_file_content, write_to_file};


#[derive(Serialize, Deserialize, Debug)]
struct PackageJson {
    name: String,
    version: String,
    private: bool,
    dependencies: Value,
    scripts: Scripts,
    eslintConfig: Value,
    browserslist: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct Scripts {
    start: String,
    build: String,
    test: String,
    eject: String,
}

pub fn change_package_json(path: PathBuf) {
    let data = read_file_content(path.clone());
    let mut package: PackageJson = serde_json::from_str(&data)
        .expect("Unable to convert data to PackageJson Packet");

    package.scripts.start = "node ./node_modules/react-scripts/bin/react-scripts.js start".to_string();
    package.scripts.build = "node ./node_modules/react-scripts/bin/react-scripts.js build".to_string();
    package.scripts.test = "node ./node_modules/react-scripts/bin/react-scripts.js test".to_string();
    package.scripts.eject = "node ./node_modules/react-scripts/bin/react-scripts.js eject".to_string();

    let updated_data = serde_json::to_string_pretty(&package)
        .expect("Failed to transform changes into PackageJson");
    write_to_file(path, &updated_data.as_str());
}