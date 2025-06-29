#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Weapon {
    pub fire_rate: f32, // shots per second
    pub last_shot_time: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            fire_rate: 4.0, // 2 shots per second
            last_shot_time: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self { current: 100.0 }
    }
}

#[derive(Component)]
pub struct HitEffect {
    pub timer: Timer,
}

impl Default for HitEffect {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}
