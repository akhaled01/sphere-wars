use bevy::prelude::*;
use crate::components::{
    player::{FollowCamera, Player},
    projectile::Weapon,
};
use crate::NetworkClient;

pub fn hitscan_shooting(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_q: Query<&mut Weapon, With<Player>>,
    camera_q: Query<&Transform, (With<FollowCamera>, Without<Player>)>,
    network: Res<NetworkClient>,
    time: Res<Time>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
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
        let ray_origin = camera_transform.translation;
        let ray_direction = camera_transform.forward().as_vec3();

        // Send shoot message to server for authoritative hitscan
        network.send_shoot(ray_origin, ray_direction);
        println!("Shot fired! Origin: {:?}, Direction: {:?}", ray_origin, ray_direction);
    }
}
