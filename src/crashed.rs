use bevy::prelude::*;
use flappyspace::*;

/// Custom game plugin for all things on the Game Over screen
pub struct CrashedPlugin;

/// Tag component for entites added on the Game Over screen
#[derive(Component)]
struct OnCrashedScreen;

impl Plugin for CrashedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Crashed),
                (spawn_crashed_text, spawn_highscore)
            )
            .add_systems(
                Update,
                (lobby_input, rotate_text).run_if(in_state(GameState::Crashed))
            )
            .add_systems(OnExit(GameState::Crashed), (
                cleanup::<Rock>,
                cleanup::<Scoreboard>,
                cleanup::<OnCrashedScreen>
            ));
    }
}

/// Spawns text on Game Over screen: heading, two input hints
fn spawn_crashed_text(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        text_from_str(
            &assets,
            "Game Over",
            HEADING_FONT_SIZE,
            Color::RED,
            0.
        ), OnCrashedScreen
    ));
    commands.spawn((
        text_from_str(
            &assets,
            "Press Enter to restart",
            INPUT_HINT_FONT_SIZE,
            Color::RED,
            INPUT_HINT_UPPER_Y
        ), OnCrashedScreen, TextRotation
    ));
    commands.spawn((
        text_from_str(
            &assets,
            "Press A for About",
            INPUT_HINT_FONT_SIZE,
            Color::RED,
            INPUT_HINT_LOWER_Y
        ), OnCrashedScreen, TextRotation
    ));
}

/// Spawn text informing about the highscore: Either "New highscore" or "Highscore: xy"
fn spawn_highscore(
    mut commands: Commands,
    mut highscore_query: ResMut<Highscore>,
    assets: Res<AssetServer>,
    score_query: Query<&mut Scoreboard>
) {
    commands.spawn((
        if score_query.single().score > highscore_query.0 {
            highscore_query.0 = score_query.single().score;
            text_from_str(
                &assets,
                "New highscore!",
                INPUT_HINT_FONT_SIZE,
                Color::TOMATO,
                HIGHSCORE_Y
            )
        } else {
            text_from_str(
                &assets,
                format!("Highscore: {}", highscore_query.0).as_str(),
                INPUT_HINT_FONT_SIZE,
                Color::RED,
                HIGHSCORE_Y
            )
        },
        OnCrashedScreen
    ));
}
