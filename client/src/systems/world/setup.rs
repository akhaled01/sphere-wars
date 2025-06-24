use bevy::prelude::*;

use crate::components::{
    player::{CameraController, FollowCamera, Grounded, Player, RotateOnLoad, Velocity},
    projectile::Weapon,
};

use super::lights;

// render ground, lights, and camera
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.0, 6.0, 2.0),
        FollowCamera,
        CameraController {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.1,
        },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1000.0, 1000.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 1.0, 0.0),
        Player,
        Velocity::default(),
        Grounded(true),
        RotateOnLoad,
        Weapon::default(),
    ));

    lights::setup_world_lighting(&mut commands);
}
