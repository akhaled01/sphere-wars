use crate::components::world::Collidable;
use crate::components::world::SharedMaze;
use bevy::prelude::*;
use rand::{Rng, rng};

use super::lights::setup_maze_lighting;

pub type MazeGrid = Vec<Vec<bool>>;

// Node representation: visited, north, south, west, east
#[derive(Clone, Debug)]
struct MazeNode {
    visited: bool,
    north: bool,
    south: bool,
    west: bool,
    east: bool,
}

impl MazeNode {
    fn new() -> Self {
        Self {
            visited: false,
            north: true, // true means wall exists
            south: true,
            west: true,
            east: true,
        }
    }
}

// Proper maze generation algorithm based on the JavaScript implementation
pub fn generate_maze(width: usize, height: usize, _difficulty: f32) -> MazeGrid {
    let count = width * height;
    let mut nodes = vec![MazeNode::new(); count];
    let mut rng = rng();

    // Parse maze using depth-first search with backtracking
    let mut move_nodes = Vec::new();
    let mut visited = 0;
    let mut position = rng.random_range(0..nodes.len());

    // Set start node visited
    nodes[position].visited = true;

    while visited < count - 1 {
        let neighbors = get_neighbors(position, width, height);
        let mut directions = Vec::new();

        // Find unvisited neighbors
        if let Some(north_pos) = neighbors.north {
            if !nodes[north_pos].visited {
                directions.push(('n', north_pos));
            }
        }
        if let Some(south_pos) = neighbors.south {
            if !nodes[south_pos].visited {
                directions.push(('s', south_pos));
            }
        }
        if let Some(west_pos) = neighbors.west {
            if !nodes[west_pos].visited {
                directions.push(('w', west_pos));
            }
        }
        if let Some(east_pos) = neighbors.east {
            if !nodes[east_pos].visited {
                directions.push(('e', east_pos));
            }
        }

        if !directions.is_empty() {
            visited += 1;

            if directions.len() > 1 {
                move_nodes.push(position);
            }

            let (direction, next_position) = directions[rng.random_range(0..directions.len())];

            // Remove walls between current and next position
            match direction {
                'n' => {
                    nodes[position].north = false;
                    nodes[next_position].south = false;
                }
                's' => {
                    nodes[position].south = false;
                    nodes[next_position].north = false;
                }
                'w' => {
                    nodes[position].west = false;
                    nodes[next_position].east = false;
                }
                'e' => {
                    nodes[position].east = false;
                    nodes[next_position].west = false;
                }
                _ => {}
            }

            position = next_position;
            nodes[position].visited = true;
        } else {
            if move_nodes.is_empty() {
                break;
            }
            position = move_nodes.pop().unwrap();
        }
    }

    // Convert nodes to matrix representation
    nodes_to_matrix(&nodes, width, height)
}

struct Neighbors {
    north: Option<usize>,
    south: Option<usize>,
    west: Option<usize>,
    east: Option<usize>,
}

fn get_neighbors(pos: usize, width: usize, height: usize) -> Neighbors {
    let total_size = width * height;

    Neighbors {
        north: if pos >= width {
            Some(pos - width)
        } else {
            None
        },
        south: if pos + width < total_size {
            Some(pos + width)
        } else {
            None
        },
        west: if pos > 0 && pos % width != 0 {
            Some(pos - 1)
        } else {
            None
        },
        east: if (pos + 1) % width != 0 {
            Some(pos + 1)
        } else {
            None
        },
    }
}

fn nodes_to_matrix(nodes: &[MazeNode], width: usize, height: usize) -> MazeGrid {
    // Create wider corridors: each node becomes a 2x2 area, with 1-tile walls between
    let matrix_width = (width * 3) + 1; // 2 tiles per node + 1 wall between
    let matrix_height = (height * 3) + 1;
    let mut matrix = vec![vec![true; matrix_width]; matrix_height];

    for i in 0..nodes.len() {
        let node_x = i % width;
        let node_y = i / width;
        let matrix_x = (node_x * 3) + 1; // Start position for 2x2 corridor
        let matrix_y = (node_y * 3) + 1;

        // Create 2x2 corridor area for each node
        matrix[matrix_y][matrix_x] = false; // Top-left
        matrix[matrix_y][matrix_x + 1] = false; // Top-right
        matrix[matrix_y + 1][matrix_x] = false; // Bottom-left
        matrix[matrix_y + 1][matrix_x + 1] = false; // Bottom-right

        // Create 2-tile wide passages between nodes
        if !nodes[i].north && matrix_y > 1 {
            // Create 2-wide passage going north
            matrix[matrix_y - 1][matrix_x] = false;
            matrix[matrix_y - 1][matrix_x + 1] = false;
        }
        if !nodes[i].south && matrix_y < matrix_height - 2 {
            // Create 2-wide passage going south
            matrix[matrix_y + 2][matrix_x] = false;
            matrix[matrix_y + 2][matrix_x + 1] = false;
        }
        if !nodes[i].west && matrix_x > 1 {
            // Create 2-wide passage going west
            matrix[matrix_y][matrix_x - 1] = false;
            matrix[matrix_y + 1][matrix_x - 1] = false;
        }
        if !nodes[i].east && matrix_x < matrix_width - 2 {
            // Create 2-wide passage going east
            matrix[matrix_y][matrix_x + 2] = false;
            matrix[matrix_y + 1][matrix_x + 2] = false;
        }
    }

    matrix
}

pub fn initialize_shared_maze(mut commands: Commands) {
    // Generate the maze once and store it as a resource
    let width = 12;
    let height = 12;
    let grid = generate_maze(width, height, 1.0);
    
    commands.insert_resource(SharedMaze {
        grid,
    });
}

pub fn render_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    shared_maze: Res<SharedMaze>,
) {
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

    for (y, row) in shared_maze.grid.iter().enumerate() {
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
