use bevy::prelude::*;

use crate::systems::world::setup::setup_world;
use crate::systems::world::ui::fps::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_world, setup_fps_counter));
        app.add_systems(Update, update_fps_counter);
    }
}
