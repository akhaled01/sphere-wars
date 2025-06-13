use bevy::prelude::*;

use crate::player::systems::{camera::*, physics::*, setup::*};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                track_scene_instances,
                rotate_player_child_once_ready,
                move_player,
                handle_jumping,
                apply_gravity,
                follow_camera_system,
                camera_look_sys,
                grab_mouse,
            ),
        );
    }
}
