use bevy::math::{Quat, Vec3};
use serde::{Deserialize, Serialize};

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
    pub color: [f32; 3], // RGB color values (0.0 to 1.0)
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
            color: [1.0, 1.0, 1.0], // Default white color
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
