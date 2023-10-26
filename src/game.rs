// TODO
// NOW add collision detection
// NOW add score
// BONUS add despawn animation
	// import Ship
// LATER increase avg distance and decrease ROCK_SPAWN_PERIOD
use super::{
	GameState,
	Rock,
	Ship,
	despawn_screen
};
use std::ops::{Range, RangeBounds};
use bevy::{
	prelude::*,
	sprite::collide_aabb::{collide, Collision}
};
use fastrand;

// TODO END comment them
pub const JUMP_VELOCITY: f32 = 500.;
const GRAVITY: f32 = 40.;
const ROCK_VELOCITY: f32 = 200.;
const UPPER_BOUND: f32 = 400.;
const BOTTOM_BOUND: f32 = -UPPER_BOUND;
const LEFT_BOUND: f32 = -500.;
const RIGHT_BOUND_RANGE: Range<u16> = -LEFT_BOUND as u16..(-LEFT_BOUND + 150.) as u16;
const ROCK_SPAWN_RANGE: Range<u8> = 4..6;

pub struct GamePlugin;

// Tag component marking the lowest Rock in a column
#[derive(Component)]
struct BottomRock;

// Timer resource for periodically spawning rocks
#[derive(Resource, Deref, DerefMut)]
struct RockTimer(Timer);

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(RockTimer(Timer::from_seconds(fastrand::u8(ROCK_SPAWN_RANGE) as f32 * 0.25, TimerMode::Once)))
			.add_systems(OnEnter(GameState::Game), spawn_rocks)
			.add_systems(
				Update,
				(
					periodic_rock_waves,
					move_rocks,
					update_ship,
					check_collisions
					// TODO RM
					// check_collisions
					// TODO MAYBE
					// scoreboard_system
					// rotate_rocks
				).run_if(in_state(GameState::Game))
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
	commands.spawn((
		rock_from_y(BOTTOM_BOUND + fastrand::u8(0..100) as f32, &assets),
		// TODO MAYBE put this in a constant or create a default impl
		Rock { velocity: ROCK_VELOCITY }, BottomRock
	));
	spawn_one_rock(BOTTOM_BOUND, commands, assets);
}

// Helper function to recursively spawn multiple rocks in a column with
// variable distances and radii
fn spawn_one_rock(
	y_point: f32,
	mut commands: Commands,
	assets: Res<AssetServer>
) {
	if y_point > UPPER_BOUND { return; }

	let y_distance = fastrand::u8(128..) as f32;

	commands.spawn((
		rock_from_y(y_point + y_distance, &assets),
		// TODO rock.velocity
		Rock { velocity: ROCK_VELOCITY }
	));

	// Recursive call to spawn another rock
	spawn_one_rock(y_point + y_distance, commands, assets);
}

// Helper function because I'm lazy
fn rock_from_y(y: f32, assets: &Res<AssetServer>) -> SpriteBundle {
	SpriteBundle {
		texture: assets.load("sprites/rock.png"),
		transform: Transform::from_xyz(fastrand::u16(RIGHT_BOUND_RANGE) as f32, y, fastrand::f32())
			.with_scale(Vec3::splat(fastrand::u8(2..7) as f32))
			// TODO TEST
			.with_rotation(Quat::from_rotation_z(fastrand::f32() * 10.)),
		..default()
	}
}

fn periodic_rock_waves(
	mut commands: Commands,
	mut timer: ResMut<RockTimer>,
	assets: Res<AssetServer>,
	time: Res<Time>
) {
	if timer.tick(time.delta()).finished() {
		commands.insert_resource(RockTimer(Timer::from_seconds(
			fastrand::u8(ROCK_SPAWN_RANGE) as f32 * 0.25,
			TimerMode::Once
		)));
		spawn_rocks(commands, assets);
	}
}

fn move_rocks(
	mut commands: Commands,
	mut query: Query<(Entity, &mut Transform, &mut Rock)>,
	assets: Res<AssetServer>,
	time: Res<Time>
) {
	for (rock_entity, mut transform, rock) in query.iter_mut() {
		transform.translation.x -= rock.velocity * time.delta_seconds();

		if transform.translation.x < LEFT_BOUND {
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

		// Apply increased velocity after keyboard event
		if key.just_pressed(KeyCode::Space) {
			// Play sound :)
			commands.spawn(
				AudioBundle {
					source: assets.load("sounds/jump.wav"),
					settings: PlaybackSettings::DESPAWN
				}
			);

			ship.velocity = JUMP_VELOCITY;
		}

		// Add space bounds
		if transform.translation.y < BOTTOM_BOUND {
			transform.translation.y = BOTTOM_BOUND;
		}
		if transform.translation.y > UPPER_BOUND {
			transform.translation.y = UPPER_BOUND;
		}
	}
}

use std::io::Write; // TODO
fn check_collisions(
	mut commands: Commands,
	mut ship_query: Query<&Transform, With<Ship>>,
	mut game_state: ResMut<NextState<GameState>>,
	rock_query: Query<&Transform, With<Rock>>
) {
	let ship_transform = ship_query.single_mut();

	for transform in &rock_query {
		if let Some(collision) = collide(
			// position of rock
			transform.translation,
			// size of rock (scale * height/width of sprite in pixels -0.5)
			transform.scale.truncate() * 11.5,
			// position of ship
			ship_transform.translation,
			// size of ship (scale * height/width of sprite in pixels -0.5)
			ship_transform.scale.truncate() * 19.5
		) {
			println!("
					\n{}\n{}\n{}\n{}
				",
				transform.translation,
				// size of rock
				transform.scale.truncate(),
				// position of ship
				ship_transform.translation,
				// size of ship
				ship_transform.scale.truncate()
			);
			game_state.set(GameState::Dead);
		}
	}
}

// TODO LATER
/*
fn check_collisions(
	mut commands: Commands,
	mut ship_query: Query<&Transform, With<Ship>>,
	rock_query: Query<&Transform, With<Rock>>
) {
	let ship_transform = &ship_query.single_mut();
	for transform in &rock_query {
		if transform.translation.x < 0.5
		&& 
	}
}
*/
