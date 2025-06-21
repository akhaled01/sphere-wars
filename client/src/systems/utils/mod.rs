use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use std::time::Duration;
use std::net::SocketAddr;

pub fn get_init_plugins() -> impl PluginGroup {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mazey".into(),
                name: Some("bevy.app".into()),
                resolution: (1920., 1080.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: true,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        })
        .add(FrameTimeDiagnosticsPlugin::default())
        .add(LogDiagnosticsPlugin::default())
}

pub fn test_server_connection(host: &str, port: u16, player_name: &str) -> bool {
    use std::net::UdpSocket;
    
    println!("Testing connection to server at {}:{}...", host, port);
    
    // Create a temporary socket for testing
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket,
        Err(e) => {
            println!("Failed to create socket: {}", e);
            return false;
        }
    };
    
    // Set a timeout for the socket
    if let Err(e) = socket.set_read_timeout(Some(Duration::from_secs(5))) {
        println!("Failed to set socket timeout: {}", e);
        return false;
    }
    
    let server_addr: SocketAddr = match format!("{}:{}", host, port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            println!("Invalid server address: {}", e);
            return false;
        }
    };
    
    // Send a join game message
    let join_msg = shared::ClientMessage::JoinGame {
        player_name: player_name.to_string(),
    };
    
    let msg_str = match serde_json::to_string(&join_msg) {
        Ok(msg) => msg,
        Err(e) => {
            println!("Failed to serialize message: {}", e);
            return false;
        }
    };
    
    // Send the message
    if let Err(e) = socket.send_to(msg_str.as_bytes(), server_addr) {
        println!("Failed to send message to server: {}", e);
        return false;
    }
    
    // Wait for response
    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((n, _)) => {
            let response = String::from_utf8_lossy(&buf[..n]);
            match serde_json::from_str::<shared::ServerMessage>(&response) {
                Ok(_) => {
                    println!(" Server connection successful!");
                    true
                }
                Err(e) => {
                    println!("Received invalid response from server: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            println!("No response from server: {}", e);
            println!("Make sure the server is running and accessible.");
            false
        }
    }
}
