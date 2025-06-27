use bevy::prelude::*;
use crate::components::{
    player::{FollowCamera, Player},
    projectile::Weapon,
    network::GameData,
};
use crate::NetworkClient;

pub fn hitscan_shooting(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_q: Query<&mut Weapon, With<Player>>,
    camera_q: Query<&Transform, (With<FollowCamera>, Without<Player>)>,
    network: Res<NetworkClient>,
    time: Res<Time>,
    game_data: Res<GameData>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Check if local player is alive before allowing shooting
    if let Some(my_id) = &game_data.my_id {
        if let Some(my_player) = game_data.players.get(my_id) {
            if !my_player.is_alive {
                return; // Don't allow shooting when dead
            }
        } else {
            return; // Player not found in game data
        }
    } else {
        return; // No player ID assigned yet
    }

    let camera_transform = if let Ok(transform) = camera_q.single() {
        transform
    } else {
        return;
    };

    for mut weapon in player_q.iter_mut() {
        let current_time = time.elapsed_secs();
        
        // Check fire rate
        if current_time - weapon.last_shot_time < 1.0 / weapon.fire_rate {
            continue;
        }
        
        weapon.last_shot_time = current_time;

        // Calculate ray from camera position in camera's forward direction
        // The ray should start slightly forward from camera to avoid self-intersection
        let camera_forward = camera_transform.forward().normalize();
        let ray_origin = camera_transform.translation + camera_forward * 0.1; // Offset slightly forward
        let ray_direction = camera_forward;

        // Send shoot message to server for authoritative hitscan
        network.send_shoot(ray_origin, ray_direction);
        println!("Shot fired! Origin: {:?}, Direction: {:?}", ray_origin, ray_direction);
    }
}
