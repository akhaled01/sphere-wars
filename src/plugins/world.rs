use bevy::prelude::*;

use crate::systems::world::{
    maze::{initialize_shared_maze, render_maze},
    setup::setup_world,
    ui::{crosshairs::*, fps::*, minimap::*},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                initialize_shared_maze,
                render_maze,
                setup_world,
                setup_fps_counter,
                setup_minimap,
                setup_crosshairs,
            )
                .chain(),
        );
        app.add_systems(
            Update,
            (
                update_fps_counter,
                update_minimap,
                update_player_position_on_minimap,
            ),
        );
    }
}
