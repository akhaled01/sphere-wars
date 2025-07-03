use clap::Parser;
use tokio::signal;

mod cli;
mod server;
mod utils;

use cli::Cli;
use server::GameServer;
use utils::{create_udp_server_socket, print_info};

#[tokio::main]
async fn main() {
    let mut cli = Cli::parse();

    let host = cli.get_host().await;

    // Validate CLI arguments
    if let Err(error) = cli.validate() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }

    print_info(&cli);

    let listener_socket = create_udp_server_socket(&host, cli.port).await;
    let mut listener = GameServer::new(listener_socket, cli.difficulty);

    // Setup signal handling for graceful shutdown
    tokio::select! {
        _ = listener.listen_and_serve() => {
            println!("Server stopped normally");
        }
        _ = signal::ctrl_c() => {
            println!("Received shutdown signal, notifying clients...");
            listener.shutdown_gracefully().await;
        }
    }
}
