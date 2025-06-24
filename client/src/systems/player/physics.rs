use bevy::prelude::*;

use crate::components::{
    player::{FollowCamera, Grounded, Player, Velocity},
    world::Collidable,
};
use crate::network::NetworkClient;

const PLAYER_SPEED: f32 = 15.0;
const GRAVITY: f32 = -9.8;
const JUMP_FORCE: f32 = 5.5;
const PLAYER_RADIUS: f32 = 0.5;
const WALL_SIZE: f32 = 3.0;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<FollowCamera>)>,
    camera_q: Query<&Transform, (With<FollowCamera>, Without<Player>)>,
    collidable_q: Query<&Transform, (With<Collidable>, Without<Player>)>,
    time: Res<Time>,
    network: Res<NetworkClient>,
) {
    let camera_transform = if let Ok(transform) = camera_q.single() {
        transform
    } else {
        return;
    };

    for (mut transform, _velocity) in player_q.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            let camera_rotation =
                Quat::from_rotation_y(camera_transform.rotation.to_euler(EulerRot::YXZ).0);
            direction = camera_rotation * direction;
        }

        let move_delta = direction * PLAYER_SPEED * time.delta_secs();

        let current_pos = transform.translation;

        let new_x = current_pos + Vec3::new(move_delta.x, 0.0, 0.0);
        if !is_position_blocked(new_x, &collidable_q) {
            transform.translation.x = new_x.x;
        }

        let new_z = transform.translation + Vec3::new(0.0, 0.0, move_delta.z);
        if !is_position_blocked(new_z, &collidable_q) {
            transform.translation.z = new_z.z;
        }

        if move_delta.length_squared() > 0.0 {
            network.send_move(transform.translation, transform.rotation);
        }
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Grounded), With<Player>>,
) {
    for (mut velocity, mut transform, mut grounded) in query.iter_mut() {
        if transform.translation.y <= 2.0 {
            transform.translation.y = 2.0;
            velocity.linear_velocity.y = 0.0;
            grounded.0 = true;
        } else {
            velocity.linear_velocity.y += GRAVITY * time.delta_secs();
            grounded.0 = false;
        }

        transform.translation.y += velocity.linear_velocity.y * time.delta_secs();
    }
}

pub fn handle_jumping(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Grounded), With<Player>>,
) {
    for (mut velocity, grounded) in query.iter_mut() {
        if grounded.0 && keyboard_input.just_pressed(KeyCode::Space) {
            velocity.linear_velocity.y = JUMP_FORCE;
        }
    }
}

pub fn handle_collisions(
    mut player_q: Query<&mut Transform, With<Player>>,
    collidable_q: Query<&Transform, (With<Collidable>, Without<Player>)>,
) {
    for mut player_transform in player_q.iter_mut() {
        if is_position_blocked(player_transform.translation, &collidable_q) {
            let pos = player_transform.translation;
            let offsets = [
                Vec3::new(0.1, 0.0, 0.0),
                Vec3::new(-0.1, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.1),
                Vec3::new(0.0, 0.0, -0.1),
            ];

            for offset in offsets.iter() {
                let test_pos = pos + *offset;
                if !is_position_blocked(test_pos, &collidable_q) {
                    player_transform.translation = test_pos;
                    break;
                }
            }
        }
    }
}

fn get_grid_cell(pos: Vec3) -> (i32, i32) {
    // Use larger grid cells (8x8) to check fewer cells
    let cell_size = 8.0;
    let x = (pos.x / cell_size).floor() as i32;
    let z = (pos.z / cell_size).floor() as i32;
    (x, z)
}

fn is_position_blocked(
    player_pos: Vec3,
    collidable_q: &Query<&Transform, (With<Collidable>, Without<Player>)>,
) -> bool {
    let (grid_x, grid_z) = get_grid_cell(player_pos);
    
    // Only check nearby grid cells
    for dx in -1..=1 {
        for dz in -1..=1 {
            // let check_x = (grid_x + dx) as f32 * 8.0;
            // let check_z = (grid_z + dz) as f32 * 8.0;
            
            // Only check walls that are in this grid cell
            for transform in collidable_q.iter() {
                let wall_pos = transform.translation;
                let (wall_grid_x, wall_grid_z) = get_grid_cell(wall_pos);
                
                if wall_grid_x == grid_x + dx && wall_grid_z == grid_z + dz {
                    let diff = player_pos - wall_pos;
                    let distance_x = diff.x.abs();
                    let distance_z = diff.z.abs();
                    
                    if distance_x < (PLAYER_RADIUS + WALL_SIZE) && 
                       distance_z < (PLAYER_RADIUS + WALL_SIZE) {
                        return true;
                    }
                }
            }
        }
    }
    
    false
}
