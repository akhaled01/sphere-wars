use crate::components::world::FpsCounter;
use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

pub fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        Text::new("FPS: "),
        TextFont {
            font: asset_server.load("SpaceMono-Regular.ttf"),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
    )).with_child((
        TextSpan::new("0.00"),
        TextFont {
            font: asset_server.load("SpaceMono-Regular.ttf"),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        FpsCounter,
    ));
}

pub fn update_fps_counter(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut TextSpan, With<FpsCounter>>) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }
}