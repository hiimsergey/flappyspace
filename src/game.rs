// TODO
// NOW add collision detection
// NOW add score
// BONUS add despawn animation
	// import Ship
// LATER increase avg distance and decrease ROCK_SPAWN_PERIOD
use super::{
	GameState,
	Rock,
	Ship
};
use std::ops::Range;
use bevy::{
	prelude::*,
	sprite::collide_aabb::collide
};
use fastrand;
use crate::menu;

// TODO END comment them
// TODO structure them
pub const JUMP_VELOCITY: f32 = 500.;
const GRAVITY: f32 = 40.;
const ROCK_VELOCITY: f32 = 200.;
const TOP_BOUND: f32 = 425.;
const BOTTOM_BOUND: f32 = -TOP_BOUND;
const ROCK_DESPAWN_X: f32 = -500.;
const ROCK_SPAWN_RANGE: Range<u16> = -ROCK_DESPAWN_X as u16..(-ROCK_DESPAWN_X + 150.) as u16;
const ROCK_SPAWN_RATE: Range<u8> = 4..6;
const ROCK_SIZE_RANGE: Range<u8> = 2..6;
const ROCK_DISTANCE_RANGE: Range<u8> = 128..255;

pub struct GamePlugin;

// Tag component marking the lowest Rock in a column
#[derive(Component)]
struct BottomRock;

// TODO make to resource
#[derive(Component)]
pub struct Scoreboard {
	score: i32
}

// Timer resource for periodically spawning rocks
#[derive(Resource, Deref, DerefMut)]
struct RockTimer(Timer);

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(RockTimer(Timer::from_seconds(fastrand::u8(ROCK_SPAWN_RATE) as f32 * 0.25, TimerMode::Once)))
			.add_systems(OnEnter(GameState::Game), (spawn_rocks, spawn_scoreboard))
			.add_systems(
				Update, (
					periodic_rock_waves,
					move_rocks,
					update_ship,
					check_collisions
					// TODO RM
					// check_collisions
					// TODO MAYBE
					// scoreboard_system
					// rotate_rocks
				).chain()
				.run_if(in_state(GameState::Game))
			);
	}
}

fn spawn_rocks(
	mut commands: Commands,
	assets: Res<AssetServer>
) {
	// TODO PLAN
	// perpetual motion of the rocks towards -x
	// if they reach LEFT_BOUND, despawn and spawn new wave
	// wave consists of random number of randomly sized rocks
	// BONUS from score XY and up, the waves move along y
	// TODO remove BottomRock
	// integrate this call into spawn_one_rock
	// rethink entire algorithm, make it recursive
	commands.spawn((
		rock_from_y(
			&assets,
			BOTTOM_BOUND + fastrand::u8(0..100) as f32
		),
		// TODO MAYBE put this in a constant or create a default impl
		Rock { velocity: ROCK_VELOCITY }, BottomRock
	));
	spawn_one_rock(
		&mut commands,
		&assets,
		BOTTOM_BOUND
	);
}

fn spawn_scoreboard(
	mut commands: Commands,
	assets: Res<AssetServer>
) {
	commands.spawn((
		menu::text_from_str(&assets, "-2", Color::BLACK, menu::TextSize::Large),
		Scoreboard { score: -2 }
	));
}

// Helper function to recursively spawn multiple rocks in a column with
// variable distances and radii
fn spawn_one_rock(
	mut commands: &mut Commands,
	assets: &Res<AssetServer>,
	y_point: f32
) {
	if y_point > TOP_BOUND { return; }

	let y_distance = fastrand::u8(ROCK_DISTANCE_RANGE) as f32;

	commands.spawn((
		rock_from_y(
			&assets,
			y_point + y_distance
		),
		// TODO rock.velocity
		Rock { velocity: ROCK_VELOCITY }
	));

	// Recursive call to spawn another rock
	spawn_one_rock(
		&mut commands,
		&assets,
		y_point + y_distance
	);
}

// Helper function because I'm lazy
fn rock_from_y(
	assets: &Res<AssetServer>,
	y: f32
) -> SpriteBundle {
	SpriteBundle {
		texture: assets.load(format!("sprites/rock{}.png", fastrand::u8(1..=4))),
		transform: Transform::from_xyz(fastrand::u16(ROCK_SPAWN_RANGE) as f32, y, fastrand::f32())
			.with_scale(Vec3::splat(fastrand::u8(ROCK_SIZE_RANGE) as f32))
			// TODO TEST
			.with_rotation(Quat::from_rotation_z(fastrand::f32() * 10.)),
		..default()
	}
}

fn periodic_rock_waves(
	mut commands: Commands,
	mut query: Query<&mut Scoreboard>,
	mut text_query: Query<&mut Text, With<Scoreboard>>,
	mut timer: ResMut<RockTimer>,
	assets: Res<AssetServer>,
	time: Res<Time>
) {
	if query.single_mut().score == 0 {
		// rewriting textstyle
		// this is kinda sloppy, so mind this
		text_query.single_mut().sections[0].style = TextStyle {
			font: assets.load("fonts/PixelifySans-SemiBold.ttf"),
			font_size: 100.,
			color: Color::WHITE
		};
	}

	if timer.tick(time.delta()).finished() {
		commands.insert_resource(RockTimer(Timer::from_seconds(
			fastrand::u8(ROCK_SPAWN_RATE) as f32 * 0.25,
			TimerMode::Once
		)));

		// Increment score
		query.single_mut().score += 1;
		text_query.single_mut().sections[0].value = query.single().score.to_string();

		spawn_rocks(commands, assets);
	}
}

fn move_rocks(
	mut commands: Commands,
	mut query: Query<(Entity, &mut Transform, &mut Rock)>,
	time: Res<Time>
) {
	for (rock_entity, mut transform, rock) in query.iter_mut() {
		transform.translation.x -= rock.velocity * time.delta_seconds();

		if transform.translation.x < ROCK_DESPAWN_X {
			commands.entity(rock_entity).despawn();
			// TODO NOW spawn_rocks
		}
	}
}

// Actually, there is no gravity in space but let's imagine...
fn update_ship(
	mut commands: Commands,
	mut query: Query<(&mut Transform, &mut Ship)>,
	assets: Res<AssetServer>,
	key: Res<Input<KeyCode>>,
	time: Res<Time>
) {
	// TODO BONUS if the ship.x is beyond the window bound, then just die
	for (mut transform, mut ship) in query.iter_mut() {
		// Gravity, oooh
		transform.translation.y += ship.velocity * time.delta_seconds();
		ship.velocity -= GRAVITY;

		// Rotate ship according to velocity
		transform.rotation = Quat::from_rotation_z(0.0005 * ship.velocity);

		// Apply increased velocity after keyboard event
		if key.just_pressed(KeyCode::Space) {
			menu::play_sound(&mut commands, &assets, "jump");
			ship.velocity = JUMP_VELOCITY;
		}

		// Add space bounds
		if transform.translation.y < BOTTOM_BOUND {
			transform.translation.y = BOTTOM_BOUND;
		}
		if transform.translation.y > TOP_BOUND {
			transform.translation.y = TOP_BOUND;
		}
	}
}

fn check_collisions(
	mut commands: Commands,
	mut ship_query: Query<(&mut TextureAtlasSprite, &Transform), With<Ship>>,
	mut game_state: ResMut<NextState<GameState>>,
	assets: Res<AssetServer>,
	rock_query: Query<&Transform, With<Rock>>
) {
	let (mut sprite, ship_transform) = ship_query.single_mut();

	for transform in &rock_query {
		if collide(
			// position of rock
			transform.translation,
			// size of rock (scale * height/width of sprite in pixels -0.5)
			transform.scale.truncate() * 11.5,
			// position of ship
			ship_transform.translation,
			// size of ship
			Vec2::new(
				// scale * (width in pixels - 0.5)
				transform.scale.truncate().x * 11.5,
				// scale * (height in pixels - 0.5)
				transform.scale.truncate().y * 9.5
			)
		).is_some() {
			sprite.index = 0;
			menu::play_sound(&mut commands, &assets, "crash");
			game_state.set(GameState::Dead);
		}
	}
}
