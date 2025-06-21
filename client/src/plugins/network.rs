use bevy::prelude::*;
use crate::{
    components::{network::{GameData, LocalPlayer}, world::SharedMaze},
    network::NetworkClient,
};
use shared::{ServerMessage, MazeConfig, generate_maze_from_config, Player};

pub struct NetworkPlugin;

#[derive(Resource)]
pub struct LocalPlayerResource {
    pub player: Option<Player>,
}

impl Default for LocalPlayerResource {
    fn default() -> Self {
        Self {
            player: None,
        }
    }
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>()
           .init_resource::<LocalPlayerResource>()
           .add_systems(Update, (
                handle_network_messages,
                sync_player_transforms,
            ));
    }
}

// System to receive and handle network messages
fn handle_network_messages(
    network: Res<NetworkClient>,
    mut game_data: ResMut<GameData>,
    mut local_player: ResMut<LocalPlayerResource>,
    mut commands: Commands,
) {
    if let Some(message) = network.try_recv() {
        match message {
            ServerMessage::GameJoined { player_id } => {
                game_data.my_id = Some(player_id.clone());
                if local_player.player.is_none() {
                    local_player.player = Some(Player::new(player_id, network.player_name().to_string()));
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
                    *existing = player;
                }
            }
            ServerMessage::PlayerJoined { player } => {
                game_data.players.insert(player.id.clone(), player);
            }
            ServerMessage::PlayerLeft { player_id } => {
                game_data.players.remove(&player_id);
            }
            ServerMessage::GameStarted { seed, width, height, difficulty } => {
                let maze_config = MazeConfig::new(seed, width, height, &difficulty);
                let maze = generate_maze_from_config(&maze_config);
                commands.insert_resource(SharedMaze { grid: maze });
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
