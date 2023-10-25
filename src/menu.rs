use bevy::prelude::*;
use super::{
	despawn_screen,
	GameState
};

pub struct MenuPlugin;

// Tag component for entities added on the menu screen
#[derive(Component)]
struct OnMenuScreen;

// Tag component for rotating text
#[derive(Component)]
struct AnimateRotation;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(GameState::Menu), menu_setup)
			.add_systems(Update, menu_action.run_if(in_state(GameState::Menu)))
			.add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>);
	}
}

fn menu_setup(mut commands: Commands, assets: Res<AssetServer>) {
	let text_style = TextStyle {
		font: assets.load("fonts/PixelifySans-SemiBold.ttf"),
		font_size: 50.,
		color: Color::WHITE
	};
	let text_alignment = TextAlignment::Center;

	commands.spawn((
		Text2dBundle {
			text: Text::from_section("Flappy Space", TextStyle {
				font_size: 100.,
				..text_style.clone()
			}).with_alignment(text_alignment),
			transform: Transform::from_translation(200. * Vec3::Y),
			..default()
		},
		OnMenuScreen
	));
	commands.spawn((
		Text2dBundle {
			text: Text::from_section("Press Space to start", text_style)
				.with_alignment(text_alignment),
			transform: Transform::from_translation(-200. * Vec3::Y),
			..default()
		},
		OnMenuScreen, AnimateRotation
	));
}

// TODO DOING
fn menu_action(
	mut commands: Commands,
	mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
	mut game_state: ResMut<NextState<GameState>>,
	assets: Res<AssetServer>,
	key: Res<Input<KeyCode>>,
	time: Res<Time>,
) {	
	// Rotate bottom text
	for mut transform in &mut query {
		transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos()) / 2.;
	}
	
	// Check for user input to enter game
	if key.just_pressed(KeyCode::Space) {
		// Play sound :)
		commands.spawn(
			AudioBundle {
				source: assets.load("sounds/start.wav"),
				..default()
			}
		);
		game_state.set(GameState::Game);
	}
}