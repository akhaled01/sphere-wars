#![allow(dead_code)]

use crate::cli;
use tokio::net::UdpSocket;

pub fn print_info(args: &cli::Cli) {
    println!("🎮 Sphere Wars UDP Server");
    println!("Host: {}", args.host);
    println!("Port: {}", args.port);

    let difficulty_info = match args.difficulty.as_str() {
        "easy" => "Easy (More connections, fewer dead ends)",
        "medium" => "Medium (Balanced maze complexity)",
        "hard" => "Hard (Minimal connections, more dead ends)",
        _ => "Unknown",
    };
    println!("Difficulty: {} - {}", args.difficulty, difficulty_info);
    println!("Maze Size: 12x12 with randomized spawn points");
    println!("=====================================");
}

pub async fn create_udp_server_socket(host: &str, port: u16) -> UdpSocket {
    let addr = format!("{}:{}", host, port);

    // Check if port is available before binding
    match UdpSocket::bind(&addr).await {
        Ok(socket) => {
            println!("✅ Successfully bound to {}", addr);
            socket
        }
        Err(e) => {
            eprintln!("❌ Failed to bind to {}: {}", addr, e);

            match e.kind() {
                std::io::ErrorKind::AddrInUse => {
                    eprintln!("💡 Port {} is already in use. Please:", port);
                    eprintln!("   1. Stop any existing server instances");
                    eprintln!("   2. Wait a few seconds for the port to be released");
                    eprintln!("   3. Try a different port with: --port <PORT>");
                    eprintln!("   4. Check what's using the port with: lsof -i :{}", port);
                }
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("💡 Permission denied. Try:");
                    eprintln!("   1. Using a port above 1024 (current: {})", port);
                    eprintln!("   2. Running with appropriate permissions");
                }
                _ => {
                    eprintln!("💡 Network error: {}", e);
                }
            }

            std::process::exit(1);
        }
    }
}

const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub fn log_info(msg: &str) {
    println!("{}{}{}", GREEN, msg, RESET);
}

pub fn log_warning(msg: &str) {
    println!("{}{}{}", YELLOW, msg, RESET);
}

pub fn log_error(msg: &str) {
    println!("{}{}{}", RED, msg, RESET);
}

pub async fn get_local_ip() -> String {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.connect("8.8.8.8:80").await.unwrap();
    socket.local_addr().unwrap().ip().to_string()
}
