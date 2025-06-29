use super::lights::setup_maze_lighting;
use crate::components::player::Player;
use crate::components::world::{Collidable, SharedMaze};
use bevy::math::Vec3;
use bevy::prelude::*;

#[derive(Component)]
pub struct MazeWall;

#[derive(Component)]
pub struct MazeFloor;

#[derive(Resource)]
pub struct MazeMaterials {
    wall: Handle<StandardMaterial>,
    floor: Handle<StandardMaterial>,
}

pub fn setup_maze_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.8, 0.9),
        metallic: 0.0,
        perceptual_roughness: 0.6,
        reflectance: 0.6,
        unlit: false,
        ..default()
    });

    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.3, 0.2),
        metallic: 0.0,
        perceptual_roughness: 0.9,
        reflectance: 0.1,
        unlit: false,
        ..default()
    });

    commands.insert_resource(MazeMaterials {
        wall: wall_material,
        floor: floor_material,
    });
}

/// System that renders the maze on startup
pub fn setup_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    maze_materials: Res<MazeMaterials>,
    maze_data: Res<SharedMaze>,
) {
    // Scale and offset values for maze positioning - no gaps between tiles
    let tile_size = 4.0; // Size of each maze tile
    let maze_width = maze_data.grid[0].len() as f32 * tile_size;
    let maze_height = maze_data.grid.len() as f32 * tile_size;
    let maze_offset_x = -maze_width / 2.0; // Center maze horizontally
    let maze_offset_z = -maze_height / 2.0; // Center maze vertically

    setup_maze_lighting(&mut commands);

    // Spawn floor tiles and walls in a single pass
    for (y, row) in maze_data.grid.iter().enumerate() {
        for (x, &is_wall) in row.iter().enumerate() {
            let pos = Vec3::new(
                x as f32 * tile_size + maze_offset_x + tile_size / 2.0,
                0.0,
                y as f32 * tile_size + maze_offset_z + tile_size / 2.0,
            );

            // Floor tile at every position (both walls and passages need floors)
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(tile_size, 0.2, tile_size))),
                MeshMaterial3d(maze_materials.floor.clone()),
                Transform::from_translation(Vec3::new(pos.x, 0.1, pos.z)),
                MazeFloor,
            ));

            // Wall at positions where is_wall is true
            if is_wall {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(tile_size, 8.0, tile_size))),
                    MeshMaterial3d(maze_materials.wall.clone()),
                    Transform::from_translation(Vec3::new(pos.x, 4.0, pos.z)),
                    MazeWall,
                    Collidable,
                ));
            }
        }
    }
}

/// System that positions the player in a valid maze location after maze is generated
pub fn position_player_in_maze(
    mut player_query: Query<&mut Transform, With<Player>>,
    maze_data: Res<SharedMaze>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        // Find the first valid (non-wall) position in the maze
        let tile_size = 4.0;
        let maze_width = maze_data.grid[0].len() as f32 * tile_size;
        let maze_height = maze_data.grid.len() as f32 * tile_size;
        let maze_offset_x = -maze_width / 2.0;
        let maze_offset_z = -maze_height / 2.0;

        // Look for the first passage (false) in the maze
        for (y, row) in maze_data.grid.iter().enumerate() {
            for (x, &is_wall) in row.iter().enumerate() {
                if !is_wall {
                    // Found a passage, position player here
                    let pos = Vec3::new(
                        x as f32 * tile_size + maze_offset_x + tile_size / 2.0,
                        2.5, // Keep player above ground
                        y as f32 * tile_size + maze_offset_z + tile_size / 2.0,
                    );
                    player_transform.translation = pos;
                    return;
                }
            }
        }
    }
}
