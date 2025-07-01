use bevy::prelude::*;

use net::{ConnectionInfo, NetworkClient};
use plugins::{NetworkPlugin, PlayerPlugin, UIPlugin, WorldPlugin};
use systems::utils::{
    get_init_plugins, handle_app_exit, handle_shutdown_signal, setup_signal_handlers,
};

mod components;
mod net;
mod plugins;
mod systems;

fn main() {
    // Get connection info through interactive prompts
    let connection_info = match ConnectionInfo::prompt_user() {
        Ok(info) => info,
        Err(e) => {
            eprintln!("\n❌ Connection failed: {}", e);
            eprintln!("\n🔄 Please restart the client and try again.");
            std::process::exit(1);
        }
    };

    println!("\n🚀 Connecting to game server...");
    let network = NetworkClient::new(
        connection_info.host,
        connection_info.port,
        connection_info.username,
    );

    network.join_game();

    // Set up signal handlers for graceful shutdown
    setup_signal_handlers();

    println!("🎮 Starting Maze Wars...");
    App::new()
        .add_plugins((
            get_init_plugins(),
            WorldPlugin,
            PlayerPlugin,
            NetworkPlugin,
            UIPlugin,
        ))
        .insert_resource(network)
        .add_systems(Update, (handle_shutdown_signal, handle_app_exit))
        .run();
}
