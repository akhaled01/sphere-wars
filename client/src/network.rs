use bevy::prelude::*;
use shared::{ClientMessage, ServerMessage};
use std::net::{SocketAddr, UdpSocket};

#[derive(Resource)]
pub struct NetworkClient {
    socket: UdpSocket,
    server_addr: SocketAddr,
    player_name: String,
}

#[allow(dead_code)]
impl NetworkClient {
    pub fn new(host: String, port: u16, player_name: String) -> Self {
        // Bind to a random local port
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_nonblocking(true).unwrap();
        let server_addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

        Self {
            socket,
            server_addr,
            player_name,
        }
    }

    pub fn join_game(&self) {
        let join_msg = ClientMessage::JoinGame {
            player_name: self.player_name.clone(),
        };

        if let Ok(msg) = serde_json::to_string(&join_msg) {
            let _ = self.socket.send_to(msg.as_bytes(), self.server_addr);
        }
    }

    pub fn try_recv(&self) -> Option<ServerMessage> {
        let mut buf = [0; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((n, _)) => {
                let msg = String::from_utf8_lossy(&buf[..n]);
                serde_json::from_str(&msg).ok()
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => None,
            Err(_) => None,
        }
    }

    pub fn send_move(&self, position: Vec3, rotation: Quat) {
        let move_msg = ClientMessage::PlayerMove { position, rotation };
        if let Ok(msg) = serde_json::to_string(&move_msg) {
            let _ = self.socket.send_to(msg.as_bytes(), self.server_addr);
        }
    }

    pub fn send_shoot(&self, origin: Vec3, direction: Vec3) {
        let shoot_msg = ClientMessage::PlayerShoot { origin, direction };
        if let Ok(msg) = serde_json::to_string(&shoot_msg) {
            let _ = self.socket.send_to(msg.as_bytes(), self.server_addr);
        }
    }

    pub fn send_respawn(&self) {
        let respawn_msg = ClientMessage::Respawn;
        if let Ok(msg) = serde_json::to_string(&respawn_msg) {
            let _ = self.socket.send_to(msg.as_bytes(), self.server_addr);
        }
    }

    pub fn player_name(&self) -> &str {
        &self.player_name
    }
}
