use bevy::{color::palettes::css::SILVER, prelude::*};

use crate::components::player::*;

use super::maze; 

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
            sensitivity: 0.1,
        },
    ));

    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(100.0, 100.0)
            ),
        ),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
        Transform::from_xyz(50.0, 0.0, 50.0),
    ));

    // Improved lighting setup
    // Main overhead light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 15_000_000.,
            range: 150.0,
            shadow_depth_bias: 0.1,
            ..default()
        },
        Transform::from_xyz(50.0, 25.0, 50.0),
    ));

    // Additional corner lights for better coverage
    commands.spawn((
        PointLight {
            shadows_enabled: false,
            intensity: 8_000_000.,
            range: 80.0,
            ..default()
        },
        Transform::from_xyz(20.0, 20.0, 20.0),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: false,
            intensity: 8_000_000.,
            range: 80.0,
            ..default()
        },
        Transform::from_xyz(80.0, 20.0, 20.0),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: false,
            intensity: 8_000_000.,
            range: 80.0,
            ..default()
        },
        Transform::from_xyz(20.0, 20.0, 80.0),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: false,
            intensity: 8_000_000.,
            range: 80.0,
            ..default()
        },
        Transform::from_xyz(80.0, 20.0, 80.0),
    ));

    // Ambient light for overall brightness
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
        ..default()
    });

    // player
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/tank.glb"))),
        Transform::from_xyz(50.0, 0.0, 50.0),
        Player,
        Velocity::default(),
        Grounded(true),
        RotateOnLoad,
    ));

    maze::render_maze(commands, meshes, materials);
}
