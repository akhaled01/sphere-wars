use bevy::color::palettes::basic::SILVER;
use bevy::prelude::*;

use crate::player::{CameraController, FollowCamera, Grounded, Player, RotateOnLoad, Velocity};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

// render ground, lights, and camera
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        FollowCamera,
        CameraController {
            yaw: 0.0,
            sensitivity: 0.1,
        },
    ));

    // ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(500.0, 500.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // player tank
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/tank.glb"))),
        Transform::from_xyz(2.0, 4.0, 2.0),
        Player,
        Velocity::default(),
        Grounded(true),
        RotateOnLoad,
    ));
}
