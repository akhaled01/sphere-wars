use bevy::prelude::*;

use crate::systems::world::setup::setup_world;
use crate::systems::world::ui::fps::*;
use crate::systems::world::ui::minimap::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_world,
                setup_fps_counter,
                setup_minimap,
                update_minimap,
            ),
        );
        app.add_systems(
            Update,
            (update_fps_counter, update_player_position_on_minimap),
        );
    }
}
