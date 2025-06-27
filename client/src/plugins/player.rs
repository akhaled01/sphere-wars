use bevy::prelude::*;

use crate::systems::player::{camera::*, physics::*, setup::*, shooting::*, respawn::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RespawnTimer>()
            .add_systems(
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
                    handle_respawn_timer,
                ),
            );
    }
}
