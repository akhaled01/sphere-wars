use bevy::prelude::*;
use shared::{MazeGrid, SpawnPoint};

#[derive(Component)]
pub struct Collidable;

#[derive(Component)]
pub struct FpsCounter;

#[derive(Component)]
pub struct Minimap;

#[derive(Component)]
pub struct MinimapPixel;

#[derive(Component)]
pub struct MinimapInitialized;

#[derive(Component)]
pub struct PlayerDot;

#[derive(Component)]
pub struct RemotePlayerDot {
    pub player_id: String,
}

#[derive(Resource)]
pub struct SharedMaze {
    pub grid: MazeGrid,
    pub spawn_points: Vec<SpawnPoint>,
}

#[derive(Component)]
pub struct Crosshairs;