mod tunnel;

use std::env;

const TCP_SERVER_IP: &str = "127.0.0.1:1986";

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the required argument is present
    if args.len() < 2 {
        eprintln!("Usage: {} <server_ip>", args[0]);
        std::process::exit(1);
    }

    let ip = &args[1];

    // Call the server function with the provided argument
    tunnel::server(ip.to_string());
}
