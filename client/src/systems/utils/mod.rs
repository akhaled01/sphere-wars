use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use std::net::SocketAddr;
use std::time::Duration;

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
        // .add(LogDiagnosticsPlugin::default())
}

pub fn test_server_connection(host: &str, port: u16) -> bool {
    use std::net::UdpSocket;

    info!("Testing connection to server at {}:{}...", host, port);

    // Create a temporary socket for testing
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket,
        Err(e) => {
            error!("Failed to create socket: {}", e);
            return false;
        }
    };

    // Set a timeout for the socket
    if let Err(e) = socket.set_read_timeout(Some(Duration::from_secs(5))) {
        error!("Failed to set socket timeout: {}", e);
        return false;
    }

    let server_addr: SocketAddr = match format!("{}:{}", host, port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            error!("Invalid server address: {}", e);
            return false;
        }
    };

    // Send a test message
    let test_msg = shared::ClientMessage::TestHealth;
    let msg_str = match serde_json::to_string(&test_msg) {
        Ok(msg) => msg,
        Err(e) => {
            error!("Failed to serialize message: {}", e);
            return false;
        }
    };

    // Send the message
    if let Err(e) = socket.send_to(msg_str.as_bytes(), server_addr) {
        error!("Failed to send message to server: {}", e);
        return false;
    }

    // Wait for response
    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((n, _)) => {
            let response = String::from_utf8_lossy(&buf[..n]);
            match serde_json::from_str::<shared::ServerMessage>(&response) {
                Ok(_) => {
                    info!("✓ Server connection successful!");
                    true
                }
                Err(e) => {
                    error!("Received invalid response from server: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            error!("No response from server: {}", e);
            error!("Make sure the server is running and accessible.");
            false
        }
    }
}
