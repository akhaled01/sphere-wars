use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::components::player::{CameraController, FollowCamera, Player, RotateOnLoad};

pub fn follow_camera_system(
    player_query: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let player_pos = player_transform.translation;
            // Position camera higher and slightly forward to avoid seeing tank parts
            // Use the player's forward direction to offset the camera properly
            let forward = player_transform.forward();
            let camera_offset = Vec3::new(0.0, 3.5, 0.0) + forward * 1.5; // Higher and forward
            camera_transform.translation = player_pos + camera_offset;
        }
    }
}

pub fn camera_look_sys(
    mut motion_evr: EventReader<MouseMotion>,
    mut query_set: ParamSet<(
        Query<(&mut Transform, &mut CameraController), With<FollowCamera>>,
        Query<&mut Transform, With<RotateOnLoad>>,
    )>,
) {
    let mut delta = Vec2::ZERO;
    for ev in motion_evr.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    // Calculate rotations using camera parameters
    let (yaw_rotation, pitch_rotation) = {
        let mut camera_query = query_set.p0();
        if let Some((_, mut controller)) = camera_query.iter_mut().next() {
            // Update yaw and pitch
            controller.yaw -= delta.x * controller.sensitivity;
            controller.pitch -= delta.y * controller.sensitivity;
            
            // Clamp pitch to prevent over-rotation (looking too far up/down)
            controller.pitch = controller.pitch.clamp(-89.0, 89.0);
            
            let yaw_radians = controller.yaw.to_radians();
            let pitch_radians = controller.pitch.to_radians();
            
            let yaw_quat = Quat::from_axis_angle(Vec3::Y, yaw_radians);
            let pitch_quat = Quat::from_axis_angle(Vec3::X, pitch_radians);
            
            (yaw_quat, pitch_quat)
        } else {
            return;
        }
    };

    // Apply both yaw and pitch to camera
    if let Some((mut transform, _)) = query_set.p0().iter_mut().next() {
        transform.rotation = yaw_rotation * pitch_rotation;
    }

    // Apply only yaw rotation to player/tank (no pitch)
    for mut transform in query_set.p1().iter_mut() {
        transform.rotation = yaw_rotation;
    }
}
