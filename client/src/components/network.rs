use bevy::prelude::*;
use shared::{GameState, Player};
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct GameData {
    pub my_id: Option<String>,
    pub players: HashMap<String, Player>,
    pub state: Option<GameState>,
    pub max_players: u32,
    pub min_players: u32,
    pub game_start_time: Option<f64>,
}

#[derive(Resource)]
pub struct MazeData {
    pub grid: Vec<Vec<bool>>,
}

// Component to mark the local player
#[derive(Component)]
pub struct LocalPlayer;
