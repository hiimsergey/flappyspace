use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use flappyspace::*;

/// Custom game plugin for all things in the gameplay
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Game),
                (load_rock_timer, spawn_scoreboard_and_rock)
            )
            .add_systems(
                Update,
                (
                    animate_ship,
                    unhide_scoreboard,
                    periodic_waves,
                    move_rocks,
                    update_ship,
                    check_collisions
                ).run_if(in_state(GameState::Game))
            );
    }
}

/// Loads RockTimer with a random duration defined by ROCK_SPAWN_RATE
fn load_rock_timer(mut commands: Commands) {
    commands.insert_resource(RockTimer(Timer::from_seconds(
        fastrand::u8(ROCK_SPAWN_RATE) as f32 * 0.25,
        TimerMode::Once
    )));
}

/// Spawns invisible scoreboard on screen, gets turned visible later
/// 
/// See also: [`periodic_waves`]
fn spawn_scoreboard_and_rock(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.spawn((
        text_from_str(
            &assets, "-2", HEADING_FONT_SIZE, Color::BLACK, HEADING_Y
        ), Scoreboard { score: -2 }
    ));

    spawn_rock(commands, assets, BOTTOM_BOUND);
}

/// Makes score visible by turning the color white
///
/// Since it takes some time for the rock waves to come closer, I see no
/// meaning in starting to count the score instanty. Thus, the scoreboard
/// gets initialised with the value -2.
///
/// See also: [`spawn_scoreboard_and_rock`]
fn unhide_scoreboard(
    mut score_query: Query<&mut Scoreboard>,
    mut score_text_query: Query<&mut Text, With<Scoreboard>>
) {
    if score_query.single_mut().score == 0 {
        score_text_query.single_mut().sections[0].style.color = Color::WHITE;
    }
}

/// Tracks RockTimer, increments score and launches new wave
/// 
/// See also: [`spawn_scoreboard_and_rock`]
fn periodic_waves(
    mut commands: Commands,
    mut score_query: Query<&mut Scoreboard>,
    mut score_text_query: Query<&mut Text, With<Scoreboard>>,
    mut timer: ResMut<RockTimer>,
    assets: Res<AssetServer>,
    time: Res<Time>
) {
    let mut scoreboard = score_query.single_mut();
    let mut score_text = score_text_query.single_mut();

    // When RockTimer finishes, reloads it and increments the score
    if timer.tick(time.delta()).finished() {
        commands.insert_resource(RockTimer(Timer::from_seconds(
            fastrand::u8(ROCK_SPAWN_RATE) as f32 * 0.25,
            TimerMode::Once
        )));

        scoreboard.score += 1;
        score_text.sections[0].value = scoreboard.score.to_string();

        // Spawns new wave, starting with the first rock
        spawn_rock(commands, assets, BOTTOM_BOUND);
    }
}

/// Moves every present Rock using ROCK_VELOCITY, despawns it if ROCK_DESPAWN_X
/// coordinate is crossed
fn move_rocks(
    mut commands: Commands,
    mut rock_query: Query<(Entity, &mut Transform), With<Rock>>,
    time: Res<Time>
) {
    for (rock_entity, mut transform) in rock_query.iter_mut() {
        transform.translation.x -= ROCK_VELOCITY * time.delta_seconds();

        if transform.translation.x < ROCK_DESPAWN_X {
            commands.entity(rock_entity).despawn();
        }
    }
}

/// Controls the movement, rotation and gravity of the ship
/// 
/// Actually, there is no gravity in space but let's imagine...
fn update_ship(
    mut commands: Commands,
    mut ship_query: Query<(&mut Transform, &mut Ship)>,
    assets: Res<AssetServer>,
    key: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    for (mut transform, mut ship) in ship_query.iter_mut() {
        // Oooh, gravity
        transform.translation.y += ship.velocity * time.delta_seconds();
        ship.velocity -= GRAVITY;

        // Rotates ship according to velocity
        // The factor 0.0005 is chosen randomly but looks convenient
        transform.rotation = Quat::from_rotation_z(0.0005 * ship.velocity);

        // Checks for input (Space) and applies increased velocity
        if key.just_pressed(KeyCode::Space) {
            play_sound(&mut commands, &assets, "jump");
            ship.velocity = JUMP_VELOCITY;
        }

        // Prevents the ship from crossing bounds in the field
        if transform.translation.y < BOTTOM_BOUND {
            transform.translation.y = BOTTOM_BOUND;
        }
        if transform.translation.y > TOP_BOUND {
            transform.translation.y = TOP_BOUND;
        }
    }
}

/// Repeatedly checks for collisions between Ship and Rock entites
/// 
/// If collision occurs, the ship sprite is changed and the Game Over screen
/// appears.
fn check_collisions(
    mut commands: Commands,
    mut ship_query: Query<(&mut TextureAtlasSprite, &Transform), With<Ship>>,
    mut game_state: ResMut<NextState<GameState>>,
    assets: Res<AssetServer>,
    rock_query: Query<&Transform, With<Rock>>
) {
    let (mut sprite, ship_transform) = ship_query.single_mut();

    for transform in &rock_query {
        // Collision check
        if collide(
            // Position of rock
            transform.translation,
            // Size of rock (scale * height/width of sprite in pixels - 0.5)
            transform.scale.truncate() * 11.5,
            // Position of ship
            ship_transform.translation,
            // Size of ship
            Vec2::new(
                // Scale * (height in pixels - 0.5)
                transform.scale.truncate().x * 9.5,
                // scale * (width in pixels - 0.5)
                transform.scale.truncate().y * 11.5
            )
        ).is_some() {
            // Changes ship sprite to broken ship
            sprite.index = 0;
            play_sound(&mut commands, &assets, "crash");
            game_state.set(GameState::Crashed);
        }
    }
}
