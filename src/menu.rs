use bevy::prelude::*;
use super::{
	JUMP_VELOCITY,
	GameState,
	Ship,
	cleanup
};

pub struct MenuPlugin;

// Tag component for entities added on the menu screen
#[derive(Component)]
struct OnMenuScreen;

// TODO move to lib.rs
// Tag component for rotating text
#[derive(Component)]
pub struct AnimateRotation;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(GameState::Menu), (menu_setup, crate::spawn_ship))
			.add_systems(Update, (crate::animate_ship, menu_action).run_if(in_state(GameState::Menu)))
			// TODO move to main.rs
			.add_systems(FixedUpdate, rotate_text)
			.add_systems(OnExit(GameState::Menu), cleanup::<OnMenuScreen>);
	}
}

fn menu_setup(mut commands: Commands, assets: Res<AssetServer>) {
	commands.spawn((
		text_from_str(&assets, "Flappy Space", Color::WHITE, TextSize::Large),
		OnMenuScreen
	));
	commands.spawn((
		text_from_str(&assets, "Press Enter to start", Color::WHITE, TextSize::Low),
		OnMenuScreen, AnimateRotation
	));
	commands.spawn((
		text_from_str(&assets, "Press A for About", Color::WHITE, TextSize::Lower),
		OnMenuScreen, AnimateRotation
	));
}

// Move all util functions to lib.rs
pub enum TextSize { Large, Low, Lower, GameOver, Normal }
pub fn text_from_str(
	assets: &Res<AssetServer>,
	text: &str,
	text_color: Color,
	text_size: TextSize
) -> Text2dBundle {
	let text_style = TextStyle {
		font: assets.load("fonts/PixelifySans-SemiBold.ttf"),
		font_size: 50.,
		color: text_color
	};

	let (text_style, text_y) = match text_size {
		TextSize::Large => {
			(TextStyle { font_size: 100., ..text_style.clone() }, 200.)
		},
		TextSize::Low => (text_style.clone(), -170.),
		// TODO TEMP
		TextSize::Lower => (text_style.clone(), -270.),
		TextSize::Normal => {
			(TextStyle { font_size: 30., ..text_style.clone() }, 0.)
		},
		TextSize::GameOver => {
			(TextStyle { font_size: 100., ..text_style.clone() }, 0.)
		}
	};

	Text2dBundle {
		text: Text::from_section(text, text_style)
			.with_alignment(TextAlignment::Center),
		transform: Transform::from_xyz(0., text_y, 1.),
		..default()
	}
}

// Also works as Update system for Game Over
pub fn menu_action(
	mut commands: Commands,
    mut ship_query: Query<(&mut TextureAtlasSprite, &mut Ship)>,
	mut game_state: ResMut<NextState<GameState>>,
	assets: Res<AssetServer>,
	key: Res<Input<KeyCode>>,
) {	
	let (mut sprite, mut ship) = ship_query.single_mut();
	
	// Check for user input to enter game
	if key.just_pressed(KeyCode::Return) {
		play_sound(&mut commands, &assets, "start");
		sprite.index = 1;
        ship.velocity = JUMP_VELOCITY;
		game_state.set(GameState::Game);
	}
	if key.just_pressed(KeyCode::A) {
		game_state.set(GameState::About);
	}
}

pub fn rotate_text(
	mut query: Query<&mut Transform, With<AnimateRotation>>,
	time: Res<Time>
) {
	for mut transform in &mut query {
		transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos()) / 2.;
	}
}

pub fn play_sound(commands: &mut Commands, assets: &Res<AssetServer>, sound: &str) {
	commands.spawn(
		AudioBundle {
			source: assets.load(format!("sounds/{sound}.ogg")),
			settings: PlaybackSettings::DESPAWN
		}
	);
}
