use bevy::prelude::*;
use flappyspace::*;

/// Custom game plugin for all things on the about screen
pub struct AboutPlugin;

/// Tag component for entities added on the about screen
#[derive(Component)]
struct OnAboutScreen;

impl Plugin for AboutPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::About),
                (spawn_about_text, cleanup::<Ship>)
            )
            .add_systems(
                Update,
                (rotate_text, about_input).run_if(in_state(GameState::About))
            )
            .add_systems(OnExit(GameState::About), cleanup::<OnAboutScreen>);
    }
}

/// Prints text seen an about screen: heading, info, input hint
fn spawn_about_text(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        text_from_str(
            &assets,
            "About the game",
            HEADING_FONT_SIZE,
            ABOUT_TEXT_COLOR,
            HEADING_Y
        ), OnAboutScreen
    ));
    commands.spawn((
        text_from_str(
            &assets,
"Flappy Space
<https://github.com/hiimsergey/flappyspace>

Built with Bevy Engine
<https://bevyengine.org>

GPL-3.0 License",
            ABOUT_TEXT_FONT_SIZE,
            ABOUT_TEXT_COLOR,
            0.
        ), OnAboutScreen
    ));
    commands.spawn((
        text_from_str(
            &assets,
            "Press Enter to exit",
            INPUT_HINT_FONT_SIZE,
            ABOUT_TEXT_COLOR,
            INPUT_HINT_ONE_Y
        ), OnAboutScreen, TextRotation
    ));
}

/// Checks for user input (Enter) to launch main menu
fn about_input(
    mut game_state: ResMut<NextState<GameState>>,
    key: Res<Input<KeyCode>>
) {
    if key.just_pressed(KeyCode::Return) {
        game_state.set(GameState::Menu);
    }
}