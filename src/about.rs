use bevy::prelude::*;
use super::{
    GameState,
    Ship,
    cleanup
};
use crate::menu::{AnimateRotation, TextSize, rotate_text, text_from_str};

pub struct AboutPlugin;

#[derive(Component)]
struct OnAboutScreen;

impl Plugin for AboutPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::About), (cleanup::<Ship>, about_setup))
            .add_systems(
                Update,
                check_input.run_if(in_state(GameState::About))
            )
            .add_systems(OnExit(GameState::About), cleanup::<OnAboutScreen>);
    }
}

fn about_setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        text_from_str(&assets, "About the game", Color::YELLOW, TextSize::Large),
        OnAboutScreen
    ));
    commands.spawn((
        text_from_str(&assets, "
Flappy Space
<https://github.com/hiimsergey/flappyspace>

Built with Bevy Engine
<https://bevyengine.org>


GPL-3.0 License
            ", Color::YELLOW, TextSize::Normal),
        OnAboutScreen
    ));
    commands.spawn((
        text_from_str(&assets, "Press Enter to exit", Color::YELLOW, TextSize::Lower),
        OnAboutScreen, AnimateRotation
    ));
}

fn check_input(
    mut game_state: ResMut<NextState<GameState>>,
    key: Res<Input<KeyCode>>
) {
    if key.just_pressed(KeyCode::Return) {
        game_state.set(GameState::Menu);
    }
}
