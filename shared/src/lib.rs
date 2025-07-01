use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

mod maze;
mod messages;
mod player;

pub use maze::*;
pub use messages::*;
pub use player::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameState {
    WaitingForPlayers,
    GameStarted,
    GameOver,
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
            damage: 50.0,
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
    pub hit_player_id: Option<String>,
    pub distance: f32,
}
