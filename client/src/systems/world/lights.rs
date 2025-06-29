use bevy::prelude::*;

pub fn setup_world_lighting(commands: &mut Commands) {
    // Single optimized directional light for better performance
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            illuminance: 3000.0, // Reduced from 10000 for better performance
            color: Color::srgb(1.0, 0.95, 0.9), // Slightly warm light
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    // Add ambient light for better visibility without performance cost
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.4, 0.4, 0.5),
        brightness: 0.3,
        affects_lightmapped_meshes: false,
    });
}

pub fn setup_maze_lighting(_commands: &mut Commands) {
    // Remove duplicate lighting - world lighting is sufficient
    // This eliminates redundant light sources for better performance
}
