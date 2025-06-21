use bevy::prelude::*;

use plugins::{NetworkPlugin, PlayerPlugin, WorldPlugin};
use systems::utils::get_init_plugins;

mod cli;
mod components;
mod network;
mod plugins;
mod systems;

fn main() {
    // Parse command line arguments
    let args = cli::parse_args();
    
    // Initialize network client
    let network = network::NetworkClient::new(
        args.host,
        args.port,
        args.name,
    );

    // Join the game
    network.join_game();

    // Start the game
    App::new()
        .add_plugins((get_init_plugins(), WorldPlugin, PlayerPlugin, NetworkPlugin))
        .insert_resource(network)
        .run();
}
