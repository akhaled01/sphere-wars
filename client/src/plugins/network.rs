use crate::{
    components::{
        network::{GameData, LocalPlayer, RemotePlayer},
        player::{Grounded, RotateOnLoad, Velocity},
        projectile::Weapon,
        world::SharedMaze,
    },
    network::NetworkClient,
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
                ),
            );
    }
}

// System to receive and handle network messages
fn handle_network_messages(
    network: Res<NetworkClient>,
    mut game_data: ResMut<GameData>,
    mut local_player: ResMut<LocalPlayerResource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(message) = network.try_recv() {
        match message {
            ServerMessage::GameJoined { player_id } => {
                game_data.my_id = Some(player_id.clone());
                if local_player.player.is_none() {
                    local_player.player = Some(Player::new(
                        player_id.clone(),
                        network.player_name().to_string(),
                    ));
                    // Spawn local player entity with sphere mesh
                    let sphere = Mesh3d(meshes.add(Sphere::new(0.5)));
                    let sphere_material = MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2)));

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
                max_players,
                min_players,
                game_start_time,
            } => {
                game_data.players = players;
                game_data.state = Some(state);
                game_data.max_players = max_players;
                game_data.min_players = min_players;
                game_data.game_start_time = game_start_time;
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

                    let entity = commands
                        .spawn((
                            Mesh3d(meshes.add(Sphere::new(0.5))),
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
                // Remove the remote player entity if it exists
                if let Some(entity) = game_data.player_entities.remove(&player_id) {
                    commands.entity(entity).despawn();
                }
            }
            ServerMessage::GameStarted {
                seed,
                width,
                height,
                difficulty,
            } => {
                let maze_config = MazeConfig::new(seed, width, height, &difficulty);
                let maze = generate_maze_from_config(&maze_config);
                commands.insert_resource(SharedMaze { grid: maze });
            }
            ServerMessage::NameAlreadyTaken => {
                error!("Name already taken");
                std::process::exit(1);
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
    mut materials: ResMut<Assets<StandardMaterial>>
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
        .map(|(id, player)| (id.clone(), player.position, player.rotation))
        .collect();

    // Spawn new remote players
    for (id, position, rotation) in players_to_spawn {
        let entity = commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(1.5))),
                MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
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
