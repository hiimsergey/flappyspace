//! A small 2D-game built with [Bevy](https://bevyengine.org)
//! - [GitHub](https://github.com/hiimsergey/flappyspace)

use bevy::prelude::*;
use bevy::sprite::TextureAtlasSprite;

// Import constants
mod consts;
pub use consts::*;

/// States of the game
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    /// Main menu, first/default game state
    #[default]
    Menu,
    /// About page, info and credits
    About,
    /// In-game screen
    Game,
    /// Game Over screen
    Crashed
}

/// Player component with variable velocity
#[derive(Component)]
pub struct Ship {
    pub velocity: f32
}

/// Obstacle component, tag only
#[derive(Component)]
pub struct Rock;

/// Tag component for rotating text
#[derive(Component)]
pub struct TextRotation;

/// Component for tracking the game score
#[derive(Component)]
pub struct Scoreboard {
    pub score: i32
}

/// Resource for storing the highscore
#[derive(Resource)]
pub struct Highscore(pub i32);

/// Timer for animating the Ship sprite
#[derive(Component, Deref, DerefMut)]
pub struct ShipAnimationTimer(pub Timer);

/// Timer resource for periodic rock spawn waves
#[derive(Resource, Deref, DerefMut)]
pub struct RockTimer(pub Timer);

/// Changes the Ship entity's texture periodically
pub fn animate_ship(
    mut query: Query<(&mut ShipAnimationTimer, &mut TextureAtlasSprite)>,
    time: Res<Time>
) {
    for (mut timer, mut sprite) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            match sprite.index {
                5 => sprite.index = 1,
                _ => sprite.index += 1
            }
        }
    }
}

/// Despawns all entities of a component
pub fn cleanup<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Checks for user input, either launches game or about screen
/// 
/// Is applied in menu and game over screen
pub fn lobby_input(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    assets: Res<AssetServer>,
    key: Res<Input<KeyCode>>
) {
    // If user presses X, launches game
    if key.just_pressed(KeyCode::X) {
        play_sound(&mut commands, &assets, "start");
        game_state.set(GameState::Game);
    }

    // If user presses 'a', launches about screen
    if key.just_pressed(KeyCode::A) {
        game_state.set(GameState::About);
    }
}

/// Plays sound at ./assets/sounds/`sound`.ogg by spawning AudioBundle
/// 
/// PlaybackSettings set to DESPAWN
pub fn play_sound(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    sound: &str
) {
    commands.spawn(
        AudioBundle {
            source: assets.load(format!("sounds/{sound}.ogg")),
            settings: PlaybackSettings::DESPAWN
        }
    );
}

/// Rotates every text field entity of the TextRotation component
pub fn rotate_text(
    mut text_query: Query<&mut Transform, With<TextRotation>>,
    time: Res<Time>
) {
    for mut transform in text_query.iter_mut() {
        transform.rotation =
            Quat::from_rotation_z(time.elapsed_seconds().cos()) / 2.;
    }
}

/// Spawns one rock in the game field
/// 
/// Recursively calls itself with y being higher by a random number
pub fn spawn_rock(
    mut commands: Commands,
    assets: Res<AssetServer>,
    y: f32
) {
    // Base case
    if y > TOP_BOUND + 64. { return }

    // Spawns rock
    commands.spawn((
        SpriteBundle {
            texture: assets.load(
                format!("sprites/rock{}.png", fastrand::u8(1..=4))
            ),
            transform: Transform::from_xyz(
                fastrand::u16(ROCK_SPAWN_X_RANGE) as f32,
                y,
                fastrand::f32()
            ).with_scale(Vec3::splat(fastrand::u8(ROCK_SIZE_RANGE) as f32))
                .with_rotation(Quat::from_rotation_z(fastrand::f32() * 10.)),
            ..default()
        },
        Rock
    ));

    let y_distance = fastrand::u8(ROCK_DISTANCE_RANGE) as f32;

    spawn_rock(commands, assets, y + y_distance);
}

/// Spawns the only Ship entity in the game at the center of the screen
/// 
/// The ship will be shown in the menu screen and in the gameplay.
pub fn spawn_ship(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<AssetServer>
) {
    // Loads the sprite sheet
    let texture_atlas = TextureAtlas::from_grid(
        assets.load("sprites/ship.png"),
        Vec2::new(12., 10.),
        6, 1, None, None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawns the entity
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            // Sets the spawn coordinates
            // z = 1.1 lets the ship appear before the rocks and score
            transform: Transform::from_xyz(0., 0., 1.1)
                .with_scale(Vec3::splat(3.)),
            ..default()
        },
        ShipAnimationTimer(Timer::from_seconds(0.4, TimerMode::Repeating)),
        Ship { velocity: JUMP_VELOCITY }
    ));
}

/// Returns a Text2dBundle to be spawned later
pub fn text_from_str(
    assets: &Res<AssetServer>,
    text: &str,
    font_size: f32,
    text_color: Color,
    text_y: f32
) -> Text2dBundle {
    let text_style = TextStyle {
        font: assets.load("fonts/PixelifySans-SemiBold.ttf"),
        font_size,
        color: text_color
    };

    Text2dBundle {
        text: Text::from_section(text, text_style)
            .with_alignment(TextAlignment::Center),
        transform: Transform::from_xyz(0., text_y, 1.),
        ..default()
    }
}
