use bevy::prelude::*;

use network::NetworkClient;
use plugins::{NetworkPlugin, PlayerPlugin, WorldPlugin};
use systems::utils::{
    get_init_plugins, handle_app_exit, handle_shutdown_signal, setup_signal_handlers, test_server_connection,
};

mod cli;
mod components;
mod network;
mod plugins;
mod systems;

fn main() {
    let args = cli::parse_args();

    if !test_server_connection(&args.host, args.port) {
        error!("Failed to connect to server. Exiting...");
        std::process::exit(1);
    }

    let network = NetworkClient::new(args.host, args.port, args.name);

    network.join_game();

    // Set up signal handlers for graceful shutdown
    setup_signal_handlers();

    info!("Starting game...");
    App::new()
        .add_plugins((get_init_plugins(), WorldPlugin, PlayerPlugin, NetworkPlugin))
        .insert_resource(network)
        .add_systems(Update, (handle_shutdown_signal, handle_app_exit))
        .run();
}
