use bevy::prelude::*;
use crate::components::network::GameData;
use crate::NetworkClient;
use std::time::{Duration, Instant};

#[derive(Resource)]
pub struct RespawnTimer {
    pub death_time: Option<Instant>,
    pub respawn_delay: Duration,
}

impl Default for RespawnTimer {
    fn default() -> Self {
        Self {
            death_time: None,
            respawn_delay: Duration::from_secs(3),
        }
    }
}

pub fn handle_respawn_timer(
    mut respawn_timer: ResMut<RespawnTimer>,
    game_data: Res<GameData>,
    network: Res<NetworkClient>,
) {
    // Check if local player is dead and start timer
    if let Some(my_id) = &game_data.my_id {
        if let Some(my_player) = game_data.players.get(my_id) {
            if !my_player.is_alive && respawn_timer.death_time.is_none() {
                // Player just died, start respawn timer
                respawn_timer.death_time = Some(Instant::now());
                println!("Player died. Respawning in {} seconds...", respawn_timer.respawn_delay.as_secs());
            } else if my_player.is_alive && respawn_timer.death_time.is_some() {
                // Player respawned, clear timer
                respawn_timer.death_time = None;
            }
        }
    }

    // Check if respawn timer has expired and send respawn request
    if let Some(death_time) = respawn_timer.death_time {
        if Instant::now().duration_since(death_time) >= respawn_timer.respawn_delay {
            // Send respawn request to server
            network.send_respawn();
            println!("Sending respawn request to server...");
            // Don't clear the timer here - it will be cleared when we receive PlayerRespawned message
        }
    }
}
