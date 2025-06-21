
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bevy::math::{Vec3, Quat};
use crate::player::Player;
use crate::{GameState, HitscanResult};

// client to server messages
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
    GameJoined {
        player_id: String,
    },
    GameState {
        players: HashMap<String, Player>,
        state: GameState,
        max_players: u32,
        min_players: u32,
        game_start_time: Option<f64>,
    },
    PlayerUpdate {
        player: Player,
    },
    PlayerJoined {
        player: Player,
    },
    PlayerLeft {
        player_id: String,
    },
    PlayerKilled {
        killer_id: String,
        victim_id: String,
    },
    PlayerRespawned {
        player_id: String,
        position: Vec3,
    },
    PlayerMoved {
        player_id: String,
        position: Vec3,
        rotation: Quat,
    },
    PlayerShot {
        player_id: String,
        origin: Vec3,
        direction: Vec3,
        hit_result: HitscanResult,
    },
    PlayerDied {
        player_id: String,
        killer_id: Option<String>,
    },
    PlayerDamaged {
        player_id: String,
        damage: f32,
        health: f32,
        damage_by: String,
    },
    ShotFired {
        shooter_id: String,
        hit_position: Vec3,
        hit_player: Option<String>,
    },
    GameStarted {
        seed: u64,
        width: usize,
        height: usize,
        difficulty: String,
    },
    Error {
        message: String,
    },
}
