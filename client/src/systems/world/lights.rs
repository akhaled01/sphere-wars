use bevy::prelude::*;

pub fn setup_world_lighting(commands: &mut Commands) {
    // Single main light with shadows
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 20_000_000.,
            range: 200.0,
            shadow_depth_bias: 0.1,
            ..default()
        },
        Transform::from_xyz(50.0, 30.0, 50.0),
    ));

    // Brighter ambient light to compensate for fewer point lights
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
        ..default()
    });
}

pub fn setup_maze_lighting(commands: &mut Commands) {
    // Single directional light for the maze
    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: false,
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));
}
