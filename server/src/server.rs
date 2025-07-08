use bevy::math::{Quat, Vec3};
use rand::Rng;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::time::{Duration, Instant};
use uuid::Uuid;

use shared::{
    ClientMessage, GameState, HitscanResult, MAZE_HEIGHT, MAZE_WIDTH, MazeConfig, MazeData, Player,
    ServerMessage, SpawnPoint, WeaponConfig, generate_maze_from_config,
};

use crate::utils::{log_error, log_info};

pub struct GameServer {
    listener: UdpSocket,
    players: HashMap<String, Player>,
    addr_to_id: HashMap<SocketAddr, String>,
    state: GameState,
    difficulty: String,
    game_start_time: Option<f64>,
    maze_seed: Option<u64>,
    maze_data: Option<MazeData>,
    used_spawn_points: Vec<usize>,
    pending_respawns: HashMap<String, Instant>,
}

impl GameServer {
    pub fn new(listener: UdpSocket, difficulty: String) -> Self {
        Self {
            listener,
            players: HashMap::new(),
            addr_to_id: HashMap::new(),
            state: GameState::WaitingForPlayers,
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
            let mut buf = [0; 4096];
            let (amt, addr) = self.listener.recv_from(&mut buf).await.unwrap();
            if let Ok((client_msg, _)) =
                bincode::serde::decode_from_slice(&buf[..amt], bincode::config::standard())
            {
                self.mux(addr, client_msg).await;
            }
        }
    }

    // this handles messages, and replies accordingly
    async fn mux(&mut self, addr: SocketAddr, msg: ClientMessage) {
        match msg {
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
        }
    }

    async fn send_message(&self, addr: SocketAddr, msg: &ServerMessage) {
        let encoded = match bincode::serde::encode_to_vec(msg, bincode::config::standard()) {
            Ok(bytes) => bytes,
            Err(e) => {
                log_error(&format!("ERROR: Failed to serialize message: {}", e));
                return;
            }
        };

        match self.listener.send_to(&encoded, addr).await {
            Ok(bytes_sent) => {
                if bytes_sent != encoded.len() {
                    log_info(&format!(
                        "WARNING: Only sent {} of {} bytes to {}",
                        bytes_sent,
                        encoded.len(),
                        addr
                    ));
                }
            }
            Err(e) => {
                log_error(&format!("ERROR: Failed to send message to {}: {}", addr, e));
            }
        }
    }

    async fn broadcast(&self, msg: &ServerMessage) {
        for addr in self.addr_to_id.keys() {
            self.send_message(*addr, msg).await;
        }
    }

    async fn broadcast_to_others(&self, exclude_addr: SocketAddr, msg: &ServerMessage) {
        for addr in self.addr_to_id.keys() {
            if *addr != exclude_addr {
                self.send_message(*addr, msg).await;
            }
        }
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
        self.send_message(addr, &health_msg).await;
    }

    // Handler methods for each message type
    async fn handle_join_game(&mut self, addr: SocketAddr, player_name: String) {
        log_info(&format!("Player {} joined", player_name));
        // Check if player already exists
        if self.addr_to_id.contains_key(&addr) {
            let error_msg = ServerMessage::Error {
                message: "Player already in game".to_string(),
            };
            self.send_message(addr, &error_msg).await;
            return;
        }

        // check if name is taken
        if self.players.values().any(|p| p.name == player_name) {
            let error_msg = ServerMessage::NameAlreadyTaken;
            self.send_message(addr, &error_msg).await;
            return;
        }

        // Create new player with random color
        let player_id = Uuid::new_v4().to_string();
        let mut player = Player::new(player_id.clone(), player_name.clone());

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
        log_info(&format!("Player {} joined", player_name));
        self.players.insert(player_id.clone(), player.clone());
        self.addr_to_id.insert(addr, player_id.clone());

        // Send join confirmation
        log_info(&format!("sending GameJoined to {}", player_name));
        let join_msg = ServerMessage::GameJoined { player_id };
        self.send_message(addr, &join_msg).await;

        // Broadcast player joined to others
        log_info(&format!("sending PlayerJoined to {}", player_name));
        let joined_msg = ServerMessage::PlayerJoined {
            player: player.clone(),
        };
        self.broadcast_to_others(addr, &joined_msg).await;

        // Send current game state to new player FIRST
        log_info(&format!("sending GameState to {}", player_name));
        let state_msg = ServerMessage::GameState {
            players: self.players.clone(),
            state: self.state.clone(),
            game_start_time: self.game_start_time,
        };
        self.send_message(addr, &state_msg).await;

        // Add a small delay to ensure GameState is processed before GameStarted
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Then if game has already started, send maze info to new player
        if matches!(self.state, GameState::GameStarted) {
            if let Some(seed) = self.maze_seed {
                log_info(&format!("sending GameStarted to {}", player_name));
                let maze_msg = ServerMessage::GameStarted {
                    seed,
                    width: MAZE_WIDTH,
                    height: MAZE_HEIGHT,
                    difficulty: self.difficulty.clone(),
                };
                self.send_message(addr, &maze_msg).await;
            }
        }

        // Check if game can start (only if not already started)
        if self.players.len() >= 1 && !matches!(self.state, GameState::GameStarted) {
            self.state = GameState::GameStarted;
            self.game_start_time = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            );

            let start_msg = ServerMessage::GameStarted {
                seed: self.maze_seed.unwrap(),
                width: MAZE_WIDTH,
                height: MAZE_HEIGHT,
                difficulty: self.difficulty.clone(),
            };
            self.broadcast(&start_msg).await;
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
                self.broadcast(&left_msg).await;
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
                self.broadcast_to_others(addr, &move_msg).await;
            }
        }
    }

    // Check if a ray from origin to target intersects any walls
    fn ray_intersects_wall(&self, origin: Vec3, target: Vec3) -> bool {
        if let Some(maze_data) = &self.maze_data {
            const TILE_SIZE: f32 = 4.0; // Same as client rendering
            const WALL_HEIGHT: f32 = 8.0; // Wall height from client rendering
            const PLAYER_RADIUS: f32 = 1.5; // Player hitbox radius

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

            // Check each wall tile for ray-box intersection
            for (z, row) in grid.iter().enumerate() {
                for (x, &is_wall) in row.iter().enumerate() {
                    if is_wall {
                        // Calculate wall box bounds
                        let wall_center_x = x as f32 * TILE_SIZE + offset_x;
                        let wall_center_z = z as f32 * TILE_SIZE + offset_z;

                        let box_min = Vec3::new(wall_center_x, 0.0, wall_center_z);
                        let box_max = Vec3::new(
                            wall_center_x + TILE_SIZE,
                            WALL_HEIGHT,
                            wall_center_z + TILE_SIZE,
                        );

                        // Perform ray-box intersection test
                        if let Some(hit_distance) =
                            self.ray_box_intersection(origin, ray_dir, box_min, box_max)
                        {
                            // Check if intersection is within ray length and not too close to target
                            if hit_distance > 0.1 && hit_distance < ray_length - PLAYER_RADIUS {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false // No wall intersection found
    }

    // Ray-box intersection using the slab method
    fn ray_box_intersection(
        &self,
        ray_origin: Vec3,
        ray_dir: Vec3,
        box_min: Vec3,
        box_max: Vec3,
    ) -> Option<f32> {
        let mut t_min: f32 = 0.0;
        let mut t_max: f32 = f32::INFINITY;

        // Check intersection with each axis
        for i in 0..3 {
            let origin_component = match i {
                0 => ray_origin.x,
                1 => ray_origin.y,
                2 => ray_origin.z,
                _ => unreachable!(),
            };
            let dir_component = match i {
                0 => ray_dir.x,
                1 => ray_dir.y,
                2 => ray_dir.z,
                _ => unreachable!(),
            };
            let box_min_component = match i {
                0 => box_min.x,
                1 => box_min.y,
                2 => box_min.z,
                _ => unreachable!(),
            };
            let box_max_component = match i {
                0 => box_max.x,
                1 => box_max.y,
                2 => box_max.z,
                _ => unreachable!(),
            };

            if dir_component.abs() < 1e-6_f32 {
                // Ray is parallel to the slab
                if origin_component < box_min_component || origin_component > box_max_component {
                    return None; // Ray misses the box
                }
            } else {
                // Calculate intersection distances
                let t1 = (box_min_component - origin_component) / dir_component;
                let t2 = (box_max_component - origin_component) / dir_component;

                let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

                t_min = t_min.max(t_near);
                t_max = t_max.min(t_far);

                if t_min > t_max {
                    return None; // No intersection
                }
            }
        }

        // Return the closest intersection distance (if positive)
        if t_min > 0.0_f32 {
            Some(t_min)
        } else if t_max > 0.0_f32 {
            Some(t_max)
        } else {
            None
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
                        let to_player = other_player.position - origin;
                        let distance = to_player.length();

                        // Check if player is within range and closer than any previous hit
                        if distance <= weapon_config.range && distance < hit_result.distance {
                            // Check if ray is pointing towards player (dot product > 0)
                            let to_player_dir = to_player.normalize();
                            let dot = direction.dot(to_player_dir);

                            // Allow some tolerance for near-misses (dot > 0.9 means within ~25 degrees)
                            if dot > 0.9 {
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
                        self.send_message(addr, &damage_msg).await;

                        if died {
                            // Update killer stats
                            if let Some(killer) = self.players.get_mut(shooter_id) {
                                killer.kills += 1;
                            }

                            let death_msg = ServerMessage::PlayerDied {
                                player_id: hit_player_id.clone(),
                                killer_id: Some(shooter_id.clone()),
                            };
                            self.broadcast(&death_msg).await;

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
                self.broadcast(&shot_msg).await;
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
                        log_info("falling back to default spawn point");
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
                    self.broadcast(&respawn_msg).await;

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
                            self.send_message(addr, &error_msg).await;
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

        self.broadcast(&shutdown_msg).await;

        // Give clients a moment to receive the message
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        println!("Shutdown complete.");
    }
}
