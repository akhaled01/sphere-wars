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
    asset_server: Res<AssetServer>,
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
        Mesh3d(meshes.add(Plane3d::default().mesh().size(400.0, 400.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
        Transform::from_xyz(200.0, 0.0, 200.0),
    ));

    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/tank.glb"))),
        Transform::from_xyz(96.0, 2.5, 96.0), // Spawn in first corridor with new maze positioning
        Player,
        Velocity::default(),
        Grounded(true),
        RotateOnLoad,
        Weapon::default(),
    ));

    lights::setup_world_lighting(&mut commands);
}
