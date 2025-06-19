use std::collections::HashMap;
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use glam::Vec3;
use uuid::Uuid;

use shared::{generate_maze, ClientMessage, GameState, HitscanResult, Player, ServerMessage, WeaponConfig};

pub struct GameServer {
    listener: UdpSocket,
    players: HashMap<String, Player>, // Map player_id to Player
    addr_to_id: HashMap<SocketAddr, String>, // Map address to player_id
    state: GameState,
    max_players: usize,
    min_players: usize,
    difficulty: String,
    game_start_time: Option<f64>,
}

impl GameServer {
    pub fn new(listener: UdpSocket, difficulty: String) -> Self {
        Self {
            listener,
            players: HashMap::new(),
            addr_to_id: HashMap::new(),
            state: GameState::WaitingForPlayers,
            max_players: 8,
            min_players: 1,
            difficulty,
            game_start_time: None,
        }
    }

    pub async fn listen_and_serve(&mut self) {
        loop {
            let mut buf = [0; 1024];
            let (amt, addr) = self.listener.recv_from(&mut buf).await.unwrap();
            let msg = String::from_utf8_lossy(&buf[..amt]);
            println!("Received from {}: {}", addr, msg);
            self.mux(addr, &msg).await;
        }
    }

    // this handles messages, and replies accordingly
    async fn mux(&mut self, addr: SocketAddr, msg: &str) {
        let client_msg: Result<ClientMessage, _> = serde_json::from_str(msg);
        
        match client_msg {
            Ok(message) => {
                match message {
                    ClientMessage::JoinGame { player_name } => {
                        self.handle_join_game(addr, player_name).await;
                    },
                    ClientMessage::LeaveGame => {
                        self.handle_leave_game(addr).await;
                    },
                    ClientMessage::PlayerMove { position, rotation } => {
                        self.handle_player_move(addr, position, rotation).await;
                    },
                    ClientMessage::PlayerShoot { origin, direction } => {
                        self.handle_player_shoot(addr, origin, direction).await;
                    },
                    ClientMessage::Respawn => {
                        self.handle_respawn(addr).await;
                    },
                }
            },
            Err(e) => {
                // Send error response for invalid message format
                let error_msg = ServerMessage::Error { 
                    message: format!("Invalid message format: {}", e) 
                };
                if let Ok(response) = serde_json::to_string(&error_msg) {
                    self.send_message(addr, &response).await;
                }
            }
        }
    }

    async fn send_message(&self, addr: SocketAddr, msg: &str) {
        self.listener.send_to(msg.as_bytes(), addr).await.unwrap();
    }

    async fn broadcast(&self, msg: &str) {
        for addr in self.addr_to_id.keys() {
            self.send_message(*addr, msg).await;
        }
    }

    async fn broadcast_to_others(&self, exclude_addr: SocketAddr, msg: &str) {
        for addr in self.addr_to_id.keys() {
            if *addr != exclude_addr {
                self.send_message(*addr, msg).await;
            }
        }
    }

    fn can_start_game(&self) -> bool {
        self.players.len() >= self.min_players && 
        matches!(self.state, GameState::WaitingForPlayers)
    }

    // Handler methods for each message type
    async fn handle_join_game(&mut self, addr: SocketAddr, player_name: String) {
        // Check if player already exists
        if self.addr_to_id.contains_key(&addr) {
            let error_msg = ServerMessage::Error { 
                message: "Player already in game".to_string() 
            };
            if let Ok(response) = serde_json::to_string(&error_msg) {
                self.send_message(addr, &response).await;
            }
            return;
        }

        // Check if room is full
        if self.players.len() >= self.max_players {
            let error_msg = ServerMessage::Error { 
                message: "Game is full".to_string() 
            };
            if let Ok(response) = serde_json::to_string(&error_msg) {
                self.send_message(addr, &response).await;
            }
            return;
        }

        // Create new player
        let player_id = Uuid::new_v4().to_string();
        let player = Player::new(player_id.clone(), player_name);

        // Add player
        self.players.insert(player_id.clone(), player.clone());
        self.addr_to_id.insert(addr, player_id.clone());

        // Send join confirmation
        let join_msg = ServerMessage::GameJoined { player_id };
        if let Ok(response) = serde_json::to_string(&join_msg) {
            self.send_message(addr, &response).await;
        }

        // Broadcast player joined to others
        let joined_msg = ServerMessage::PlayerJoined { player: player.clone() };
        if let Ok(response) = serde_json::to_string(&joined_msg) {
            self.broadcast_to_others(addr, &response).await;
        }

        // Send current game state
        let state_msg = ServerMessage::GameState {
            players: self.players.clone(),
            state: self.state.clone(),
            max_players: self.max_players as u32,
            min_players: self.min_players as u32,
            game_start_time: self.game_start_time,
        };
        if let Ok(response) = serde_json::to_string(&state_msg) {
            self.send_message(addr, &response).await;
        }

        // Check if game can start
        if self.can_start_game() {
            self.state = GameState::GameStarted;
            self.game_start_time = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64());

            let maze = generate_maze(12, 12, &self.difficulty);
            let start_msg = ServerMessage::GameStarted { maze };
            if let Ok(response) = serde_json::to_string(&start_msg) {
                self.broadcast(&response).await;
            }
        }
    }

    async fn handle_leave_game(&mut self, addr: SocketAddr) {
        if let Some(player_id) = self.addr_to_id.remove(&addr) {
            if let Some(player) = self.players.remove(&player_id) {
                let left_msg = ServerMessage::PlayerLeft { 
                    player_id: player.id 
                };
                if let Ok(response) = serde_json::to_string(&left_msg) {
                    self.broadcast(&response).await;
                }
            }
        }
    }

    async fn handle_player_move(&mut self, addr: SocketAddr, position: Vec3, rotation: glam::Quat) {
        if let Some(player_id) = self.addr_to_id.get(&addr) {
            if let Some(player) = self.players.get_mut(player_id) {
                player.position = position;
                player.rotation = rotation;

                let move_msg = ServerMessage::PlayerMoved { 
                    player_id: player_id.clone(),
                    position,
                    rotation,
                };
                if let Ok(response) = serde_json::to_string(&move_msg) {
                    self.broadcast_to_others(addr, &response).await;
                }
            }
        }
    }

    async fn handle_player_shoot(&mut self, addr: SocketAddr, origin: Vec3, direction: Vec3) {
        if let Some(shooter_id) = self.addr_to_id.get(&addr) {
            if let Some(_) = self.players.get(shooter_id) {
                let weapon_config = WeaponConfig::default();
                let mut hit_result = HitscanResult {
                    hit: false,
                    hit_position: None,
                    hit_player_id: None,
                    distance: weapon_config.range,
                };

                // Check for hits against other players
                for (other_id, other_player) in self.players.iter() {
                    if other_id != shooter_id && other_player.is_alive {
                        let distance = origin.distance(other_player.position);
                        if distance <= weapon_config.range && distance < hit_result.distance {
                            hit_result.hit = true;
                            hit_result.hit_position = Some(other_player.position);
                            hit_result.hit_player_id = Some(other_id.clone());
                            hit_result.distance = distance;
                        }
                    }
                }

                // Apply damage if hit
                if let Some(ref hit_player_id) = hit_result.hit_player_id {
                    if let Some(hit_player) = self.players.get_mut(hit_player_id) {
                        let died = hit_player.take_damage(weapon_config.damage);
                        
                        let damage_msg = ServerMessage::PlayerDamaged {
                            player_id: hit_player_id.clone(),
                            damage: weapon_config.damage,
                            health: hit_player.health,
                            damage_by: shooter_id.clone(),
                        };
                        if let Ok(response) = serde_json::to_string(&damage_msg) {
                            self.broadcast(&response).await;
                        }

                        if died {
                            // Update killer stats
                            if let Some(killer) = self.players.get_mut(shooter_id) {
                                killer.kills += 1;
                            }

                            let death_msg = ServerMessage::PlayerDied {
                                player_id: hit_player_id.clone(),
                                killer_id: Some(shooter_id.clone()),
                            };
                            if let Ok(response) = serde_json::to_string(&death_msg) {
                                self.broadcast(&response).await;
                            }
                        }
                    }
                }

                let shot_msg = ServerMessage::PlayerShot {
                    player_id: shooter_id.clone(),
                    origin,
                    direction,
                    hit_result,
                };
                if let Ok(response) = serde_json::to_string(&shot_msg) {
                    self.broadcast(&response).await;
                }
            }
        }
    }

    async fn handle_respawn(&mut self, addr: SocketAddr) {
        if let Some(player_id) = self.addr_to_id.get(&addr) {
            if let Some(player) = self.players.get_mut(player_id) {
                if !player.is_alive {
                    player.respawn();

                    let respawn_msg = ServerMessage::PlayerRespawned {
                        player_id: player_id.clone(),
                        position: player.position,
                    };
                    if let Ok(response) = serde_json::to_string(&respawn_msg) {
                        self.broadcast(&response).await;
                    }
                }
            }
        }
    }
}
