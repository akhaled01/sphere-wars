use bevy::prelude::*;

use crate::systems::player::{camera::*, physics::*, setup::*, shooting::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                track_scene_instances,
                move_player,
                handle_jumping,
                apply_gravity,
                follow_camera_system,
                camera_look_sys,
                grab_mouse,
                handle_collisions,
                hitscan_shooting,
            ),
        );
    }
}
