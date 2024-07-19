use tokio::fs;
use std::path::PathBuf;

pub async fn get_local_path() -> PathBuf {
    tokio::task::spawn_blocking(|| std::env::current_dir().unwrap())
        .await
        .unwrap()
}

pub async fn read_file_content(path: PathBuf) -> String {
    fs::read_to_string(path).await.unwrap()
}

pub async fn write_to_file(path: PathBuf, data: &str) {
    fs::write(path, data).await.expect("Failed to write data to file");
}
