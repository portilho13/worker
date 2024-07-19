use crate::{files, frameworks, helper, wrangler::{self, config}};
use tokio::net::TcpListener;
use std::path::PathBuf;

pub async fn server(ip: &str) {
    let listener = TcpListener::bind(ip)
        .await
        .expect("Failed to Listen");

    println!("Listening on ip {}", ip);

    loop {
        match listener.accept().await {
            Ok((conn, _)) => {
                println!("Received connection from {}", conn.peer_addr().unwrap());
                match handle_conn(conn).await {
                    Ok(data) => {
                        if let Some(project_name) = files::create_folder(data).await {
                            println!("Project Name: {}", project_name);

                            let local_path = helper::get_local_path().await;
                            let project_path = format!("{}/{}", local_path.display(), project_name);
                            println!("Project Path: {}", project_path);

                            let toml_file_path = format!("{}/wrangler.toml", project_path);
                            println!("TOML file path: {}", toml_file_path);

                            let content = helper::read_file_content(PathBuf::from(&toml_file_path)).await;

                            if let Ok(Some(wrangler_cfg)) = config::read_toml_file(&PathBuf::from(&toml_file_path)).await {
                                println!("wrangler_cfg: {:?}", wrangler_cfg);

                                let project_cfg = config::ProjectConfig::new(wrangler_cfg, project_path);
                                println!("{:?}", project_cfg);

                                let package_json_path = format!("{}/package.json", project_cfg.path);
                                
                                #[cfg(unix)]
                                frameworks::json::define_framework(PathBuf::from(package_json_path)).await;

                                wrangler::run::run_wrangler(project_cfg).await;
                            } else {
                                eprintln!("Failed to read TOML, returning...");
                            }
                        } else {
                            println!("Couldn't find project name, returning...");
                        }
                    }
                    Err(e) => eprintln!("Failed to handle connection: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}

async fn handle_conn(mut conn: tokio::net::TcpStream) -> Result<Vec<u8>, std::io::Error> {
    use tokio::io::AsyncReadExt;

    let mut buffer = [0u8; 4]; // Buffer to read size of data
    conn.read_exact(&mut buffer).await?;
    let data_size = u32::from_be_bytes(buffer) as usize;
    let mut data_buffer = vec![0u8; data_size];
    conn.read_exact(&mut data_buffer).await?;
    Ok(data_buffer)
}
