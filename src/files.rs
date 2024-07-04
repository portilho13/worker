use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    File { path: String, content: Vec<u8> },
    Directory { path: String },
}

pub fn create_folder(data: Vec<u8>) -> io::Result<()> {
    let entries: Vec<Entry> = serde_json::from_slice(&data).unwrap();
    for entry in entries {
        match entry {
            Entry::File { path, content } => {
                println!("Received file: {}", path);
                if let Some(parent) = Path::new(&path).parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(path, content)?;
            }
            Entry::Directory { path } => {
                println!("Received directory: {}", path);
                fs::create_dir_all(path)?;
            }
        }
    }
    Ok(())
}
