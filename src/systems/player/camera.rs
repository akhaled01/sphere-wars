use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::components::player::{CameraController, FollowCamera, Player, RotateOnLoad};

pub fn follow_camera_system(
    player_query: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let player_pos = player_transform.translation;
            // Position camera at player's head height
            camera_transform.translation = player_pos + Vec3::Y * 2.0;
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

    // Calculate rotation using camera parameters
    let yaw_rotation = {
        let mut camera_query = query_set.p0();
        if let Some((_, mut controller)) = camera_query.iter_mut().next() {
            controller.yaw -= delta.x * controller.sensitivity;
            let yaw_radians = controller.yaw.to_radians();
            Quat::from_axis_angle(Vec3::Y, yaw_radians)
        } else {
            return;
        }
    };

    // Apply same rotation to both camera and tank
    if let Some((mut transform, _)) = query_set.p0().iter_mut().next() {
        transform.rotation = yaw_rotation;
    }

    // Apply rotation to player/tank
    for mut transform in query_set.p1().iter_mut() {
        transform.rotation = yaw_rotation;
    }
}

