use bevy::prelude::*;
use super::{
	JUMP_VELOCITY,
	GameState,
	Ship,
	despawn_screen
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
			.add_systems(OnEnter(GameState::Menu), menu_setup)
			.add_systems(Update, menu_action.run_if(in_state(GameState::Menu)))
			.add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>);
	}
}

fn menu_setup(mut commands: Commands, assets: Res<AssetServer>) {
	commands.spawn((
		text_from_str(&assets, "Flappy Space", Color::WHITE, TextSize::Large),
		OnMenuScreen
	));
	commands.spawn((
		text_from_str(&assets, "Press Enter to start", Color::WHITE, TextSize::Normal),
		OnMenuScreen, AnimateRotation
	));
}

// Move all util functions to lib.rs
pub enum TextSize { Normal, Large }
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
		TextSize::Normal => (text_style.clone(), -200.),
		TextSize::Large => {
			(TextStyle { font_size: 100., ..text_style.clone() }, 200.)
		}
	};

	Text2dBundle {
		text: Text::from_section(text, text_style)
			.with_alignment(TextAlignment::Center),
		transform: Transform::from_xyz(0., text_y, 1.),
		..default()
	}
}

pub fn menu_action(
	mut commands: Commands,
	mut query: Query<&mut Transform, With<AnimateRotation>>,
    mut ship_query: Query<(&mut TextureAtlasSprite, &mut Ship)>,
	mut game_state: ResMut<NextState<GameState>>,
	assets: Res<AssetServer>,
	key: Res<Input<KeyCode>>,
	time: Res<Time>,
) {	
	let (mut sprite, mut ship) = ship_query.single_mut();
	
	// Rotate bottom text
	for mut transform in &mut query {
		transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos()) / 2.;
	}
	
	// Check for user input to enter game
	if key.just_pressed(KeyCode::Return) {
		play_sound(&mut commands, &assets, "start");
		sprite.index = 1;
        ship.velocity = JUMP_VELOCITY;
		game_state.set(GameState::Game);
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
