use serde::{Deserialize, Serialize};
use glam::{Vec3, Quat};
use std::collections::HashMap;
use rand::{rng, Rng};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameState {
    WaitingForPlayers,
    GameStarted,
    GameOver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub health: f32,
    pub max_health: f32,
    pub kills: u32,
    pub deaths: u32,
    pub is_alive: bool,
    pub last_shot_time: f64,
    pub death_time: Option<f64>,
    pub last_damage_time: Option<f64>,
    pub last_damage_by: Option<u32>,
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            position: Vec3::new(96.0, 2.5, 96.0), // Default spawn position
            rotation: Quat::IDENTITY,
            health: 100.0,
            max_health: 100.0,
            kills: 0,
            deaths: 0,
            is_alive: true,
            last_shot_time: 0.0,
            death_time: None,
            last_damage_time: None,
            last_damage_by: None,
        }
    }

    pub fn respawn(&mut self) {
        self.health = self.max_health;
        self.is_alive = true;
        self.position = Vec3::new(96.0, 2.5, 96.0); // Reset to spawn
        self.death_time = None;
        self.last_damage_time = None;
        self.last_damage_by = None;
    }

    pub fn take_damage(&mut self, damage: f32) -> bool {
        if !self.is_alive {
            return false;
        }
        
        self.health -= damage;
        if self.health <= 0.0 {
            self.health = 0.0;
            self.is_alive = false;
            self.deaths += 1;
            return true; // Player died
        }
        false
    }
}



// Client to Server Messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    JoinGame { player_name: String },
    LeaveGame,
    PlayerMove { position: Vec3, rotation: Quat },
    PlayerShoot { origin: Vec3, direction: Vec3 },
    Respawn,
}

// Server to Client Messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    GameJoined { player_id: String },
    GameState { 
        players: HashMap<String, Player>,
        state: GameState,
        max_players: u32,
        min_players: u32,
        game_start_time: Option<f64>
    },
    PlayerUpdate { player: Player },
    PlayerJoined { player: Player },
    PlayerLeft { player_id: String },
    PlayerKilled { killer_id: String, victim_id: String },
    PlayerRespawned { player_id: String, position: Vec3 },
    PlayerMoved { player_id: String, position: Vec3, rotation: Quat },
    PlayerShot { player_id: String, origin: Vec3, direction: Vec3, hit_result: HitscanResult },
    PlayerDied { player_id: String, killer_id: Option<String> },
    PlayerDamaged { player_id: String, damage: f32, health: f32, damage_by: String },
    ShotFired { shooter_id: String, hit_position: Vec3, hit_player: Option<String> },
    GameStarted { maze: MazeGrid },
    Error { message: String },
}

// Weapon configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponConfig {
    pub damage: f32,
    pub range: f32,
    pub fire_rate: f32, // shots per second
}

impl Default for WeaponConfig {
    fn default() -> Self {
        Self {
            damage: 25.0,
            range: 100.0,
            fire_rate: 4.0,
        }
    }
}

// Hitscan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitscanResult {
    pub hit: bool,
    pub hit_position: Option<Vec3>,
    pub hit_player_id: Option<String>,
    pub distance: f32,
}

type MazeGrid = Vec<Vec<bool>>;

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
pub fn generate_maze(width: usize, height: usize, difficulty: &str) -> MazeGrid {
    let count = width * height;
    let mut nodes = vec![MazeNode::new(); count];
    let mut rng = rng();

    // Difficulty-based parameters
    let (dead_end_removal_chance, extra_connections_chance, corridor_width_multiplier) = match difficulty {
        "easy" => (0.3, 0.2, 1.5), // Remove 30% of dead ends, 20% chance for extra connections, wider corridors
        "medium" => (0.15, 0.1, 1.0), // Remove 15% of dead ends, 10% chance for extra connections, normal corridors
        "hard" => (0.0, 0.0, 0.8), // No dead end removal, no extra connections, narrower corridors
        _ => (0.15, 0.1, 1.0), // Default to medium
    };

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

    // Post-processing based on difficulty
    if extra_connections_chance > 0.0 {
        add_extra_connections(&mut nodes, width, height, extra_connections_chance, &mut rng);
    }

    if dead_end_removal_chance > 0.0 {
        remove_dead_ends(&mut nodes, width, height, dead_end_removal_chance, &mut rng);
    }

    // Convert nodes to matrix representation with difficulty-based corridor width
    nodes_to_matrix(&nodes, width, height, corridor_width_multiplier)
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

// Convert nodes to matrix representation with difficulty-based corridor width
fn nodes_to_matrix(nodes: &[MazeNode], width: usize, height: usize, corridor_width_multiplier: f32) -> MazeGrid {
    // Create wider corridors: each node becomes a 2x2 area, with 1-tile walls between
    let matrix_width = (width as f32 * corridor_width_multiplier * 3.0) as usize + 1; // 2 tiles per node + 1 wall between
    let matrix_height = (height as f32 * corridor_width_multiplier * 3.0) as usize + 1;
    let mut matrix = vec![vec![true; matrix_width]; matrix_height];

    for i in 0..nodes.len() {
        let node_x = i % width;
        let node_y = i / width;
        let matrix_x = (node_x as f32 * corridor_width_multiplier * 3.0) as usize + 1; // Start position for 2x2 corridor
        let matrix_y = (node_y as f32 * corridor_width_multiplier * 3.0) as usize + 1;

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