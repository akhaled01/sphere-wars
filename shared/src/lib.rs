use serde::{Deserialize, Serialize};
use glam::{Vec3, Quat};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameState {
    WaitingForPlayers,
    GameStarted,
    GameOver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub health: f32,
    pub max_health: f32,
    pub kills: u32,
    pub deaths: u32,
    pub is_alive: bool,
    pub last_shot_time: f64,
    pub death_time: Option<f64>,
    pub last_damage_time: Option<f64>,
    pub last_damage_by: Option<u32>,
}

impl Player {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            position: Vec3::new(96.0, 2.5, 96.0), // Default spawn position
            rotation: Quat::IDENTITY,
            health: 100.0,
            max_health: 100.0,
            kills: 0,
            deaths: 0,
            is_alive: true,
            last_shot_time: 0.0,
            death_time: None,
            last_damage_time: None,
            last_damage_by: None,
        }
    }

    pub fn respawn(&mut self) {
        self.health = self.max_health;
        self.is_alive = true;
        self.position = Vec3::new(96.0, 2.5, 96.0); // Reset to spawn
        self.death_time = None;
        self.last_damage_time = None;
        self.last_damage_by = None;
    }

    pub fn take_damage(&mut self, damage: f32) -> bool {
        if !self.is_alive {
            return false;
        }
        
        self.health -= damage;
        if self.health <= 0.0 {
            self.health = 0.0;
            self.is_alive = false;
            self.deaths += 1;
            return true; // Player died
        }
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRoom {
    pub id: String,
    pub players: HashMap<u32, Player>,
    pub state: GameState,
    pub max_players: u32,
    pub min_players: u32,
    pub game_start_time: Option<f64>,
    pub maze_seed: u64,
}

impl GameRoom {
    pub fn new(id: String) -> Self {
        Self {
            id,
            players: HashMap::new(),
            state: GameState::WaitingForPlayers,
            max_players: 8,
            min_players: 2,
            game_start_time: None,
            maze_seed: rand::random(),
        }
    }

    pub fn can_start_game(&self) -> bool {
        self.players.len() >= self.min_players as usize && 
        matches!(self.state, GameState::WaitingForPlayers)
    }

    pub fn add_player(&mut self, player: Player) -> bool {
        if self.players.len() >= self.max_players as usize {
            return false;
        }
        self.players.insert(player.id, player);
        true
    }

    pub fn remove_player(&mut self, player_id: u32) -> Option<Player> {
        self.players.remove(&player_id)
    }

    pub fn get_alive_players(&self) -> Vec<&Player> {
        self.players.values().filter(|p| p.is_alive).collect()
    }
}

// Client to Server Messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    JoinGame { player_name: String },
    LeaveGame,
    PlayerMove { position: Vec3, rotation: Quat },
    PlayerShoot { origin: Vec3, direction: Vec3 },
    Respawn,
}

// Server to Client Messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    GameJoined { player_id: u32, room_id: String },
    GameState { room: GameRoom },
    PlayerUpdate { player: Player },
    PlayerJoined { player: Player },
    PlayerLeft { player_id: u32 },
    PlayerKilled { killer_id: u32, victim_id: u32 },
    PlayerRespawned { player_id: u32, position: Vec3 },
    PlayerMoved { player_id: u32, position: Vec3, rotation: Quat },
    PlayerShot { player_id: u32, origin: Vec3, direction: Vec3, hit_result: HitscanResult },
    PlayerDied { player_id: u32, killer_id: Option<u32> },
    PlayerDamaged { player_id: u32, damage: f32, health: f32, damage_by: u32 },
    ShotFired { shooter_id: u32, hit_position: Vec3, hit_player: Option<u32> },
    GameStarted,
    Error { message: String },
}

// Weapon configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponConfig {
    pub damage: f32,
    pub range: f32,
    pub fire_rate: f32, // shots per second
}

impl Default for WeaponConfig {
    fn default() -> Self {
        Self {
            damage: 25.0,
            range: 100.0,
            fire_rate: 4.0,
        }
    }
}

// Hitscan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitscanResult {
    pub hit: bool,
    pub hit_position: Option<Vec3>,
    pub hit_player_id: Option<u32>,
    pub distance: f32,
}

// Maze generation (simplified for server)
pub fn generate_maze_seed() -> u64 {
    rand::random()
}
