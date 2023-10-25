// TODO
// Update system: run only if state is GameState::Game
// BONUS add despawn animation
	// import Ship
use super::{
	GameState,
	Rock,
	Ship,
	despawn_screen
};
use bevy::prelude::*;
use fastrand;

// TODO END comment them
const GRAVITY: f32 = 40.;
const JUMP_VELOCITY: f32 = 500.;
const ROCK_VELOCITY: f32 = 200.;
const UPPER_BOUND: f32 = 400.;
const BOTTOM_BOUND: f32 = -UPPER_BOUND;
const RIGHT_BOUND: f32 = 500.;
const LEFT_BOUND: f32 = -RIGHT_BOUND;
const ROCK_SPAWN_PERIOD: f32 = 1.;

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
			.insert_resource(RockTimer(Timer::from_seconds(ROCK_SPAWN_PERIOD, TimerMode::Repeating)))
			.add_systems(OnEnter(GameState::Game), (spawn_rocks, lil_jump))
			.add_systems(
				Update,
				(
					update_ship,
					move_rocks,
					periodic_rock_waves
					// check_collisions
					// TODO MAYBE
					// scoreboard_system
					// rotate_rocks
				).run_if(in_state(GameState::Game))
			);
	}
}

// The ship performs a little jump at the start of the game
fn lil_jump(
	mut query: Query<&mut Ship>,
) {
	for mut ship in query.iter_mut() {
		ship.velocity = JUMP_VELOCITY;
		println!("{}", ship.velocity);
	}
}

// Helper function because I'm lazy
fn rock_from_y(y: f32, assets: &Res<AssetServer>) -> SpriteBundle {
	SpriteBundle {
		texture: assets.load("sprites/rock.png"),
		transform: Transform::from_xyz(RIGHT_BOUND, y, 0.)
			.with_scale(Vec3::splat(fastrand::u8(2..7) as f32)),
		..default()
	}
}

fn spawn_rocks(
 mut commands: Commands,
 assets: Res<AssetServer>
) {
	// TODO PLAN
	// perpetual motion of the stones towards -x
	// if they reach LEFT_BOUND, despawn and spawn new wave
	// wave consists of random number of randomly sized rocks
	// BONUS from score XY and up, the waves move along y
	commands.spawn((
		rock_from_y(BOTTOM_BOUND, &assets),
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

	let y_distance = fastrand::u8(150..250) as f32;

	commands.spawn((
		rock_from_y(y_point + y_distance, &assets),
		// TODO rock.velocity
		Rock { velocity: ROCK_VELOCITY }
	));

	// Recursive call to spawn another rock
	spawn_one_rock(y_point + y_distance, commands, assets);
}

fn periodic_rock_waves(
	mut commands: Commands,
	mut timer: ResMut<RockTimer>,
	assets: Res<AssetServer>,
	time: Res<Time>
) {
	if timer.tick(time.delta()).finished() {
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
					..default()
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
