use std::{env, fs, path::PathBuf};

pub fn get_local_path() -> PathBuf {
    let path = env::current_dir().unwrap();
    path
}

pub fn read_file_content(path: PathBuf) -> String {
    let content = fs::read_to_string(path).unwrap();
    content
}

pub fn write_to_file(path: PathBuf, data: &str) {
    fs::write(path, data).expect("Failed to write data to file");
}