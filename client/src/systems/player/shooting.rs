use crate::NetworkClient;
use crate::components::{
    player::{FollowCamera, Player},
    projectile::Weapon,
};
use crate::systems::ui::death_screen::DeathState;
use bevy::prelude::*;

pub fn hitscan_shooting(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_q: Query<&mut Weapon, With<Player>>,
    camera_q: Query<&Transform, (With<FollowCamera>, Without<Player>)>,
    network: Res<NetworkClient>,
    time: Res<Time>,
    death_state: Res<DeathState>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Don't allow shooting when dead
    if death_state.is_dead {
        return;
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
        println!(
            "Shot fired! Origin: {:?}, Direction: {:?}",
            ray_origin, ray_direction
        );
    }
}
