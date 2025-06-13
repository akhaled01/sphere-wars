use bevy::prelude::*;

use crate::player::{Grounded, Player, Velocity};

const PLAYER_SPEED: f32 = 3.0;
const GRAVITY: f32 = -9.8;
const JUMP_FORCE: f32 = 5.5;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in player_q.iter_mut() {
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
        }

        let move_delta = direction * PLAYER_SPEED * time.delta_secs();
        transform.translation += Vec3::new(move_delta.x, 0.0, move_delta.z);

        // Apply vertical velocity
        transform.translation.y += velocity.linear_velocity.y * time.delta_secs();
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
