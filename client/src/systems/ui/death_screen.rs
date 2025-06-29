use bevy::prelude::*;
use crate::components::network::GameData;
use crate::network::NetworkClient;

#[derive(Resource, Default)]
pub struct DeathState {
    pub is_dead: bool,
}

#[derive(Component)]
pub struct DeathScreenUI;

#[derive(Component)]
pub struct DeathText;

pub fn setup_death_screen(mut commands: Commands) {
    // Create death screen overlay (initially hidden)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // Dark overlay
            Visibility::Hidden, // Start hidden
            DeathScreenUI,
        ))
        .with_children(|parent| {
            // "YOU DIED" text
            parent.spawn((
                Text::new("YOU DIED"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.2)), // Red text
                DeathText,
            ));
            
            // Instructions text
            parent.spawn((
                Text::new("Press R to respawn"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)), // Gray text
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(100.0),
                    ..default()
                },
            ));
        });
}

pub fn handle_death_screen(
    death_state: Res<DeathState>,
    mut death_screen_query: Query<&mut Visibility, With<DeathScreenUI>>,
) {
    for mut visibility in death_screen_query.iter_mut() {
        if death_state.is_dead {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn update_death_state(
    game_data: Res<GameData>,
    mut death_state: ResMut<DeathState>,
) {
    // Update death state based on local player status
    if let Some(my_id) = &game_data.my_id {
        if let Some(my_player) = game_data.players.get(my_id) {
            death_state.is_dead = !my_player.is_alive;
        }
    }
}

pub fn handle_manual_respawn(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    death_state: Res<DeathState>,
    network: Res<NetworkClient>,
    _game_data: Res<GameData>,
) {
    // Only allow respawn when dead and R key is pressed
    if death_state.is_dead && keyboard_input.just_pressed(KeyCode::KeyR) {
        network.send_respawn();
        println!("Manual respawn requested - sending to server...");
    }
}

pub fn disable_movement_when_dead(
    death_state: Res<DeathState>,
    game_data: Res<GameData>,
    mut player_query: Query<&mut Transform, With<crate::components::player::Player>>,
) {
    if death_state.is_dead {
        // Keep player at current position when dead
        // Movement systems will be blocked by checking death state
        for mut _transform in player_query.iter_mut() {
            // Transform is frozen by not updating it
        }
    }
}
