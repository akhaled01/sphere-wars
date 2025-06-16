use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::{PresentMode, WindowPlugin, WindowTheme};
use bevy::DefaultPlugins;

pub fn get_init_plugins() -> impl PluginGroup {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mazey".into(),
                name: Some("bevy.app".into()),
                resolution: (1920., 1080.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: true,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        })
        .add(FrameTimeDiagnosticsPlugin::default())
        .add(LogDiagnosticsPlugin::default())
}