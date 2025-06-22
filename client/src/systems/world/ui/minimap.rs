use crate::components::world::{Minimap, MinimapInitialized, MinimapPixel, PlayerDot, RemotePlayerDot, SharedMaze};
use crate::components::network::{GameData, RemotePlayer};
use bevy::prelude::*;
use std::collections::HashMap;

const MINIMAP_SIZE: f32 = 200.0;
const MINIMAP_MARGIN: f32 = 20.0;
const TILE_SIZE: f32 = 4.0; // Match the maze rendering tile size

pub fn setup_minimap(mut commands: Commands) {
    // Create minimap container in bottom right corner
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(MINIMAP_MARGIN),
            right: Val::Px(MINIMAP_MARGIN),
            width: Val::Px(MINIMAP_SIZE),
            height: Val::Px(MINIMAP_SIZE),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BorderColor(Color::WHITE),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        Minimap,
    ));
}

pub fn update_minimap(
    mut commands: Commands,
    minimap_query: Query<Entity, (With<Minimap>, Without<MinimapInitialized>)>,
    player_query: Query<&Transform, (With<Camera>, Without<MinimapPixel>, Without<PlayerDot>)>,
    shared_maze: Res<SharedMaze>,
) {
    // Only initialize the minimap once
    if let Ok(minimap_entity) = minimap_query.single() {
        // Use the shared maze instead of generating a new one
        let maze = &shared_maze.grid;
        let maze_size = maze.len();
        let pixel_size = (MINIMAP_SIZE / maze_size as f32).max(2.0);

        println!(
            "Initializing minimap: maze_size={}, pixel_size={}",
            maze_size, pixel_size
        );

        // Create minimap pixel
        for (y, row) in maze.iter().enumerate() {
            for (x, &is_wall) in row.iter().enumerate() {
                let color = if is_wall {
                    Color::srgb(0.9, 0.9, 1.0) // Bright white-blue for walls
                } else {
                    Color::srgb(0.1, 0.6, 0.1) // Bright green for corridor
                };

                commands.entity(minimap_entity).with_children(|parent| {
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(x as f32 * pixel_size),
                            top: Val::Px(y as f32 * pixel_size),
                            width: Val::Px(pixel_size.max(1.0)),
                            height: Val::Px(pixel_size.max(1.0)),
                            ..default()
                        },
                        BackgroundColor(color),
                        MinimapPixel,
                    ));
                });
            }
        }

        // Add initial player dot
        if let Ok(player_transform) = player_query.single() {
            let player_pos = player_transform.translation;

            // Convert 3D world coordinates to minimap coordinates
            // The maze is centered, so we need to account for the offset
            let maze_width = maze[0].len() as f32 * TILE_SIZE;
            let maze_height = maze.len() as f32 * TILE_SIZE;
            let maze_offset_x = -maze_width / 2.0;
            let maze_offset_z = -maze_height / 2.0;
            
            let minimap_x = ((player_pos.x - maze_offset_x) / TILE_SIZE) * pixel_size;
            let minimap_z = ((player_pos.z - maze_offset_z) / TILE_SIZE) * pixel_size;

            // Clamp to minimap bounds
            let minimap_x = minimap_x.clamp(0.0, MINIMAP_SIZE - 8.0);
            let minimap_z = minimap_z.clamp(0.0, MINIMAP_SIZE - 8.0);

            commands.entity(minimap_entity).with_children(|parent| {
                parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(minimap_x),
                        top: Val::Px(minimap_z),
                        width: Val::Px(8.0),
                        height: Val::Px(8.0),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.0, 0.0)), // Bright red for player
                    BorderColor(Color::WHITE),
                    PlayerDot,
                ));
            });
        }

        // Mark minimap as initialized
        commands.entity(minimap_entity).insert(MinimapInitialized);
    }
}

pub fn update_player_position_on_minimap(
    mut dots: ParamSet<(
        Query<&mut Node, With<PlayerDot>>,
        Query<(&mut Node, &RemotePlayerDot)>
    )>,
    player_query: Query<&Transform, (With<Camera>, Without<PlayerDot>)>,
    remote_query: Query<(&Transform, &RemotePlayer)>,
    shared_maze: Res<SharedMaze>,
    game_data: Res<GameData>,
    mut commands: Commands,
    minimap_query: Query<Entity, With<Minimap>>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut player_dot_node) = dots.p0().single_mut() {
        let player_pos = player_transform.translation;

        // Convert 3D world coordinates to minimap coordinates
        let maze = &shared_maze.grid;
        let maze_size = maze.len();
        let pixel_size = (MINIMAP_SIZE / maze_size as f32).max(2.0);
        let maze_width = maze[0].len() as f32 * TILE_SIZE;
        let maze_height = maze.len() as f32 * TILE_SIZE;
        let maze_offset_x = -maze_width / 2.0;
        let maze_offset_z = -maze_height / 2.0;
        
        let minimap_x = ((player_pos.x - maze_offset_x) / TILE_SIZE) * pixel_size;
        let minimap_z = ((player_pos.z - maze_offset_z) / TILE_SIZE) * pixel_size;

        // Clamp to minimap bounds
        let minimap_x = minimap_x.clamp(0.0, MINIMAP_SIZE - 8.0);
        let minimap_z = minimap_z.clamp(0.0, MINIMAP_SIZE - 8.0);

        // Update player dot position
        player_dot_node.left = Val::Px(minimap_x);
        player_dot_node.top = Val::Px(minimap_z);
        }
    }

    // Track existing remote dots
    let mut existing_dots = HashMap::new();
    for (_, remote_dot) in dots.p1().iter() {
        existing_dots.insert(remote_dot.player_id.clone(), true);
    }

    // Update or create remote player dots
    if let Ok(minimap_entity) = minimap_query.single() {
        for (transform, remote_player) in remote_query.iter() {
            let player_pos = transform.translation;
            
            // Convert world coordinates to minimap coordinates
            let maze = &shared_maze.grid;
            let maze_size = maze.len();
            let pixel_size = (MINIMAP_SIZE / maze_size as f32).max(2.0);
            let maze_width = maze[0].len() as f32 * TILE_SIZE;
            let maze_height = maze.len() as f32 * TILE_SIZE;
            let maze_offset_x = -maze_width / 2.0;
            let maze_offset_z = -maze_height / 2.0;
            
            let minimap_x = ((player_pos.x - maze_offset_x) / TILE_SIZE) * pixel_size;
            let minimap_z = ((player_pos.z - maze_offset_z) / TILE_SIZE) * pixel_size;

            // Clamp to minimap bounds
            let minimap_x = minimap_x.clamp(0.0, MINIMAP_SIZE - 8.0);
            let minimap_z = minimap_z.clamp(0.0, MINIMAP_SIZE - 8.0);

            if !existing_dots.contains_key(&remote_player.id) {
                // Create new remote player dot
                commands.entity(minimap_entity).with_children(|parent| {
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(minimap_x),
                            top: Val::Px(minimap_z),
                            width: Val::Px(8.0),
                            height: Val::Px(8.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.0, 0.0, 1.0)), // Blue for remote players
                        BorderColor(Color::WHITE),
                        RemotePlayerDot {
                            player_id: remote_player.id.clone(),
                        },
                    ));
                });
            } else {
                // Update existing remote player dot
                for (mut node, dot) in dots.p1().iter_mut() {
                    if dot.player_id == remote_player.id {
                        node.left = Val::Px(minimap_x);
                        node.top = Val::Px(minimap_z);
                        break;
                    }
                }
            }
        }

        // Remove dots for disconnected players
        for (entity, (_, dot)) in dots.p1().iter().enumerate() {
            if !game_data.players.contains_key(&dot.player_id) {
                commands.entity(Entity::from_raw(entity as u32)).despawn();
            }
        }
    }
}
