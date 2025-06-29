use bevy::prelude::*;

use crate::systems::player::{camera::*, physics::*, setup::*, shooting::*};
use crate::systems::ui::death_screen::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeathState>()
            .add_systems(Startup, setup_death_screen)
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
                    update_death_state,
                    handle_death_screen,
                    handle_manual_respawn,
                    disable_movement_when_dead,
                ),
            );
    }
}
