use crate::components::world::Crosshairs;
use bevy::prelude::*;

pub fn setup_crosshairs(mut commands: Commands) {
    // Create crosshairs container in the center of the screen
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                // Center the crosshairs by offsetting by half their size
                margin: UiRect {
                    left: Val::Px(-10.0),
                    top: Val::Px(-10.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Crosshairs,
        ))
        .with_children(|parent| {
            // Horizontal line
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(6.0),
                    top: Val::Px(9.0),
                    width: Val::Px(8.0),
                    height: Val::Px(2.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
            ));

            // Vertical line
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(9.0),
                    top: Val::Px(6.0),
                    width: Val::Px(2.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
            ));

            // Center dot
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(9.0),
                    top: Val::Px(9.0),
                    width: Val::Px(2.0),
                    height: Val::Px(2.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.9)),
            ));
        });
}
