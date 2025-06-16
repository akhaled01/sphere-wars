use bevy::prelude::*;

use crate::systems::world::maze::MazeGrid;

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

#[derive(Resource)]
pub struct SharedMaze {
    pub grid: MazeGrid,
}

#[derive(Component)]
pub struct Crosshairs;