use bevy::prelude::*;

mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, world::WorldPlugin, player::PlayerPlugin))
        .run();
}
