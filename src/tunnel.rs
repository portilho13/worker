use std::{io::Read, net::{TcpListener, TcpStream}, path::PathBuf};
use crate::{files, helper, wrangler::{self, config}};

pub fn server(ip: String) {
    let listener = TcpListener::bind(ip.clone()).expect("Failed to bind");

    println!("Listening on ip: {}", ip);

    for conn in listener.incoming() {
        match conn {
            Ok(conn) => {
                println!("Received connection from {}", conn.peer_addr().unwrap());
                match handle_conn(conn) {
                    Ok(data) => {
                        if let Some(project_name) = files::create_folder(data) {
                            println!("Project Name: {}", project_name);

                            let project_path = format!("{}/{}", helper::get_local_path().display(), project_name);
                            println!("Project Path: {}", project_path);

                            let toml_file_path = format!("{}/wrangler.toml", project_path);
                            println!("TOML file path: {}", toml_file_path);

                            let content = helper::read_file_content(PathBuf::from(&toml_file_path));

                            if let Some(wrangler_cfg) = config::read_toml_file(content) {
                                println!("wrangler_cfg: {:?}", wrangler_cfg);

                                let project_cfg = config::ProjectConfig::new(wrangler_cfg, project_path);
                                println!("{:?}", project_cfg);

                                wrangler::run::run_wrangler(project_cfg);
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

fn handle_conn(mut conn: TcpStream) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = [0u8; 4]; // Buffer to read size of data
    conn.read_exact(&mut buffer)?;
    let data_size = u32::from_be_bytes(buffer) as usize;
    let mut data_buffer = vec![0u8; data_size];
    conn.read_exact(&mut data_buffer)?;
    Ok(data_buffer)
}
