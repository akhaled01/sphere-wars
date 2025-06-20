use crate::components::network::MazeData;
use crate::components::world::Collidable;
use bevy::prelude::*;

use super::lights::setup_maze_lighting;

#[derive(Component)]
pub struct MazeWall;

#[derive(Component)]
pub struct MazeFloor;

/// System that renders the maze when received from the server
#[allow(clippy::type_complexity)]
pub fn render_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_data: Res<MazeData>,
    // Query to clean up existing maze entities
    query: Query<Entity, Or<(With<MazeWall>, With<MazeFloor>)>>,
) {
    // Remove existing maze entities
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // Create enhanced wall material with much better visibility
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.8, 0.9),
        metallic: 0.0,
        perceptual_roughness: 0.6,
        reflectance: 0.6,
        unlit: false,
        ..default()
    });

    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.3, 0.2),
        metallic: 0.0,
        perceptual_roughness: 0.9,
        reflectance: 0.1,
        unlit: false,
        ..default()
    });

    let floor_mesh = meshes.add(Mesh::from(Cuboid {
        half_size: Vec3::new(2.0, 0.1, 2.0),
    }));

    setup_maze_lighting(&mut commands);

    let scale_factor = 6.0;
    let maze_offset = 89.0; // Center maze on 400x400 floor (200 - maze_size/2)

    for (y, row) in maze_data.grid.iter().enumerate() {
        for (x, &is_wall) in row.iter().enumerate() {
            if is_wall {
                let wall_position = Vec3::new(
                    x as f32 * scale_factor + maze_offset,
                    9.0,
                    y as f32 * scale_factor + maze_offset,
                );

                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(6.0, 18.0, 6.0))),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_translation(wall_position),
                    Collidable,
                ));
            } else {
                commands.spawn((
                    Mesh3d(floor_mesh.clone()),
                    MeshMaterial3d(floor_material.clone()),
                    Transform::from_xyz(
                        x as f32 * scale_factor + maze_offset,
                        0.1,
                        y as f32 * scale_factor + maze_offset,
                    ),
                ));
            }
        }
    }
}
