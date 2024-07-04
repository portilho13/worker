use std::{io::Read, net::{TcpListener, TcpStream}};

use serde_json::map::Entry;

use crate::files;

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
                files::create_folder(data).unwrap();
                
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