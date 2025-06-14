use bevy::color::palettes::basic::GRAY;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use crate::components::world::Collidable;

type MazeGrid = Vec<Vec<bool>>;

fn generate_maze(width: usize, height: usize, _difficulty: f32) -> MazeGrid {
    let mut grid = vec![vec![true; width]; height];

    fn carve(x: usize, y: usize, grid: &mut MazeGrid, rng: &mut ThreadRng) {
        let dirs = &mut [
            (1isize, 0isize),
            (-1isize, 0isize),
            (0isize, 1isize),
            (0isize, -1isize),
        ];
        dirs.shuffle(rng);

        for &(dx, dy) in dirs.iter() {
            let nx = (x as isize + dx * 5) as usize; 
            let ny = (y as isize + dy * 5) as usize;

            if ny < grid.len() && nx < grid[0].len() && grid[ny][nx] {
                grid[ny][nx] = false;
                
                for step in 1..5 {
                    let mid_x = (x as isize + dx * step) as usize;
                    let mid_y = (y as isize + dy * step) as usize;
                    if mid_y < grid.len() && mid_x < grid[0].len() {
                        grid[mid_y][mid_x] = false;
                        if dx == 0 { 
                            if mid_x > 0 { grid[mid_y][mid_x - 1] = false; }
                            if mid_x + 1 < grid[0].len() { grid[mid_y][mid_x + 1] = false; }
                        } else { 
                            if mid_y > 0 { grid[mid_y - 1][mid_x] = false; }
                            if mid_y + 1 < grid.len() { grid[mid_y + 1][mid_x] = false; }
                        }
                    }
                }
                
                carve(nx, ny, grid, rng);
            }
        }
    }

    let mut rng = ThreadRng::default();
    for y in 1..4 {
        for x in 1..4 {
            if y < grid.len() && x < grid[0].len() {
                grid[y][x] = false;
            }
        }
    }
    carve(2, 2, &mut grid, &mut rng);
    grid
}

pub fn render_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let maze = generate_maze(100, 100, 0.5);

    for (y, row) in maze.iter().enumerate() {
        for (x, &is_wall) in row.iter().enumerate() {
            let world_x = x as f32;
            let world_z = y as f32;

            if is_wall {
                commands.spawn((
                    Mesh3d(meshes.add(Mesh::from(Cuboid {
                        half_size: Vec3::new(0.5, 10.0, 0.5),
                    }))),
                    MeshMaterial3d(materials.add(Color::from(GRAY))),
                    Transform::from_xyz(world_x, 0.5, world_z),
                    Collidable,
                ));
            }
        }
    }
}
