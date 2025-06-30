use bevy::math::{Quat, Vec3};
use rand::Rng;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::time::{Duration, Instant};
use uuid::Uuid;

use shared::{
    ClientMessage, GameState, HitscanResult, MazeConfig, MazeData, Player, ServerMessage,
    SpawnPoint, WeaponConfig, generate_maze_from_config,
};

use crate::utils::log_info;

pub struct GameServer {
    listener: UdpSocket,
    players: HashMap<String, Player>, // Map player_id to Player
    addr_to_id: HashMap<SocketAddr, String>, // Map address to player_id
    state: GameState,
    max_players: usize,
    min_players: usize,
    difficulty: String,
    game_start_time: Option<f64>,
    maze_seed: Option<u64>,
    maze_data: Option<MazeData>, // Store generated maze data with spawn points
    used_spawn_points: Vec<usize>, // Track which spawn points are in use
    pending_respawns: HashMap<String, Instant>, // Track respawn timers
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
            maze_seed: None,
            maze_data: None,
            used_spawn_points: Vec::new(),
            pending_respawns: HashMap::new(),
        }
    }

    pub async fn listen_and_serve(&mut self) {
        loop {
            let mut buf = [0; 1024];
            let (amt, addr) = self.listener.recv_from(&mut buf).await.unwrap();
            let msg = String::from_utf8_lossy(&buf[..amt]);
            self.mux(addr, &msg).await;
        }
    }

    // this handles messages, and replies accordingly
    async fn mux(&mut self, addr: SocketAddr, msg: &str) {
        let client_msg: Result<ClientMessage, _> = serde_json::from_str(msg);
        log_info(&format!("Received from {}: {}", addr, msg));
        match client_msg {
            Ok(message) => match message {
                ClientMessage::TestHealth => {
                    self.handle_test_health(addr).await;
                }
                ClientMessage::JoinGame { player_name } => {
                    self.handle_join_game(addr, player_name).await;
                }
                ClientMessage::LeaveGame => {
                    self.handle_leave_game(addr).await;
                }
                ClientMessage::PlayerMove { position, rotation } => {
                    self.handle_player_move(addr, position, rotation).await;
                }
                ClientMessage::PlayerShoot { origin, direction } => {
                    self.handle_player_shoot(addr, origin, direction).await;
                }
                ClientMessage::Respawn => {
                    self.handle_respawn(addr).await;
                }
            },
            Err(e) => {
                // Send error response for invalid message format
                let error_msg = ServerMessage::Error {
                    message: format!("Invalid message format: {}", e),
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
        self.players.len() >= self.min_players && matches!(self.state, GameState::WaitingForPlayers)
    }

    // Get a random available spawn point
    fn get_random_spawn_point(&mut self) -> Option<SpawnPoint> {
        if let Some(maze_data) = &self.maze_data {
            let available_points: Vec<usize> = (0..maze_data.spawn_points.len())
                .filter(|i| !self.used_spawn_points.contains(i))
                .collect();

            if !available_points.is_empty() {
                let mut rng = rand::thread_rng();
                let selected_idx = available_points[rng.gen_range(0..available_points.len())];
                self.used_spawn_points.push(selected_idx);
                return Some(maze_data.spawn_points[selected_idx].clone());
            }
        }
        None
    }

    // Release a spawn point when player leaves
    fn release_spawn_point(&mut self, position: Vec3) {
        if let Some(maze_data) = &self.maze_data {
            for (idx, spawn_point) in maze_data.spawn_points.iter().enumerate() {
                let distance = spawn_point.position.distance(position);
                if distance < 2.0 {
                    // Close enough to be the same spawn point
                    self.used_spawn_points.retain(|&x| x != idx);
                    break;
                }
            }
        }
    }

    // TestHealth makes sure server is running
    async fn handle_test_health(&mut self, addr: SocketAddr) {
        let health_msg = ServerMessage::HealthCheck;
        if let Ok(response) = serde_json::to_string(&health_msg) {
            self.send_message(addr, &response).await;
        }
    }

    // Handler methods for each message type
    async fn handle_join_game(&mut self, addr: SocketAddr, player_name: String) {
        // Check if player already exists
        if self.addr_to_id.contains_key(&addr) {
            let error_msg = ServerMessage::Error {
                message: "Player already in game".to_string(),
            };
            if let Ok(response) = serde_json::to_string(&error_msg) {
                self.send_message(addr, &response).await;
            }
            return;
        }

        // Check if room is full
        if self.players.len() >= self.max_players {
            let error_msg = ServerMessage::Error {
                message: "Game is full".to_string(),
            };
            if let Ok(response) = serde_json::to_string(&error_msg) {
                self.send_message(addr, &response).await;
            }
            return;
        }

        // check if name is taken
        if self.players.values().any(|p| p.name == player_name) {
            let error_msg = ServerMessage::NameAlreadyTaken;
            if let Ok(response) = serde_json::to_string(&error_msg) {
                self.send_message(addr, &response).await;
            }
            return;
        }

        // Create new player with random color
        let player_id = Uuid::new_v4().to_string();
        let mut player = Player::new(player_id.clone(), player_name);

        // Generate random bright color
        let mut rng = rand::thread_rng();
        player.color = [
            rng.gen_range(0.3..1.0), // Red component (avoid too dark)
            rng.gen_range(0.3..1.0), // Green component
            rng.gen_range(0.3..1.0), // Blue component
        ];

        // Generate maze data if not already generated
        if self.maze_data.is_none() {
            if self.maze_seed.is_none() {
                self.maze_seed = Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as u64,
                );
            }
            let config = MazeConfig::new(self.maze_seed.unwrap(), 12, 12, &self.difficulty);
            self.maze_data = Some(generate_maze_from_config(&config));
        }

        // Assign random spawn point to player
        if let Some(spawn_point) = self.get_random_spawn_point() {
            player.position = spawn_point.position;
            player.rotation = spawn_point.rotation;
        }

        // Add player
        self.players.insert(player_id.clone(), player.clone());
        self.addr_to_id.insert(addr, player_id.clone());

        // Send join confirmation
        let join_msg = ServerMessage::GameJoined { player_id };
        if let Ok(response) = serde_json::to_string(&join_msg) {
            self.send_message(addr, &response).await;
        }

        // Broadcast player joined to others
        let joined_msg = ServerMessage::PlayerJoined {
            player: player.clone(),
        };
        if let Ok(response) = serde_json::to_string(&joined_msg) {
            self.broadcast_to_others(addr, &response).await;
        }

        // Send current game state to new player
        let state_msg = ServerMessage::GameState {
            players: self.players.clone(),
            state: self.state.clone(),
            max_players: self.max_players as u32,
            min_players: self.min_players as u32,
            game_start_time: self.game_start_time,
        };

        // If game has already started, send maze info to new player
        if matches!(self.state, GameState::GameStarted) {
            if let Some(seed) = self.maze_seed {
                let maze_msg = ServerMessage::GameStarted {
                    seed,
                    width: 12,  // Consistent maze size
                    height: 12, // Consistent maze size
                    difficulty: self.difficulty.clone(),
                };
                if let Ok(response) = serde_json::to_string(&maze_msg) {
                    self.send_message(addr, &response).await;
                }
            }
        }
        if let Ok(response) = serde_json::to_string(&state_msg) {
            self.send_message(addr, &response).await;
        }

        // Check if game can start
        if self.can_start_game() {
            self.state = GameState::GameStarted;
            self.game_start_time = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            );

            let start_msg = ServerMessage::GameStarted {
                seed: self.maze_seed.unwrap(),
                width: 12,
                height: 12,
                difficulty: self.difficulty.clone(),
            };
            if let Ok(response) = serde_json::to_string(&start_msg) {
                self.broadcast(&response).await;
            }
        }
    }

    async fn handle_leave_game(&mut self, addr: SocketAddr) {
        if let Some(player_id) = self.addr_to_id.remove(&addr) {
            if let Some(player) = self.players.remove(&player_id) {
                // Release the spawn point for reuse
                self.release_spawn_point(player.position);

                let left_msg = ServerMessage::PlayerLeft {
                    player_id: player.id,
                };
                if let Ok(response) = serde_json::to_string(&left_msg) {
                    self.broadcast(&response).await;
                }
            }
        }
    }

    async fn handle_player_move(&mut self, addr: SocketAddr, position: Vec3, rotation: Quat) {
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

    // Check if a ray from origin to target intersects any walls
    fn ray_intersects_wall(&self, origin: Vec3, target: Vec3) -> bool {
        if let Some(maze_data) = &self.maze_data {
            const TILE_SIZE: f32 = 4.0; // Same as client rendering
            const PLAYER_RADIUS: f32 = 0.8; // Player hitbox radius
            
            let grid = &maze_data.grid;
            let grid_height = grid.len();
            let grid_width = if grid_height > 0 { grid[0].len() } else { 0 };
            
            if grid_width == 0 || grid_height == 0 {
                return false;
            }
            
            // Calculate maze offset (same as client)
            let maze_width_world = grid_width as f32 * TILE_SIZE;
            let maze_height_world = grid_height as f32 * TILE_SIZE;
            let offset_x = -maze_width_world / 2.0 + TILE_SIZE / 2.0;
            let offset_z = -maze_height_world / 2.0 + TILE_SIZE / 2.0;
            
            let ray_dir = (target - origin).normalize();
            let ray_length = origin.distance(target);
            
            // Use smaller step size for more precision
            let step_size = 0.2;
            let num_steps = (ray_length / step_size) as i32;
            
            for i in 1..num_steps { // Start from 1 to skip shooter position, end before target
                let t = (i as f32) * step_size;
                let ray_pos = origin + ray_dir * t;
                
                // Only check at player height (around chest level)
                if ray_pos.y < 1.0 || ray_pos.y > 3.0 {
                    continue;
                }
                
                // Convert world position to grid coordinates
                let world_x = ray_pos.x - offset_x;
                let world_z = ray_pos.z - offset_z;
                
                let grid_x = (world_x / TILE_SIZE).floor() as i32;
                let grid_z = (world_z / TILE_SIZE).floor() as i32;
                
                // Check bounds
                if grid_x >= 0 && grid_x < grid_width as i32 && grid_z >= 0 && grid_z < grid_height as i32 {
                    let grid_x = grid_x as usize;
                    let grid_z = grid_z as usize;
                    
                    // Check if this position has a wall
                    if grid[grid_z][grid_x] {
                        // Additional check: make sure we're not too close to the target
                        // (to account for player standing right next to wall)
                        let remaining_distance = ray_pos.distance(target);
                        if remaining_distance > PLAYER_RADIUS {
                            return true; // Ray hit a wall with sufficient distance from target
                        }
                    }
                }
            }
        }
        
        false // No wall intersection found
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
                            // Check if there's a wall between shooter and target
                            if !self.ray_intersects_wall(origin, other_player.position) {
                                hit_result.hit = true;
                                hit_result.hit_position = Some(other_player.position);
                                hit_result.hit_player_id = Some(other_id.clone());
                                hit_result.distance = distance;
                            }
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

                            // Start respawn timer
                            self.pending_respawns
                                .insert(hit_player_id.clone(), Instant::now());
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
        if let Some(player_id) = self.addr_to_id.get(&addr).cloned() {
            // Check if player is dead and respawn timer has expired first
            let should_respawn = if let Some(player) = self.players.get(&player_id) {
                if !player.is_alive {
                    if let Some(respawn_time) = self.pending_respawns.get(&player_id) {
                        Instant::now().duration_since(*respawn_time) >= Duration::from_secs(3)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if should_respawn {
                // Get spawn point before getting mutable reference to player
                let spawn_point = self.get_random_spawn_point();

                if let Some(player) = self.players.get_mut(&player_id) {
                    // Respawn player at random maze spawn point
                    if let Some(spawn_point) = spawn_point {
                        player.position = spawn_point.position;
                        player.rotation = spawn_point.rotation;
                    } else {
                        // Fallback to default spawn if no spawn points available
                        player.position = Vec3::new(48.0, 2.5, 48.0);
                    }
                    player.health = player.max_health;
                    player.is_alive = true;
                    player.death_time = None;
                    player.last_damage_time = None;
                    player.last_damage_by = None;

                    let respawn_msg = ServerMessage::PlayerRespawned {
                        player_id: player_id.clone(),
                        position: player.position,
                    };
                    if let Ok(response) = serde_json::to_string(&respawn_msg) {
                        self.broadcast(&response).await;
                    }

                    // Remove respawn timer
                    self.pending_respawns.remove(&player_id);
                }
            } else {
                // Check if we should send error message for respawn timer
                if let Some(player) = self.players.get(&player_id) {
                    if !player.is_alive {
                        if let Some(respawn_time) = self.pending_respawns.get(&player_id) {
                            let remaining_time = Duration::from_secs(3)
                                - Instant::now().duration_since(*respawn_time);
                            let error_msg = ServerMessage::Error {
                                message: format!(
                                    "Respawn in {:.1} seconds",
                                    remaining_time.as_secs_f32()
                                ),
                            };
                            if let Ok(response) = serde_json::to_string(&error_msg) {
                                self.send_message(addr, &response).await;
                            }
                        }
                    }
                }
            }
        }
    }

    pub async fn shutdown_gracefully(&self) {
        println!(
            "Sending shutdown notification to {} client{}...",
            self.players.len(),
            if self.players.len() == 1 { "" } else { "s" }
        );

        let shutdown_msg = ServerMessage::GameEnded {
            reason: "Server is shutting down".to_string(),
        };

        if let Ok(response) = serde_json::to_string(&shutdown_msg) {
            self.broadcast(&response).await;
        }

        // Give clients a moment to receive the message
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        println!("Shutdown complete.");
    }
}
