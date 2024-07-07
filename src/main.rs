mod tunnel;
mod files;
mod wrangler;
mod settings;
mod helper;

use std::{env, thread};

const TCP_SERVER_IP: &str = "127.0.0.1:1986";

fn main() {

    // // Call the server function with the provided argument
    // tunnel::server(ip.to_string());
    thread::spawn(move || {
        let args: Vec<String> = env::args().collect();

        // Check if the required argument is present
        if args.len() < 2 {
            eprintln!("Usage: {} <server_ip>", args[0]);
            std::process::exit(1);
        }
    
        let ip = &args[1];
        tunnel::server(ip.to_string());
    });

    if let Some(config) = wrangler::config::read_toml_file() {
        println!("Sucessfully readed wrangler file: {:?}", config);
    } else {
        return
    }
}
