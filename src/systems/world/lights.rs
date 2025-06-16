use bevy::prelude::*;

pub fn setup_world_lighting(commands: &mut Commands) {
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

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
        ..default()
    });
}

pub fn setup_maze_lighting(commands: &mut Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 12000.0,
            shadows_enabled: false,
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));

    commands.spawn((
        PointLight {
            intensity: 50_000_000.0,
            range: 300.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(100.0, 30.0, 100.0),
    ));
}
