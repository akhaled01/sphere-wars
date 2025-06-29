#![allow(dead_code)]

use crate::cli;
use tokio::net::UdpSocket;

pub fn print_info(args: &cli::Cli) {
    println!("🎮 Maze Wars Multiplayer Server");
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
    println!("Max Players: 8");
    println!("=====================================");
}

pub async fn create_udp_server_socket(host: &str, port: u16) -> UdpSocket {
    let addr = format!("{}:{}", host, port);
    let socket = UdpSocket::bind(addr).await.unwrap();
    socket
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
