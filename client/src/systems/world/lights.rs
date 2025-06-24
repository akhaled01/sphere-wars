use bevy::prelude::*;

pub fn setup_world_lighting(commands: &mut Commands) {
    // Use directional light instead of point light for better performance
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 15000.0,
            shadow_depth_bias: 0.1,
            ..default()
        },
        Transform::from_xyz(50.0, 30.0, 50.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add ambient light for better visibility
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
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
