use bevy::prelude::*;

pub fn setup_world_lighting(commands: &mut Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(50.0, 30.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn setup_maze_lighting(commands: &mut Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));
}
