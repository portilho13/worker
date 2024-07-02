use std::{io::Read, net::{TcpListener, TcpStream}};

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
                handle_conn(conn).unwrap();
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        }
    }
}

fn handle_conn(mut conn: TcpStream) -> Result<(), std::io::Error> {
    let mut buffer = [0u8; 1500];
    match conn.read(&mut buffer) {
        Ok(0) => {
            println!("Client disconected");
        },
        Ok(n) => {
            println!("Readed {} bytes from conn", n);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e);
        }
    };
    println!("Buffer: {}", String::from_utf8(buffer.to_vec()).unwrap());
    Ok(())
}