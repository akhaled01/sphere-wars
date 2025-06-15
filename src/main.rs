use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use plugins::{PlayerPlugin, WorldPlugin};

mod components;
mod plugins;
mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            WorldPlugin,
            PlayerPlugin,
        ))
        .run();
}
