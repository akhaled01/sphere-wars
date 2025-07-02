use bevy::prelude::*;

use crate::{
    components::world::SharedMaze,
    systems::world::{
        maze::{position_player_in_maze, setup_maze, setup_maze_materials},
        setup::setup_world,
        ui::{crosshairs::*, fps::*, minimap::*},
    },
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ClearColor(Color::srgb(0.0, 0.8, 0.9)))
        .add_systems(
            Startup,
            (
                setup_maze_materials,
                setup_world,
                setup_fps_counter,
                setup_minimap,
                setup_crosshairs,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                (setup_maze, position_player_in_maze)
                    .chain()
                    .run_if(resource_added::<SharedMaze>),
                update_fps_counter,
                update_minimap,
                update_player_position_on_minimap,
                update_player_dot_colors,
            )
                .run_if(resource_exists::<SharedMaze>),
        );
    }
}
