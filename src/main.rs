use bevy::prelude::*;
use plugins::*;

mod components;
mod plugins;
mod systems;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldPlugin, PlayerPlugin))
        .run();
}
