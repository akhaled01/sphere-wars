use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

pub type MazeGrid = Vec<Vec<bool>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MazeConfig {
    pub seed: u64,
    pub width: usize,
    pub height: usize,
    pub difficulty: String,
}

impl MazeConfig {
    pub fn new(seed: u64, width: usize, height: usize, difficulty: &str) -> Self {
        Self {
            seed,
            width,
            height,
            difficulty: difficulty.to_string(),
        }
    }
}

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
pub fn generate_maze_from_config(config: &MazeConfig) -> MazeGrid {
    generate_maze_with_seed(config.width, config.height, &config.difficulty, config.seed)
}

pub fn generate_maze_with_seed(width: usize, height: usize, difficulty: &str, seed: u64) -> MazeGrid {
    let count = width * height;
    let mut nodes = vec![MazeNode::new(); count];
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

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
            // Backtrack
            if let Some(last_position) = move_nodes.pop() {
                position = last_position;
            } else {
                break;
            }
        }
    }

    // Apply difficulty-based modifications
    match difficulty {
        "easy" => {
            add_extra_connections(&mut nodes, width, height, 0.2, &mut rng);
            remove_dead_ends(&mut nodes, width, height, 0.3, &mut rng);
        }
        "medium" => {
            add_extra_connections(&mut nodes, width, height, 0.1, &mut rng);
            remove_dead_ends(&mut nodes, width, height, 0.15, &mut rng);
        }
        "hard" => {
            // No modifications for hard difficulty
        }
        _ => {
            // Default to medium
            add_extra_connections(&mut nodes, width, height, 0.1, &mut rng);
            remove_dead_ends(&mut nodes, width, height, 0.15, &mut rng);
        }
    }

    // Convert to simple grid format
    nodes_to_simple_grid(&nodes, width, height)
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

// Add extra connections between nodes
fn add_extra_connections(nodes: &mut [MazeNode], width: usize, height: usize, chance: f32, rng: &mut impl Rng) {
    for i in 0..nodes.len() {
        let neighbors = get_neighbors(i, width, height);
        let mut directions = Vec::new();

        // Find unvisited neighbors
        if let Some(north_pos) = neighbors.north {
            if nodes[north_pos].visited {
                directions.push(('n', north_pos));
            }
        }
        if let Some(south_pos) = neighbors.south {
            if nodes[south_pos].visited {
                directions.push(('s', south_pos));
            }
        }
        if let Some(west_pos) = neighbors.west {
            if nodes[west_pos].visited {
                directions.push(('w', west_pos));
            }
        }
        if let Some(east_pos) = neighbors.east {
            if nodes[east_pos].visited {
                directions.push(('e', east_pos));
            }
        }

        for (direction, next_position) in directions {
            if rng.random_bool(chance.into()) {
                // Remove walls between current and next position
                match direction {
                    'n' => {
                        nodes[i].north = false;
                        nodes[next_position].south = false;
                    }
                    's' => {
                        nodes[i].south = false;
                        nodes[next_position].north = false;
                    }
                    'w' => {
                        nodes[i].west = false;
                        nodes[next_position].east = false;
                    }
                    'e' => {
                        nodes[i].east = false;
                        nodes[next_position].west = false;
                    }
                    _ => {}
                }
            }
        }
    }
}

// Remove dead ends
fn remove_dead_ends(nodes: &mut [MazeNode], width: usize, height: usize, chance: f32, rng: &mut impl Rng) {
    for i in 0..nodes.len() {
        let neighbors = get_neighbors(i, width, height);
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

        if directions.len() == 1 && rng.random_bool(chance.into()) {
            // Remove wall to dead end
            let (direction, next_position) = directions[0];
            match direction {
                'n' => {
                    nodes[i].north = true;
                    nodes[next_position].south = true;
                }
                's' => {
                    nodes[i].south = true;
                    nodes[next_position].north = true;
                }
                'w' => {
                    nodes[i].west = true;
                    nodes[next_position].east = true;
                }
                'e' => {
                    nodes[i].east = true;
                    nodes[next_position].west = true;
                }
                _ => {}
            }
        }
    }
}

// Convert nodes to simple grid format
fn nodes_to_simple_grid(nodes: &[MazeNode], width: usize, height: usize) -> MazeGrid {
    // Create a larger grid: each node becomes a 3x3 area with walls between
    let grid_width = width * 2 + 1;
    let grid_height = height * 2 + 1;
    let mut grid = vec![vec![true; grid_width]; grid_height]; // Start with all walls

    for i in 0..nodes.len() {
        let node_x = i % width;
        let node_y = i / width;
        
        // Convert to grid coordinates (each node is at odd positions)
        let grid_x = node_x * 2 + 1;
        let grid_y = node_y * 2 + 1;

        // Create passage at node position
        grid[grid_y][grid_x] = false;

        // Create passages between nodes based on removed walls
        if !nodes[i].north && node_y > 0 {
            // Create passage going north
            grid[grid_y - 1][grid_x] = false;
        }
        if !nodes[i].south && node_y < height - 1 {
            // Create passage going south
            grid[grid_y + 1][grid_x] = false;
        }
        if !nodes[i].west && node_x > 0 {
            // Create passage going west
            grid[grid_y][grid_x - 1] = false;
        }
        if !nodes[i].east && node_x < width - 1 {
            // Create passage going east
            grid[grid_y][grid_x + 1] = false;
        }
    }

    grid
}