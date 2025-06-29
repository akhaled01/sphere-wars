use bevy::prelude::*;
use crate::components::ui::{MessageDisplay, MessageContainer};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, (update_message_display, cleanup_expired_messages));
    }
}

fn setup_ui(mut commands: Commands) {
    // Create UI root
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            // Message container
            parent.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    margin: UiRect::all(Val::Px(20.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                MessageContainer,
            ));
        });
}

pub fn show_message(
    commands: &mut Commands,
    message: String,
    duration: f32,
    container_query: &Query<Entity, With<MessageContainer>>,
) {
    if let Ok(container) = container_query.single() {
        commands.entity(container).with_children(|parent| {
            parent.spawn((
                Text::new(message.clone()),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)), // Yellow-orange color
                Node {
                    margin: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // Semi-transparent black background
                MessageDisplay::new(duration),
            ));
        });
    }
}

fn update_message_display(
    mut query: Query<&mut MessageDisplay>,
    time: Res<Time>,
) {
    for mut message in query.iter_mut() {
        message.timer.tick(time.delta());
    }
}

fn cleanup_expired_messages(
    mut commands: Commands,
    query: Query<(Entity, &MessageDisplay)>,
) {
    for (entity, message) in query.iter() {
        if message.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
