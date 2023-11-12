use bevy::prelude::*;
use flappyspace::*;

/// Custom game plugin for all things on the menu screen
pub struct MenuPlugin;

/// Tag component for entites added on the menu screen
#[derive(Component)]
struct OnMenuScreen;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), (spawn_menu_text, spawn_ship))
            .add_systems(
                Update,
                (animate_ship, lobby_input, rotate_text)
                    .run_if(in_state(GameState::Menu))
            )
            .add_systems(OnExit(GameState::Menu), cleanup::<OnMenuScreen>);
    }
}

/// Spawns the game title and keyboard input hints
fn spawn_menu_text(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        text_from_str(
            &assets,
            "Flappy Space",
            HEADING_FONT_SIZE,
            Color::WHITE,
            HEADING_Y
        ), OnMenuScreen
    ));
    commands.spawn((
        text_from_str(
            &assets,
            "Press X to start",
            INPUT_HINT_FONT_SIZE,
            Color::WHITE,
            INPUT_HINT_UPPER_Y
        ), OnMenuScreen, TextRotation
    ));
    commands.spawn((
        text_from_str(
            &assets,
            "Press A for About",
            INPUT_HINT_FONT_SIZE,
            Color::WHITE,
            INPUT_HINT_LOWER_Y
        ), OnMenuScreen, TextRotation
    ));
}
