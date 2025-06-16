use bevy::prelude::*;

use plugins::{PlayerPlugin, WorldPlugin};
use systems::utils::get_init_plugins;

mod components;
mod plugins;
mod systems;

fn main() {
    App::new()
        .add_plugins((get_init_plugins(), WorldPlugin, PlayerPlugin))
        .run();
}
