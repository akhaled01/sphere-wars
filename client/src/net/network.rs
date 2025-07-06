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

        if let Ok(encoded) = bincode::serde::encode_to_vec(&join_msg, bincode::config::standard()) {
            let _ = self.socket.send_to(&encoded, self.server_addr);
        }
    }

    pub fn try_recv(&self) -> Option<ServerMessage> {
        let mut buf = [0; 65536]; // Increased buffer size for binary data
        match self.socket.recv_from(&mut buf) {
            Ok((n, _)) => bincode::serde::decode_from_slice(&buf[..n], bincode::config::standard())
                .ok()
                .map(|(msg, _)| msg),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => None,
            Err(_) => None,
        }
    }

    pub fn send_move(&self, position: Vec3, rotation: Quat) {
        let move_msg = ClientMessage::PlayerMove { position, rotation };
        if let Ok(encoded) = bincode::serde::encode_to_vec(&move_msg, bincode::config::standard()) {
            let _ = self.socket.send_to(&encoded, self.server_addr);
        }
    }

    pub fn send_shoot(&self, origin: Vec3, direction: Vec3) {
        let shoot_msg = ClientMessage::PlayerShoot { origin, direction };
        if let Ok(encoded) = bincode::serde::encode_to_vec(&shoot_msg, bincode::config::standard())
        {
            let _ = self.socket.send_to(&encoded, self.server_addr);
        }
    }

    pub fn send_respawn(&self) {
        let respawn_msg = ClientMessage::Respawn;
        if let Ok(encoded) =
            bincode::serde::encode_to_vec(&respawn_msg, bincode::config::standard())
        {
            let _ = self.socket.send_to(&encoded, self.server_addr);
        }
    }

    pub fn send_leave_game(&self) {
        let leave_msg = ClientMessage::LeaveGame;
        if let Ok(encoded) = bincode::serde::encode_to_vec(&leave_msg, bincode::config::standard())
        {
            let _ = self.socket.send_to(&encoded, self.server_addr);
        }
    }

    pub fn player_name(&self) -> &str {
        &self.player_name
    }
}
