use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Velocity {
    pub linear_velocity: Vec3,
}

#[derive(Component)]
pub struct Grounded(pub bool);

// Camera Components

#[derive(Component)]
pub struct FollowCamera;

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,   // Horizontal rotation (around Y axis)
    pub pitch: f32, // Vertical rotation (around X axis)
    pub sensitivity: f32,
}

#[derive(Component)]
pub struct RotateOnLoad;
