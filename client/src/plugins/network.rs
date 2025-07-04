use crate::{
    components::{
        network::{GameData, LocalPlayer, RemotePlayer},
        player::{Grounded, RotateOnLoad, Velocity},
        projectile::{HitEffect, Weapon},
        ui::MessageContainer,
        world::SharedMaze,
    },
    net::NetworkClient,
    plugins::ui::show_message,
    systems::ui::death_screen::DamageOverlayState,
};
use bevy::prelude::*;
use shared::{MazeConfig, Player, ServerMessage, generate_maze_from_config};
use std::collections::HashMap;

pub struct NetworkPlugin;

#[derive(Resource)]
pub struct LocalPlayerResource {
    pub player: Option<Player>,
    pub entity: Option<Entity>,
}

impl Default for LocalPlayerResource {
    fn default() -> Self {
        Self {
            player: None,
            entity: None,
        }
    }
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>()
            .init_resource::<LocalPlayerResource>()
            .add_systems(
                Update,
                (
                    handle_network_messages,
                    sync_player_transforms,
                    sync_remote_players,
                    cleanup_hit_effects,
                ),
            );
    }
}

// System to receive and handle network messages
fn handle_network_messages(
    mut game_data: ResMut<GameData>,
    mut local_player: ResMut<LocalPlayerResource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_transforms: Query<&mut Transform, (With<LocalPlayer>, Without<RemotePlayer>)>,
    mut remote_transforms: Query<&mut Transform, (With<RemotePlayer>, Without<LocalPlayer>)>,
    mut damage_overlay: ResMut<DamageOverlayState>,
    message_container: Query<Entity, With<MessageContainer>>,
    network: Res<NetworkClient>,
    maze: Option<Res<SharedMaze>>,
) {
    if let Some(message) = network.try_recv() {
        match message {
            ServerMessage::GameJoined { player_id } => {
                game_data.my_id = Some(player_id.clone());
                if local_player.player.is_none() {
                    // Get player data from game_data to access server-assigned color
                    let player_color = if let Some(player) = game_data.players.get(&player_id) {
                        Color::srgb(player.color[0], player.color[1], player.color[2])
                    } else {
                        Color::srgb(0.8, 0.2, 0.2) // Fallback color
                    };

                    local_player.player = Some(Player::new(
                        player_id.clone(),
                        network.player_name().to_string(),
                    ));

                    let sphere = Mesh3d(meshes.add(Sphere::new(1.5)));
                    let sphere_material = MeshMaterial3d(materials.add(player_color));

                    let entity = commands
                        .spawn((
                            sphere,
                            sphere_material,
                            Transform::from_xyz(0.0, 1.0, 0.0),
                            LocalPlayer,
                            Velocity::default(),
                            Grounded(true),
                            RotateOnLoad,
                            Weapon::default(),
                        ))
                        .id();
                    local_player.entity = Some(entity);
                    game_data.player_entities.insert(player_id, entity);
                }
            }
            ServerMessage::GameState {
                players,
                state,
                game_start_time,
            } => {
                // First update game data and players
                game_data.state = Some(state);
                game_data.game_start_time = game_start_time;
                game_data.players = players.clone();

                println!("GameState received: {} players total", players.len());
                println!("My ID: {:?}", game_data.my_id);
                println!(
                    "Existing entities: {:?}",
                    game_data.player_entities.keys().collect::<Vec<_>>()
                );

                // Then spawn entities for new players
                for (player_id, player) in players.iter() {
                    if Some(player_id.as_str()) == game_data.my_id.as_deref()
                        || game_data.player_entities.contains_key(player_id)
                    {
                        println!(
                            "Skipping player {}: is_local={}, has_entity={}",
                            player_id,
                            Some(player_id.as_str()) == game_data.my_id.as_deref(),
                            game_data.player_entities.contains_key(player_id)
                        );
                        continue;
                    }

                    println!(
                        "Spawning remote player: {} at {:?}",
                        player_id, player.position
                    );

                    let player_color =
                        Color::srgb(player.color[0], player.color[1], player.color[2]);
                    let entity = commands
                        .spawn((
                            Mesh3d(meshes.add(Sphere::new(1.5))),
                            MeshMaterial3d(materials.add(player_color)),
                            Transform::from_translation(player.position)
                                .with_rotation(player.rotation),
                            RemotePlayer {
                                id: player_id.clone(),
                            },
                            Velocity::default(),
                            Grounded(true),
                            RotateOnLoad,
                            Weapon::default(),
                        ))
                        .id();
                    game_data.player_entities.insert(player_id.clone(), entity);
                }
            }
            ServerMessage::PlayerUpdate { player } => {
                // Update local player if it's us
                if let Some(my_player) = &mut local_player.player {
                    if my_player.id == player.id {
                        *my_player = player.clone();
                    }
                }
                // Update game data
                if let Some(existing) = game_data.players.get_mut(&player.id) {
                    *existing = player.clone();
                }
            }
            ServerMessage::PlayerMoved {
                player_id,
                position,
                rotation,
            } => {
                // Update the player's position in game data
                if let Some(player) = game_data.players.get_mut(&player_id) {
                    player.position = position;
                    player.rotation = rotation;
                }
            }
            ServerMessage::PlayerJoined { player } => {
                game_data.players.insert(player.id.clone(), player.clone());
                // Spawn remote player entity if it's not us
                if Some(player.id.as_str()) != game_data.my_id.as_deref() {
                    // Adjust position to be on the ground
                    let mut position = player.position;
                    position.y = 1.0; // Set tank height to be on ground

                    let player_color =
                        Color::srgb(player.color[0], player.color[1], player.color[2]);
                    let entity = commands
                        .spawn((
                            Mesh3d(meshes.add(Sphere::new(1.5))),
                            MeshMaterial3d(materials.add(player_color)),
                            Transform::from_translation(position).with_rotation(player.rotation),
                            RemotePlayer {
                                id: player.id.clone(),
                            },
                            Velocity::default(),
                            Grounded(true),
                            RotateOnLoad,
                            Weapon::default(),
                        ))
                        .id();
                    game_data.player_entities.insert(player.id, entity);
                }
            }
            ServerMessage::PlayerLeft { player_id } => {
                game_data.players.remove(&player_id);
                // Only remove and despawn if it's not the local player
                if Some(player_id.as_str()) != game_data.my_id.as_deref() {
                    if let Some(entity) = game_data.player_entities.remove(&player_id) {
                        commands.entity(entity).despawn();
                    }
                }
            }
            ServerMessage::GameStarted {
                seed,
                width,
                height,
                difficulty,
            } => {
                let config = MazeConfig {
                    seed,
                    width,
                    height,
                    difficulty,
                };
                let maze_data = generate_maze_from_config(&config);
                commands.insert_resource(SharedMaze {
                    grid: maze_data.grid,
                    spawn_points: maze_data.spawn_points,
                });
            }
            ServerMessage::NameAlreadyTaken => {
                error!("Name already taken");
                std::process::exit(1);
            }
            ServerMessage::PlayerShot {
                player_id,
                origin,
                direction,
                hit_result,
            } => {
                // Spawn visual hit effect based on hit result
                if hit_result.hit {
                    if let Some(hit_pos) = hit_result.hit_position {
                        spawn_shot_effect(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            hit_pos,
                            true,
                        );
                    }
                } else {
                    // Show miss effect at max range
                    let miss_pos = origin + direction * hit_result.distance;
                    spawn_shot_effect(&mut commands, &mut meshes, &mut materials, miss_pos, false);
                }
                println!("Player {} fired shot. Hit: {}", player_id, hit_result.hit);
            }
            ServerMessage::PlayerDamaged {
                player_id,
                damage,
                health,
                damage_by,
            } => {
                // Update player health
                if let Some(player) = game_data.players.get_mut(&player_id) {
                    player.health = health;
                }

                // Trigger damage overlay if this is the local player
                if Some(player_id.as_str()) == game_data.my_id.as_deref() {
                    damage_overlay.trigger_damage_flash();
                }

                println!(
                    "Player {} took {} damage from {}. Health: {}",
                    player_id, damage, damage_by, health
                );
            }
            ServerMessage::PlayerDied {
                player_id,
                killer_id,
            } => {
                // Update player state
                if let Some(player) = game_data.players.get_mut(&player_id) {
                    player.is_alive = false;
                    player.health = 0.0;
                    player.deaths += 1;
                }

                // Hide dead players by scaling to zero instead of despawning
                if let Some(entity) = game_data.player_entities.get(&player_id) {
                    // For remote players, use remote_transforms query
                    if Some(&player_id) != game_data.my_id.as_ref() {
                        if let Ok(mut transform) = remote_transforms.get_mut(*entity) {
                            transform.scale = Vec3::ZERO;
                        }
                    } else {
                        // For local player, use player_transforms query
                        if let Ok(mut transform) = player_transforms.get_mut(*entity) {
                            transform.scale = Vec3::ZERO;
                        }
                    }
                }

                // Update killer stats
                if let Some(killer_id) = killer_id {
                    if let Some(killer) = game_data.players.get_mut(&killer_id) {
                        killer.kills += 1;
                    }
                    println!("Player {} was killed by {}", player_id, killer_id);
                } else {
                    println!("Player {} died", player_id);
                }
            }
            ServerMessage::PlayerRespawned {
                player_id,
                position,
            } => {
                // Store the final position and rotation for entity update
                let mut final_position = position;
                let mut final_rotation = Quat::IDENTITY;

                // Calculate spawn index before mutable borrow
                let spawn_index = if let Some(maze) = maze.as_ref() {
                    if !maze.spawn_points.is_empty() {
                        Some((player_id.len() + game_data.players.len()) % maze.spawn_points.len())
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Update player state
                if let Some(player) = game_data.players.get_mut(&player_id) {
                    player.is_alive = true;
                    player.health = player.max_health;

                    // Use client-side spawn point selection if we have maze data
                    if let Some(maze) = maze.as_ref() {
                        // Select a spawn point from the maze
                        let spawn_points = &maze.spawn_points;
                        if !spawn_points.is_empty() {
                            if let Some(index) = spawn_index {
                                let spawn_point = &spawn_points[index];
                                player.position = spawn_point.position;
                                player.rotation = spawn_point.rotation;
                                final_position = spawn_point.position;
                                final_rotation = spawn_point.rotation;
                            }
                        } else {
                            // Fallback to server position if no spawn points
                            player.position = position;
                            final_position = position;
                        }
                    } else {
                        // Use server position if no maze data
                        player.position = position;
                        final_position = position;
                    }
                }

                // Recreate entity if it doesn't exist (was despawned on death)
                if !game_data.player_entities.contains_key(&player_id) {
                    // Check if this is the local player
                    if Some(player_id.as_str()) == game_data.my_id.as_deref() {
                        // Recreate local player entity
                        let player_color =
                            if let Some(player_data) = game_data.players.get(&player_id) {
                                Color::srgb(
                                    player_data.color[0],
                                    player_data.color[1],
                                    player_data.color[2],
                                )
                            } else {
                                Color::srgb(0.8, 0.2, 0.2) // Fallback red
                            };
                        let entity = commands
                            .spawn((
                                Mesh3d(meshes.add(Sphere::new(1.5))),
                                MeshMaterial3d(materials.add(player_color)),
                                Transform::from_translation(final_position)
                                    .with_rotation(final_rotation),
                                LocalPlayer,
                                Velocity::default(),
                                Grounded(true),
                                RotateOnLoad,
                                Weapon::default(),
                            ))
                            .id();
                        local_player.entity = Some(entity);
                        game_data.player_entities.insert(player_id.clone(), entity);
                    } else {
                        // Recreate remote player entity
                        let player_color =
                            if let Some(player_data) = game_data.players.get(&player_id) {
                                Color::srgb(
                                    player_data.color[0],
                                    player_data.color[1],
                                    player_data.color[2],
                                )
                            } else {
                                Color::srgb(0.2, 0.2, 0.8) // Fallback blue
                            };
                        let entity = commands
                            .spawn((
                                Mesh3d(meshes.add(Sphere::new(1.5))),
                                MeshMaterial3d(materials.add(player_color)),
                                Transform::from_translation(final_position)
                                    .with_rotation(final_rotation),
                                RemotePlayer {
                                    id: player_id.clone(),
                                },
                                Velocity::default(),
                                Grounded(true),
                                RotateOnLoad,
                                Weapon::default(),
                            ))
                            .id();
                        game_data.player_entities.insert(player_id.clone(), entity);
                    }
                }

                // Update entity position if it exists
                if let Some(entity) = game_data.player_entities.get(&player_id) {
                    // Check if it's the local player
                    if let Some(local_player_entity) = local_player.entity {
                        if *entity == local_player_entity {
                            if let Ok(mut transform) = player_transforms.get_mut(*entity) {
                                transform.translation = final_position;
                                transform.rotation = final_rotation;
                                transform.scale = Vec3::ONE; // Restore normal scale on respawn
                            }
                        }
                    }
                    // Check if it's a remote player
                    if let Ok(mut transform) = remote_transforms.get_mut(*entity) {
                        transform.translation = final_position;
                        transform.rotation = final_rotation;
                        transform.scale = Vec3::ONE; // Restore normal scale on respawn
                    }
                }

                println!("Player {} respawned at {:?}", player_id, final_position);
            }
            ServerMessage::GameEnded { reason: _ } => {
                println!("Server is shutting down");
                println!("Game ended. Closing application...");
                std::process::exit(0);
            }
            ServerMessage::Error { message } => {
                // Show error message in UI
                show_message(&mut commands, message, 3.0, &message_container);
            }
            _ => {}
        }
    }
}

// System to sync player transforms with network state
fn sync_player_transforms(
    local_player: Res<LocalPlayerResource>,
    mut query: Query<&mut Transform, With<LocalPlayer>>,
) {
    if let Some(player) = &local_player.player {
        for mut transform in query.iter_mut() {
            transform.translation = player.position;
            transform.rotation = player.rotation;
        }
    }
}

// System to sync remote players
fn sync_remote_players(
    mut game_data: ResMut<GameData>,
    mut commands: Commands,
    mut query: Query<(Entity, &RemotePlayer, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut existing_players: HashMap<String, Entity> = HashMap::new();

    // Update existing remote players
    for (entity, remote_player, mut transform) in query.iter_mut() {
        if let Some(player) = game_data.players.get(&remote_player.id) {
            transform.translation = player.position;
            transform.rotation = player.rotation;
            existing_players.insert(remote_player.id.clone(), entity);
        }
    }

    // Collect players that need to be spawned
    let players_to_spawn: Vec<_> = game_data
        .players
        .iter()
        .filter(|(id, _)| {
            !existing_players.contains_key(*id) && Some(id.as_str()) != game_data.my_id.as_deref()
        })
        .map(|(id, player)| (id.clone(), player.position, player.rotation, player.color))
        .collect();

    // Spawn new remote players
    for (id, position, rotation, color) in players_to_spawn {
        let player_color = Color::srgb(color[0], color[1], color[2]);
        let entity = commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(1.5))),
                MeshMaterial3d(materials.add(player_color)),
                Transform::from_translation(position).with_rotation(rotation),
                RemotePlayer { id: id.clone() },
                Velocity::default(),
                Grounded(true),
                RotateOnLoad,
                Weapon::default(),
            ))
            .id();

        game_data.player_entities.insert(id, entity);
    }
}

fn spawn_shot_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    hit: bool,
) {
    let color = if hit {
        Color::srgb(1.0, 0.5, 0.0) // Orange for hits
    } else {
        Color::srgb(1.0, 1.0, 0.0) // Yellow for misses
    };

    // Spawn a small sphere as hit effect
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            emissive: LinearRgba::new(
                color.to_linear().red,
                color.to_linear().green,
                color.to_linear().blue,
                1.0,
            ),
            ..default()
        })),
        Transform::from_translation(position),
        HitEffect {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
    ));
}

fn cleanup_hit_effects(
    mut commands: Commands,
    mut query: Query<(Entity, &mut HitEffect)>,
    time: Res<Time>,
) {
    for (entity, mut hit_effect) in query.iter_mut() {
        hit_effect.timer.tick(time.delta());
        if hit_effect.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
