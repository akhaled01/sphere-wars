#![allow(dead_code)]

use tokio::net::UdpSocket;
use crate::cli;

pub fn print_info(args: &cli::Cli) {
    println!("Tank wars server");
    println!("Host: {}", args.host);
    println!("Port: {}", args.port);
    println!("Difficulty: {}", args.difficulty);
    println!("----------------");
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