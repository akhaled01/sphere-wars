use bevy::{prelude::*, window::CursorGrabMode};

use crate::components::player::{Player, RotateOnLoad};

pub fn track_scene_instances(mut commands: Commands, player_query: Query<Entity, Added<Player>>) {
    for entity in player_query.iter() {
        // Mark the entity to be checked for children once loaded
        commands.entity(entity).insert(RotateOnLoad);
    }
}

pub fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}
