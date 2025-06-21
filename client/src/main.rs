use bevy::prelude::*;

use plugins::{NetworkPlugin, PlayerPlugin, WorldPlugin};
use systems::utils::{test_server_connection, get_init_plugins};

mod cli;
mod components;
mod network;
mod plugins;
mod systems;

fn main() {
    // Parse command line arguments
    let args = cli::parse_args();
    
    // Test connection to server first
    if !test_server_connection(&args.host, args.port) {
        error!("Failed to connect to server. Exiting...");
        std::process::exit(1);
    }

    // Initialize network client
    let network = network::NetworkClient::new(
        args.host,
        args.port,
        args.name,
    );

    // Join the game
    network.join_game();

    // Start the game with window (only if connection succeeded)
    info!("Starting game...");
    App::new()
        .add_plugins((get_init_plugins(), WorldPlugin, PlayerPlugin, NetworkPlugin))
        .insert_resource(network)
        .run();
}

