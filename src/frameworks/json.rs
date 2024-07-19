use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::helper::{read_file_content, write_to_file};
use super::types::{AngularPackage, ProjectTypes, ReactPackage};

pub async fn define_framework(path: PathBuf) {
    let data = read_file_content(path.clone()).await;

    let variants = ProjectTypes::all_variants();
    for variant in variants {
        match variant {
            ProjectTypes::React => {
                match serde_json::from_str::<ReactPackage>(&data) {
                    Ok(mut package) => {
                        println!("Detected React Project");
                        package = change_react_json(package);
                        let updated_data =
                            serde_json::to_string_pretty(&package).expect("Failed to serialize");
                        write_to_file(path.clone(), &updated_data).await;
                        return;
                    }
                    Err(_) => {
                        continue;
                    }
                };
            }
            ProjectTypes::Angular => {
                match serde_json::from_str::<AngularPackage>(&data) {
                    Ok(_) => {
                        println!("Detected Angular Project");
                        return;
                    }
                    Err(_) => {
                        continue;
                    }
                };
            }
        }
    }
    println!("Unknown project type or invalid JSON data");
}

pub fn change_react_json(mut package: ReactPackage) -> ReactPackage {
    package.scripts.start = "node ./node_modules/react-scripts/bin/react-scripts.js start".to_string();
    package.scripts.build = "node ./node_modules/react-scripts/bin/react-scripts.js build".to_string();
    package.scripts.test = "node ./node_modules/react-scripts/bin/react-scripts.js test".to_string();
    package.scripts.eject = "node ./node_modules/react-scripts/bin/react-scripts.js eject".to_string();
    package
}
