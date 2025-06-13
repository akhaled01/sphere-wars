use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::player::{CameraController, FollowCamera, Player};

pub fn follow_camera_system(
    player_query: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let player_pos = player_transform.translation;

            let forward = player_transform.forward();
            let offset = forward * 3.0 + Vec3::Y * 1.8;

            camera_transform.translation = player_pos + offset;
        }
    }
}

pub fn camera_look_sys(
    mut motion_evr: EventReader<MouseMotion>,
    mut camera_q: Query<(&mut Transform, &mut CameraController), With<FollowCamera>>,
) {
    let mut delta = Vec2::ZERO;
    for ev in motion_evr.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    for (mut transform, mut controller) in camera_q.iter_mut() {
        controller.yaw -= delta.x * controller.sensitivity;

        let yaw_radians = controller.yaw.to_radians();
        let yaw_rotation = Quat::from_axis_angle(Vec3::Y, yaw_radians);

        transform.rotation = yaw_rotation;
    }
}
