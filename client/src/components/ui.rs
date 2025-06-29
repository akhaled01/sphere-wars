use bevy::prelude::*;

#[derive(Component)]
pub struct MessageDisplay {
    pub timer: Timer,
}

impl MessageDisplay {
    pub fn new(duration_secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_secs, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct MessageContainer;
