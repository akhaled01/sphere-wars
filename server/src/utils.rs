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