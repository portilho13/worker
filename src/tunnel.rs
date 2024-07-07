use std::{io::Read, net::{TcpListener, TcpStream}, path::PathBuf};
use crate::{files, helper, wrangler::config};

pub fn server(ip: String) {
    let listener = match TcpListener::bind(ip.clone()) {
        Ok(listener) => {
            println!("Listenning on ip: {}", ip);
            listener
        },
        Err(e) => {
            eprintln!("error: {}", e);
            return;
        }
    };

    for conn in listener.incoming() {
        match conn {
            Ok(conn) => {
                println!("Recived connection from {}", conn.peer_addr().unwrap());
                let data = handle_conn(conn).unwrap();
                let project_name = match files::create_folder(data) {
                    Some(project_name) => project_name,
                    None => {
                        println!("Couldnt find project name, returning...");
                        return
                    }
                };

                println!("Project Name {}", project_name);

                let project_path = helper::get_local_path().display().to_string() + "/" + &project_name;

                println!("Project Path {}", project_path);

                let toml_file_path = project_path.clone() + "/wrangler.toml";

                println!("toml_file_path {}", toml_file_path);

                let content = helper::read_file_content(PathBuf::from(toml_file_path));


                let wrangler_cfg = match config::read_toml_file(content) {
                    Some(wrangler_cfg) => wrangler_cfg,
                    None => {
                        eprintln!("Failed to read toml, returning...");
                        return
                    }
                };

                println!("wranger_cfg {:?}", wrangler_cfg);

                let project_cfg = config::ProjectConfig::new(wrangler_cfg, project_path);

                println!("{:?}", project_cfg);

                
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        }
    }
}

fn handle_conn(mut conn: TcpStream) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = [0u8; 4]; // Buffer to read size of data
    match conn.read_exact(&mut buffer) {
        Ok(_) => {
            println!("Read 4 bytes for data length");
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e);
        }
    };
    println!("Buffer content: {:?}", buffer);

    let data_size = u32::from_be_bytes(buffer) as usize;

    let mut data_buffer = vec![0u8; data_size];
    match conn.read_exact(&mut data_buffer) {
        Ok(_) => {
            println!("Read {} bytes from conn", data_size);
        },
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    }

    Ok(data_buffer)
}