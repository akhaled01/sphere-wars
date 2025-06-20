use bevy::prelude::*;
use crate::{
    components::network::{GameData, LocalPlayer, MazeData},
    network::NetworkClient,
};
use shared::ServerMessage;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>()
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
    mut commands: Commands,
    // maze_data: Option<ResMut<MazeData>>,
) {
    // Use try_recv() for non-blocking message handling
    if let Some(message) = network.try_recv() {
        match message {
            ServerMessage::GameJoined { player_id } => {
                game_data.my_id = Some(player_id);
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
            ServerMessage::GameStarted { maze } => {
                commands.insert_resource(MazeData { grid: maze });
            }
            // Handle other messages as needed
            _ => {}
        }
    }
}

// System to sync player transforms with network state
fn sync_player_transforms(
    game_data: Res<GameData>,
    mut query: Query<&mut Transform, With<LocalPlayer>>,
) {
    if let Some(my_id) = &game_data.my_id {
        if let Some(my_player) = game_data.players.get(my_id) {
            for mut transform in query.iter_mut() {
                transform.translation = my_player.position;
                transform.rotation = my_player.rotation;
            }
        }
    }
}
