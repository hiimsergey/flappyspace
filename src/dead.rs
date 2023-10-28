// TODO
// add dead sound
// add the system
// add death message
    // if crashed -> "You crashed!"
    // if out of map -> "You got lost in space!"
// TODO order - std, bevy, mine
// TODO think about using: use crate::Component etc.
use bevy::prelude::*;
use super::{
	GameState,
    Rock,
	game::Scoreboard,
	cleanup
};
use crate::menu;

pub struct DeadPlugin;

// Tag component for text spawned on the Game Over screen
#[derive(Component)]
struct OnDeadScreen;

impl Plugin for DeadPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(GameState::Dead), game_over)
			.add_systems(
                Update, menu::menu_action.run_if(in_state(GameState::Dead))
			)
			.add_systems(OnExit(GameState::Dead), (
                cleanup::<Rock>,
				cleanup::<Scoreboard>,
                cleanup::<OnDeadScreen>
            ));
	}
}

fn game_over(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.spawn((
        menu::text_from_str(&assets, "Game Over", Color::RED, menu::TextSize::GameOver),
        OnDeadScreen
    ));
    commands.spawn((
        menu::text_from_str(&assets, "Press Enter to restart", Color::RED, menu::TextSize::Low),
        OnDeadScreen, menu::AnimateRotation
    ));
    commands.spawn((
        menu::text_from_str(&assets, "Press A for About", Color::RED, menu::TextSize::Lower),
        OnDeadScreen, menu::AnimateRotation
    ));
}
