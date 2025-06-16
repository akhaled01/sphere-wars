use crate::components::world::{Minimap, MinimapPixel, PlayerDot};
use crate::systems::world::maze::generate_maze;
use bevy::prelude::*;

const MINIMAP_SIZE: f32 = 200.0;
const MINIMAP_MARGIN: f32 = 20.0;
const MAZE_WIDTH: usize = 12;
const MAZE_HEIGHT: usize = 12;
const SCALE_FACTOR: f32 = 6.0;
const MAZE_OFFSET: f32 = 2.0;

#[derive(Component)]
pub struct MinimapInitialized;

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
) {
    // Only initialize the minimap once
    if let Ok(minimap_entity) = minimap_query.single() {
        // Generate maze using same algorithm as 3D world
        let maze = generate_maze(MAZE_WIDTH, MAZE_HEIGHT, 1.0);
        let maze_size = maze.len();
        let pixel_size = (MINIMAP_SIZE / maze_size as f32).max(2.0);

        println!("Initializing minimap: maze_size={}, pixel_size={}", maze_size, pixel_size);

        // Create minimap pixels
        for (y, row) in maze.iter().enumerate() {
            for (x, &is_wall) in row.iter().enumerate() {
                let color = if is_wall {
                    Color::srgb(0.9, 0.9, 1.0) // Bright white-blue for walls
                } else {
                    Color::srgb(0.1, 0.6, 0.1) // Bright green for corridors
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
            let minimap_x = ((player_pos.x - MAZE_OFFSET) / SCALE_FACTOR) * pixel_size;
            let minimap_z = ((player_pos.z - MAZE_OFFSET) / SCALE_FACTOR) * pixel_size;

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
    mut player_dot_query: Query<&mut Node, With<PlayerDot>>,
    player_query: Query<&Transform, (With<Camera>, Without<PlayerDot>)>,
) {
    if let (Ok(mut player_dot_node), Ok(player_transform)) =
        (player_dot_query.single_mut(), player_query.single())
    {
        let player_pos = player_transform.translation;

        // Convert 3D world coordinates to minimap coordinates
        let maze_size = (MAZE_WIDTH * 3) + 1; // Based on nodes_to_matrix calculation
        let pixel_size = (MINIMAP_SIZE / maze_size as f32).max(2.0);
        let minimap_x = ((player_pos.x - MAZE_OFFSET) / SCALE_FACTOR) * pixel_size;
        let minimap_z = ((player_pos.z - MAZE_OFFSET) / SCALE_FACTOR) * pixel_size;

        // Clamp to minimap bounds
        let minimap_x = minimap_x.clamp(0.0, MINIMAP_SIZE - 8.0);
        let minimap_z = minimap_z.clamp(0.0, MINIMAP_SIZE - 8.0);

        // Update player dot position
        player_dot_node.left = Val::Px(minimap_x);
        player_dot_node.top = Val::Px(minimap_z);
    }
}
