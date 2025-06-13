use bevy::{prelude::*, window::CursorGrabMode};

use crate::player::{Player, RotateOnLoad};

pub fn track_scene_instances(mut commands: Commands, player_query: Query<Entity, Added<Player>>) {
    for entity in player_query.iter() {
        // Mark the entity to be checked for children once loaded
        commands.entity(entity).insert(RotateOnLoad);
    }
}

pub fn rotate_player_child_once_ready(
    mut commands: Commands,
    scene_query: Query<(Entity, &Children), With<RotateOnLoad>>,
    mut transform_query: Query<&mut Transform>,
) {
    for (entity, children) in scene_query.iter() {
        for child in children.iter() {
            if let Ok(mut transform) = transform_query.get_mut(child) {
                // Rotate the child 90 degrees around Y
                transform.rotation = Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2);

                // Optional: remove the RotateOnLoad tag so it runs once
                commands.entity(entity).remove::<RotateOnLoad>();
                break;
            }
        }
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
