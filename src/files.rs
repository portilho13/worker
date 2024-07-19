use serde::{Serialize, Deserialize};
use tokio::fs;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    File { path: String, content: Vec<u8> },
    Directory { path: String },
}

pub async fn create_folder(data: Vec<u8>) -> Option<String> {
    let entries: Vec<Entry> = serde_json::from_slice(&data).unwrap();

    let main_dir_name = if let Some(Entry::Directory { path }) = entries.iter().find(|entry| matches!(entry, Entry::Directory { .. })) {
        path.clone()
    } else {
        eprintln!("No directory entry found in the entries.");
        return None;
    };

    for entry in &entries {
        match entry {
            Entry::File { path, content } => {
                if let Some(parent) = Path::new(&path).parent() {
                    fs::create_dir_all(parent).await.unwrap();
                    
                    // Set permissions synchronously
                    set_permissions_sync(parent).unwrap();
                }

                fs::write(path, content).await.unwrap();

                // Set permissions synchronously
                set_permissions_sync(path).unwrap();
            }
            Entry::Directory { path } => {
                fs::create_dir_all(path).await.unwrap();
                
                // Set permissions synchronously
                set_permissions_sync(path).unwrap();
            }
        }
    }

    Some(main_dir_name)
}

/// Set permissions synchronously
fn set_permissions_sync<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let permissions = Permissions::from_mode(0o777);
    std::fs::set_permissions(path, permissions)
}
